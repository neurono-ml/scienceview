//! Test-only allocation probe for the sparse geometry.
//!
//! The probe counts the bytes the current thread allocates while an operation
//! runs. It is thread-local and disabled outside the measured block, so the
//! other tests sharing this binary keep their normal allocator behaviour.

use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;

thread_local! {
    static MEASURING: Cell<bool> = const { Cell::new(false) };
    static ALLOCATED_BYTES: Cell<usize> = const { Cell::new(0) };
}

/// A `System`-backed allocator that records the current thread's allocations.
struct SVAllocationProbe;

unsafe impl GlobalAlloc for SVAllocationProbe {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let pointer = unsafe { System.alloc(layout) };
        if !pointer.is_null() {
            sv_record(layout.size());
        }
        pointer
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let pointer = unsafe { System.alloc_zeroed(layout) };
        if !pointer.is_null() {
            sv_record(layout.size());
        }
        pointer
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let resized = unsafe { System.realloc(pointer, layout, new_size) };
        if !resized.is_null() {
            sv_record(new_size);
        }
        resized
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        unsafe { System.dealloc(pointer, layout) }
    }
}

#[global_allocator]
static ALLOCATOR: SVAllocationProbe = SVAllocationProbe;

fn sv_record(bytes: usize) {
    let _ = MEASURING.try_with(|measuring| {
        if measuring.get() {
            let _ = ALLOCATED_BYTES.try_with(|allocated| {
                allocated.set(allocated.get().saturating_add(bytes));
            });
        }
    });
}

/// Runs `operation` and returns its value with the bytes it allocated.
pub(crate) fn sv_measure_allocations<Result>(
    operation: impl FnOnce() -> Result,
) -> (Result, usize) {
    let _ = ALLOCATED_BYTES.try_with(|allocated| allocated.set(0));
    let _ = MEASURING.try_with(|measuring| measuring.set(true));
    let result = operation();
    let _ = MEASURING.try_with(|measuring| measuring.set(false));
    let bytes = ALLOCATED_BYTES.try_with(Cell::get).unwrap_or_default();
    (result, bytes)
}
