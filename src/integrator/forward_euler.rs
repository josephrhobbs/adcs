//! ADCS
//! Copyright (c) 2026 Joseph Hobbs
//!
//! Forward Euler integrator.

use pyo3::prelude::*;

use crate::{
    Integrator,
    State,
};

#[pyclass]
/// Forward Euler integrator for rigid-body motion.
pub struct ForwardEuler {
    // Time step.
    h: f32,
}

#[pymethods]
impl ForwardEuler {
    #[new]
    /// Construct a new forward Euler integrator.
    pub fn new(h: f32) -> Self {
        Self {
            h,
        }
    }

    /// Integrate one step.
    pub fn step(&self, state: State) -> State {
        let (qdot, wdot, wddot) = self.dynamics(state);

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

impl Integrator for ForwardEuler {
    fn step(&self, state: State) -> State {
        self.step(state)
    }
}
