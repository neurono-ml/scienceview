//! Keeps the shared operation-span callsite interested across the test binary.

use std::sync::Once;
use tracing::{Event, Id, Metadata, Subscriber, span};

/// A subscriber that only answers "interested", never records.
struct SVInterestSubscriber;

impl Subscriber for SVInterestSubscriber {
    fn enabled(&self, _metadata: &Metadata<'_>) -> bool {
        true
    }

    fn new_span(&self, _attributes: &span::Attributes<'_>) -> Id {
        Id::from_u64(1)
    }

    fn record(&self, _identifier: &Id, _values: &span::Record<'_>) {}

    fn record_follows_from(&self, _identifier: &Id, _follows: &Id) {}

    fn event(&self, _event: &Event<'_>) {}

    fn enter(&self, _identifier: &Id) {}

    fn exit(&self, _identifier: &Id) {}
}

/// Installs the interest subscriber once so a scoped recorder always sees the
/// shared `sv_run_operation` callsite.
pub(super) fn sv_keep_callsite_interest_enabled() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        let _ = tracing::subscriber::set_global_default(SVInterestSubscriber);
    });
}
