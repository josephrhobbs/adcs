//! ADCS
//! Copyright (c) 2026 Joseph Hobbs
//!
//! Integrator abstraction.

mod forward_euler;

use crate::{
    AngularVelocity,
    Quaternion,
    State,
    Torque,
};

pub use forward_euler::ForwardEuler;

/// Numerical integrator for Ordinary Differential Equations (ODEs).
pub trait Integrator {
    /// Determine the time derivatives of both attitude and angular velocity for the
    /// rigid body, and the time derivative of angular velocity for the simulated damper.
    fn dynamics(&self, state: State) -> (Quaternion, AngularVelocity, AngularVelocity) {
        let (q, w) = (state.quaternion, state.angular_velocity);

        // Applied torque
        let mut t = state.torque;

        // Damper velocity derivative (if damping present)
        let wddot = if let Some (d) = state.damper {
            let wd = d.angular_velocity;
            
            // Damping torque (damper ON rigid body)
            let wdiff = wd - w;
            let td = Torque::new(
                wdiff.x,
                wdiff.y,
                wdiff.z,
            ).scale(d.coefficient);

            // Update applied torque
            t = t + td;

            wd.diff(d.inertia, -td)
        } else {
            AngularVelocity::new(0.0, 0.0, 0.0)
        };

        // Rigid-body orientation derivative
        let qdot = q.diff(w);

        // Rigid-body velocity derivative
        let wdot = w.diff(state.inertia, t);

        (qdot, wdot, wddot)
    }

    /// Perform one integration step.
    fn step(&self, state: State) -> State;
}
