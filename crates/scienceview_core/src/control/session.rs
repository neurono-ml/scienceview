//! The session: one reducer, one optional timeline and one subscriber registry.

use std::sync::Arc;

use crate::sv_animation::{SVAnimation, SVPlaybackState};

use super::{
    SVControlAction, SVControlChange, SVControlEvent, SVControlOrigin, SVControlState,
    SVControlSurface, SVPlaybackChange,
};

/// A handle to one registered subscriber.
///
/// Keep it to unsubscribe; a dropped handle does not unsubscribe by itself, so
/// the session keeps delivering until [`SVControlSession::unsubscribe`] is
/// called.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SVControlSubscription {
    id: u64,
}

impl SVControlSubscription {
    /// Returns the subscription identity.
    pub fn id(self) -> u64 {
        self.id
    }
}

type SVControlObserver = Arc<dyn Fn(&SVControlEvent) + Send + Sync>;

/// A running control session.
///
/// The session owns the reducer, the animation timeline the playback controls
/// drive and the observers. It is the object a backend or a host application
/// holds: dispatch one action, then every built-in and third-party subscriber
/// receives the same payloads in registration order.
pub struct SVControlSession {
    state: SVControlState,
    animation: Option<SVAnimation>,
    subscribers: Vec<(u64, SVControlObserver)>,
    next_subscription: u64,
    last_events: Vec<SVControlEvent>,
}

impl SVControlSession {
    /// Creates a session over a reducer state, without a timeline.
    pub fn new(state: SVControlState) -> Self {
        Self {
            state,
            animation: None,
            subscribers: Vec::new(),
            next_subscription: 0,
            last_events: Vec::new(),
        }
    }

    /// Attaches the animation timeline the playback controls drive.
    #[must_use]
    pub fn with_animation(mut self, animation: SVAnimation) -> Self {
        self.animation = Some(animation);
        self
    }

    /// Returns the reducer state.
    pub fn state(&self) -> &SVControlState {
        &self.state
    }

    /// Borrows the reducer state mutably, to register axes and bounds.
    pub fn state_mut(&mut self) -> &mut SVControlState {
        &mut self.state
    }

    /// Returns the attached animation timeline, when there is one.
    pub fn animation(&self) -> Option<&SVAnimation> {
        self.animation.as_ref()
    }

    /// Borrows the attached animation timeline mutably, to advance its clock.
    pub fn animation_mut(&mut self) -> Option<&mut SVAnimation> {
        self.animation.as_mut()
    }

    /// Registers an observer and returns its subscription handle.
    pub fn subscribe<F>(&mut self, observer: F) -> SVControlSubscription
    where
        F: Fn(&SVControlEvent) + Send + Sync + 'static,
    {
        self.next_subscription += 1;
        let subscription = SVControlSubscription {
            id: self.next_subscription,
        };
        self.subscribers.push((subscription.id, Arc::new(observer)));
        subscription
    }

    /// Removes an observer; returns whether it was registered.
    pub fn unsubscribe(&mut self, subscription: SVControlSubscription) -> bool {
        let before = self.subscribers.len();
        self.subscribers.retain(|(id, _)| *id != subscription.id);
        before != self.subscribers.len()
    }

    /// Returns the number of registered observers.
    pub fn subscriber_count(&self) -> usize {
        self.subscribers.len()
    }

    /// Applies one action with no widget attached.
    ///
    /// # Errors
    ///
    /// Propagates the dispatch errors of the reducer and of the timeline.
    pub fn dispatch(
        &mut self,
        action: &SVControlAction,
    ) -> Result<&[SVControlEvent], crate::sv_error::SVError> {
        self.dispatch_from(SVControlOrigin::Programmatic, action)
    }

    /// Activates a control or widget and applies its action.
    ///
    /// # Errors
    ///
    /// Propagates the activation and dispatch errors.
    pub fn dispatch_surface(
        &mut self,
        surface: &dyn SVControlSurface,
    ) -> Result<&[SVControlEvent], crate::sv_error::SVError> {
        let action = surface.activate()?;
        self.dispatch_from(surface.origin(), &action)
    }

    /// Applies one action attributed to an origin.
    ///
    /// # Errors
    ///
    /// Returns [`crate::sv_error::SVError::MissingAnimationTimeline`] when the
    /// action delegates playback and no timeline is attached, and propagates
    /// the reducer and timeline errors. A failing action leaves the state and
    /// the subscribers untouched.
    pub fn dispatch_from(
        &mut self,
        origin: SVControlOrigin,
        action: &SVControlAction,
    ) -> Result<&[SVControlEvent], crate::sv_error::SVError> {
        let report = self.state.dispatch_from(origin.clone(), action)?;
        let mut events = report.events().to_vec();

        if let Some(animation_action) = report.animation_action() {
            let change = {
                let animation = self
                    .animation
                    .as_mut()
                    .ok_or(crate::sv_error::SVError::MissingAnimationTimeline)?;
                let previous_state = animation.state();
                let previous_position_seconds = animation.position_seconds();
                let previous_speed = animation.speed();
                animation.dispatch(animation_action)?;
                SVPlaybackChange {
                    previous_state,
                    state: animation.state(),
                    previous_position_seconds,
                    position_seconds: animation.position_seconds(),
                    previous_speed,
                    speed: animation.speed(),
                }
            };
            events.push(SVControlEvent::new(
                origin,
                SVControlChange::Playback(change),
            ));
        }

        self.last_events = events;
        for (_, subscriber) in &self.subscribers {
            for event in &self.last_events {
                subscriber(event);
            }
        }
        Ok(&self.last_events)
    }

    /// Returns the events of the last dispatch.
    pub fn last_events(&self) -> &[SVControlEvent] {
        &self.last_events
    }

    /// Returns the playback state of the attached timeline, when there is one.
    pub fn playback_state(&self) -> Option<SVPlaybackState> {
        self.animation.as_ref().map(SVAnimation::state)
    }
}
