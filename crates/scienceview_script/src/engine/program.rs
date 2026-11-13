//! The prepared form of one script: what an engine produces and executes.

use super::SVScriptEngineKind;
use crate::directive::SVScriptStatement;

/// The body an engine prepared.
pub(crate) enum SVScriptProgramBody {
    /// Statements already parsed and resolved into semantic actions.
    Directives(Vec<SVScriptStatement>),
    /// Source text a host interpreter prepares itself at execution time.
    Source(String),
}

/// One source prepared by one engine.
///
/// Preparation is separate from execution because it is the expensive step: a
/// script is parsed once and invoked many times. Reload preparation also makes
/// a replacement atomic — a source that fails to parse never reaches the
/// registry, so the previous program keeps running.
pub struct SVScriptProgram {
    kind: SVScriptEngineKind,
    digest: u64,
    body: SVScriptProgramBody,
}

impl SVScriptProgram {
    /// Builds a program from parsed directives.
    pub(crate) fn from_directives(
        kind: SVScriptEngineKind,
        digest: u64,
        statements: Vec<SVScriptStatement>,
    ) -> Self {
        Self {
            kind,
            digest,
            body: SVScriptProgramBody::Directives(statements),
        }
    }

    /// Builds a program that a host interpreter prepares itself.
    pub(crate) fn from_source(kind: SVScriptEngineKind, digest: u64, source: String) -> Self {
        Self {
            kind,
            digest,
            body: SVScriptProgramBody::Source(source),
        }
    }

    /// Returns the engine kind that prepared the program.
    pub fn kind(&self) -> SVScriptEngineKind {
        self.kind
    }

    /// Returns the FNV-1a digest of the prepared source.
    ///
    /// The digest changes whenever the source changes, which is what a reload
    /// observability span reports.
    pub fn source_digest(&self) -> u64 {
        self.digest
    }

    /// Returns the number of resolved statements, when the engine parsed them.
    ///
    /// A host interpreter that prepares its source itself returns `None`.
    pub fn statement_count(&self) -> Option<usize> {
        match &self.body {
            SVScriptProgramBody::Directives(statements) => Some(statements.len()),
            SVScriptProgramBody::Source(_) => None,
        }
    }

    /// Returns the parsed statements, when the engine parsed them.
    pub(crate) fn directives(&self) -> Option<&[SVScriptStatement]> {
        match &self.body {
            SVScriptProgramBody::Directives(statements) => Some(statements),
            SVScriptProgramBody::Source(_) => None,
        }
    }

    /// Returns the source text when the engine did not parse it itself.
    pub fn source_text(&self) -> Option<&str> {
        match &self.body {
            SVScriptProgramBody::Directives(_) => None,
            SVScriptProgramBody::Source(source) => Some(source),
        }
    }
}

/// Hashes a source with FNV-1a, dependency-free and stable across builds.
pub(crate) fn sv_source_digest(source: &str) -> u64 {
    let mut digest = 0xcbf2_9ce4_8422_2325_u64;
    for byte in source.as_bytes() {
        digest ^= u64::from(*byte);
        digest = digest.wrapping_mul(0x0000_0100_0000_01b3);
    }
    digest
}
