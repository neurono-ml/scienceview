//! One refresh cycle over every shared source.

use super::core_implementation::SVDashboard;
use crate::sv_error::SVError;
use crate::sv_events::{
    SVDashboardChange, SVDashboardDelivery, SVDashboardDispatch, SVRefreshTrigger,
};

/// The report of one refresh cycle.
#[derive(Clone, Debug, PartialEq)]
pub struct SVRefreshReport {
    trigger: SVRefreshTrigger,
    sequence: u64,
    source_reads: Vec<String>,
    dispatch: SVDashboardDispatch,
}

impl SVRefreshReport {
    /// Returns the trigger that started the cycle.
    pub fn trigger(&self) -> &SVRefreshTrigger {
        &self.trigger
    }

    /// Returns the monotonic refresh sequence number.
    pub fn sequence(&self) -> u64 {
        self.sequence
    }

    /// Returns the shared sources resolved exactly once by this cycle.
    pub fn source_reads(&self) -> &[String] {
        &self.source_reads
    }

    /// Returns the panels the cycle updated.
    pub fn dispatch(&self) -> &SVDashboardDispatch {
        &self.dispatch
    }
}

impl SVDashboard<'_> {
    /// Runs one refresh cycle over every shared source.
    ///
    /// Each source is resolved at most once and every panel bound to it is
    /// reported exactly once, so a source shared by two cells fans one read out
    /// to both. The read is dropped immediately: the cycle invalidates and
    /// reports, it never retains a second copy of the data.
    ///
    /// # Errors
    ///
    /// Propagates a shared source's column resolution error.
    pub fn refresh(&mut self, trigger: SVRefreshTrigger) -> Result<SVRefreshReport, SVError> {
        self.refresh_sequence = self.refresh_sequence.saturating_add(1);
        let sequence = self.refresh_sequence;
        let mut source_reads = Vec::new();
        let mut deliveries = Vec::new();

        for source in &self.sources {
            if let Some(descriptor) = source.source().schema().columns().first() {
                drop(source.column(descriptor.name())?);
                source_reads.push(source.identifier().to_owned());
            }
            for panel in self
                .panels
                .iter()
                .filter(|panel| panel.source_identifier() == source.identifier())
            {
                deliveries.push(SVDashboardDelivery::new(
                    panel.identifier(),
                    SVDashboardChange::Refresh {
                        sequence,
                        trigger: trigger.clone(),
                    },
                ));
            }
        }
        Ok(SVRefreshReport {
            trigger,
            sequence,
            source_reads,
            dispatch: SVDashboardDispatch::new(deliveries),
        })
    }
}
