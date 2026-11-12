//! The frame channel: one pending frame and the waker that announces it.

use crate::canvas::SVNativeFrame;
use std::sync::Arc;
use std::sync::Mutex;

/// The callback a consumer installs to be told that a frame was published.
type SVFrameWaker = Arc<dyn Fn() + Send + Sync>;

/// The one frame a producer published and the consumer has not presented yet.
#[derive(Default)]
struct SVFrameSlot {
    pending: Option<SVNativeFrame>,
    waker: Option<SVFrameWaker>,
}

/// The producer side of the window's frame channel.
///
/// A streaming figure clones the publisher, renders the new data into a frame
/// and publishes it; the running window presents the frame in place. Only the
/// newest frame is kept, so a producer faster than the window never queues
/// stale frames.
#[derive(Clone)]
pub struct SVFramePublisher {
    slot: Arc<Mutex<SVFrameSlot>>,
}

impl SVFramePublisher {
    /// Publishes a frame and wakes the window so it redraws.
    pub fn publish(&self, frame: SVNativeFrame) {
        let waker = match self.slot.lock() {
            Ok(mut slot) => {
                slot.pending = Some(frame);
                slot.waker.clone()
            }
            Err(_) => return,
        };
        if let Some(waker) = waker {
            waker();
        }
    }
}

/// The consumer side of the window's frame channel.
#[cfg(any(feature = "window", test))]
#[derive(Clone, Default)]
pub(crate) struct SVFrameChannel {
    slot: Arc<Mutex<SVFrameSlot>>,
}

#[cfg(any(feature = "window", test))]
impl SVFrameChannel {
    /// Returns a publisher that feeds this channel.
    pub(crate) fn publisher(&self) -> SVFramePublisher {
        SVFramePublisher {
            slot: Arc::clone(&self.slot),
        }
    }

    /// Takes the pending frame, if a producer published one.
    pub(crate) fn take(&self) -> Option<SVNativeFrame> {
        self.slot.lock().ok()?.pending.take()
    }

    /// Returns `true` when a frame is waiting to be presented.
    pub(crate) fn has_pending(&self) -> bool {
        self.slot
            .lock()
            .map(|slot| slot.pending.is_some())
            .unwrap_or(false)
    }

    /// Installs the callback that wakes the window after a publish.
    pub(crate) fn set_waker(&self, waker: SVFrameWaker) {
        if let Ok(mut slot) = self.slot.lock() {
            slot.waker = Some(waker);
        }
    }
}
