//! ADCS
//! Copyright (c) 2026 Joseph Hobbs
//! 
//! Rigid-body damper.

use pyo3::prelude::*;

use crate::{
    AngularVelocity,
    Inertia,
};

#[pyclass]
#[derive(Clone, Copy, Debug)]
/// Kane damper.
/// 
/// The _Kane damper_ is a simple model of rigid-body damping used to account
/// for energy dissipation in rotating rigid bodies.
pub struct KaneDamper {
    #[pyo3(get, set)]
    /// Damper inertia.
    pub inertia: Inertia,

    #[pyo3(get, set)]
    /// Damping coefficient.
    pub coefficient: f32,

    #[pyo3(get, set)]
    /// Damper angular velocity.
    pub angular_velocity: AngularVelocity,
}

#[pymethods]
impl KaneDamper {
    #[new]
    /// Construct a new Kane damper, given a damper inertia and damping coefficient.
    pub fn new(inertia: Inertia, coefficient: f32) -> Self {
        Self {
            inertia,
            coefficient,
            angular_velocity: AngularVelocity::new(0.0, 0.0, 0.0),
        }
    }
}
