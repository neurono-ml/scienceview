//! The present loop: a published frame reaches a presenter, a second frame
//! differs from the first, and a streaming update redraws in place.

use super::SVFrameChannel;
use super::SVPresentLoop;
use super::SVPresenter;
use crate::backend::SVNativeBackend;
use crate::canvas::SVNativeFrame;
use scienceview_core::sv_backend::SVBackendKind;
use scienceview_core::sv_backend::SVRenderBackend;
use scienceview_core::sv_backend::SVRenderRequest;
use scienceview_core::sv_backend::SVUpdateRequest;
use scienceview_core::sv_data::SVColumnView;
use scienceview_core::sv_error::SVError;
use scienceview_core::sv_figure::SVExecutionMode;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

/// A presenter that records what a window would have blitted, without a display.
#[derive(Default)]
struct SVRecordingPresenter {
    frames: Vec<SVNativeFrame>,
}

impl SVRecordingPresenter {
    fn presented_frames(&self) -> &[SVNativeFrame] {
        &self.frames
    }
}

impl SVPresenter for SVRecordingPresenter {
    fn present(&mut self, frame: &SVNativeFrame) -> Result<(), SVError> {
        self.frames.push(frame.clone());
        Ok(())
    }
}

fn sv_solid_frame(value: u8) -> SVNativeFrame {
    SVNativeFrame::new(2, 2, 1.0, vec![value; 2 * 2 * 4])
}

fn sv_request(values: &[f64]) -> SVRenderRequest<'_> {
    SVRenderRequest {
        backend_kind: SVBackendKind::Native,
        execution_mode: SVExecutionMode::Automatic,
        view: SVColumnView::Dense(values),
        title: None,
    }
}

#[test]
fn a_published_frame_wakes_the_window_and_is_presented() {
    let channel = SVFrameChannel::default();
    let publisher = channel.publisher();
    let mut presenter = SVRecordingPresenter::default();
    let wakeups = Arc::new(AtomicUsize::new(0));
    let counter = Arc::clone(&wakeups);
    channel.set_waker(Arc::new(move || {
        counter.fetch_add(1, Ordering::SeqCst);
    }));
    let mut present_loop = SVPresentLoop::new(channel);

    publisher.publish(sv_solid_frame(4));

    assert_eq!(wakeups.load(Ordering::SeqCst), 1);
    assert!(
        present_loop
            .present_pending(&mut presenter)
            .expect("the recording presenter cannot fail")
    );
    assert_eq!(presenter.presented_frames(), &[sv_solid_frame(4)]);
}

#[test]
fn presenting_nothing_does_not_call_the_presenter() {
    let channel = SVFrameChannel::default();
    let mut presenter = SVRecordingPresenter::default();
    let mut present_loop = SVPresentLoop::new(channel);

    assert!(
        !present_loop
            .present_pending(&mut presenter)
            .expect("the recording presenter cannot fail")
    );
    assert!(presenter.presented_frames().is_empty());
}

#[test]
fn a_second_published_frame_differs_and_is_presented() {
    let channel = SVFrameChannel::default();
    let publisher = channel.publisher();
    let mut presenter = SVRecordingPresenter::default();
    let mut present_loop = SVPresentLoop::new(channel);

    publisher.publish(sv_solid_frame(0));
    present_loop
        .present_pending(&mut presenter)
        .expect("the recording presenter cannot fail");
    publisher.publish(sv_solid_frame(255));
    present_loop
        .present_pending(&mut presenter)
        .expect("the recording presenter cannot fail");

    let frames = presenter.presented_frames();
    assert_eq!(frames.len(), 2);
    assert_ne!(
        frames[0].pixels(),
        frames[1].pixels(),
        "the second frame must not be a repeat of the first"
    );
}

#[test]
fn a_streaming_update_redraws_the_same_window_in_place() {
    let channel = SVFrameChannel::default();
    let publisher = channel.publisher();
    let mut presenter = SVRecordingPresenter::default();
    let mut present_loop = SVPresentLoop::new(channel);

    let initial_values = [1.0, 2.0, 3.0];
    let backend = SVNativeBackend::new();
    let initial = backend
        .render_frame(&sv_request(&initial_values))
        .expect("the initial frame renders");
    publisher.publish(initial);
    present_loop
        .present_pending(&mut presenter)
        .expect("the recording presenter cannot fail");

    let appended_values = [1.0, 2.0, 3.0, 8.0];
    let update = SVUpdateRequest {
        backend_kind: SVBackendKind::Native,
        execution_mode: SVExecutionMode::Automatic,
        view: SVColumnView::Dense(&appended_values),
        appended_points: 1,
    };
    let output = backend.update(&update).expect("the update renders");
    let updated = SVNativeFrame::new(640, 400, 1.0, output.bytes);
    publisher.publish(updated);
    present_loop
        .present_pending(&mut presenter)
        .expect("the recording presenter cannot fail");

    let frames = presenter.presented_frames();
    assert_eq!(frames.len(), 2, "the same window presented both frames");
    assert_eq!(
        (frames[1].width(), frames[1].height()),
        (frames[0].width(), frames[0].height()),
        "the update kept the window's dimensions"
    );
    assert_ne!(
        frames[0].pixels(),
        frames[1].pixels(),
        "new data changed the presented pixels"
    );
}
