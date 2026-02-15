//! ADCS
//! Copyright (c) 2026 Joseph Hobbs
//!
//! Rigid-body state abstraction.

use pyo3::prelude::*;

use crate::{
    AngularVelocity,
    Inertia,
    Quaternion,
    Torque,
};

#[pyclass]
#[derive(Clone, Copy, Debug)]
/// Rigid-body state.
pub struct State {
    #[pyo3(get, set)]
    /// Attitude.
    pub quaternion: Quaternion, 

    #[pyo3(get, set)]
    /// Angular velocity.
    pub angular_velocity: AngularVelocity,

    #[pyo3(get, set)]
    /// Rigid-body inertia.
    pub inertia: Inertia,

    #[pyo3(get, set)]
    /// Input torques (body frame).
    pub torque: Torque,
}

#[pymethods]
impl State {
    #[new]
    /// Initialize a new state, with body at default attitude and no velocities or torque.
    fn new(inertia: Inertia) -> Self {
        Self {
            quaternion: Quaternion::new(1.0, 0.0, 0.0, 0.0),
            angular_velocity: AngularVelocity::new(0.0, 0.0, 0.0),
            inertia,
            torque: Torque::new(0.0, 0.0, 0.0),
        }
    }
}
