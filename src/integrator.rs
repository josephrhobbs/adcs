//! ADCS
//! Copyright (c) 2026 Joseph Hobbs
//!
//! Integrator abstraction.

use crate::state::State;

/// Numerical integrator for Ordinary Differential Equations (ODEs).
pub trait Integrator {
    /// Perform one integration step.
    fn step(&self, x0: State) -> State;
}
