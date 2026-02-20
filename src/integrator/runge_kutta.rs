//! ADCS
//! Copyright (c) 2026 Joseph Hobbs
//!
//! Runge-Kutta integrators.

use pyo3::prelude::*;

use crate::{
    Integrator,
    State,
};

#[pyclass]
/// Fourth-order Runge-Kutta integrator for rigid-body motion.
pub struct RungeKutta4 {
    // Time step.
    h: f32,
}

#[pymethods]
impl RungeKutta4 {
    #[new]
    /// Construct a new fourth-order Runge-Kutta integrator.
    pub fn new(h: f32) -> Self {
        Self {
            h,
        }
    }

    /// Integrate one step.
    pub fn step(&self, state: State) -> State {
        // First step
        let (qdot1, wdot1, wddot1) = self.dynamics(state);

        // Second step
        let mut k2 = state.clone();
        k2.quaternion = (state.quaternion + qdot1.scale(0.5 * self.h)).normalize();
        k2.angular_velocity = state.angular_velocity + wdot1.scale(0.5 * self.h);
        if let Some (d) = state.damper {
            // This is safe because we cloned the original state
            let mut newd = k2.damper.unwrap();

            newd.angular_velocity = d.angular_velocity + wddot1.scale(0.5 * self.h);
            k2.damper = Some (newd);
        }
        let (qdot2, wdot2, wddot2) = self.dynamics(k2);

        // Third step
        let mut k3 = state.clone();
        k3.quaternion = (state.quaternion + qdot2.scale(0.5 * self.h)).normalize();
        k3.angular_velocity = state.angular_velocity + wdot2.scale(0.5 * self.h);
        if let Some (d) = state.damper {
            // This is safe because we cloned the original state
            let mut newd = k3.damper.unwrap();

            newd.angular_velocity = d.angular_velocity + wddot2.scale(0.5 * self.h);
            k3.damper = Some (newd);
        }
        let (qdot3, wdot3, wddot3) = self.dynamics(k3);

        // Fourth step
        let mut k4 = state.clone();
        k4.quaternion = (state.quaternion + qdot3.scale(self.h)).normalize();
        k4.angular_velocity = state.angular_velocity + wdot3.scale(self.h);
        if let Some (d) = state.damper {
            // This is safe because we cloned the original state
            let mut newd = k4.damper.unwrap();

            newd.angular_velocity = d.angular_velocity + wddot3.scale(self.h);
            k4.damper = Some (newd);
        }
        let (qdot4, wdot4, wddot4) = self.dynamics(k4);

        // Combine
        let qdot = (qdot1 + qdot2.scale(2.0) + qdot3.scale(2.0) + qdot4).scale(1.0/6.0);
        let wdot = (wdot1 + wdot2.scale(2.0) + wdot3.scale(2.0) + wdot4).scale(1.0/6.0);
        let wddot = (wddot1 + wddot2.scale(2.0) + wddot3.scale(2.0) + wddot4).scale(1.0/6.0);

        // Construct new state
        let mut newstate = state.clone();
        newstate.quaternion = (state.quaternion + qdot.scale(self.h)).normalize();
        newstate.angular_velocity = state.angular_velocity + wdot.scale(self.h);
        if let Some (d) = state.damper {
            // This is safe because we cloned the original state
            let mut newd = newstate.damper.unwrap();

            newd.angular_velocity = d.angular_velocity + wddot.scale(self.h);
            newstate.damper = Some (newd);
        }

        // Step time
        newstate.time = state.time + self.h;

        newstate
    }
}

impl Integrator for RungeKutta4 {
    fn step(&self, state: State) -> State {
        self.step(state)
    }
}
