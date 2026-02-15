//! ADCS
//! Copyright (c) 2026 Joseph Hobbs
//!
//! Integrator abstraction.

mod forward_euler;

use crate::{
    AngularVelocity,
    Quaternion,
    State,
};

pub use forward_euler::ForwardEuler;

/// Numerical integrator for Ordinary Differential Equations (ODEs).
pub trait Integrator {
    /// Determine the time derivatives of both attitude and angular velocity.
    fn dynamics(&self, state: State) -> (Quaternion, AngularVelocity) {
        let (q, w) = (state.quaternion, state.angular_velocity);

        // Time derivatives
        let qdot = q.diff(w);
        let wdot = w.diff(state.inertia, state.torque);

        (qdot, wdot)
    }

    /// Perform one integration step.
    fn step(&self, state: State) -> State;
}
