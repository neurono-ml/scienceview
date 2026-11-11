//! The solve pass, split into root sizing, node placement and distribution.

mod distribution;
mod node_solver;
mod root_solver;

pub(crate) use root_solver::sv_solve;
