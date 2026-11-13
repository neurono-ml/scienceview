//! One registered script and everything the runtime keeps for it.

use std::collections::BTreeMap;

use crate::engine::SVScriptProgram;

use super::super::SVScript;

pub(crate) struct SVScriptRegistration {
    pub(crate) script: SVScript,
    pub(crate) program: SVScriptProgram,
    pub(crate) state: BTreeMap<String, f64>,
    pub(crate) invocations: u64,
}
