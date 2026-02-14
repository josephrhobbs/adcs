//! ADCS
//! Copyright (c) 2026 Joseph Hobbs
//!
//! System state abstraction.

use pyo3::prelude::*;

use crate::{
    AngularVelocity,
    Quaternion,
};

#[pyclass]
#[derive(Clone, Copy, Debug)]
/// Attitude state.
pub struct State {
    /// Attitude.
    pub attitude: Quaternion, 

    /// Angular velocity.
    pub angular_velocity: AngularVelocity,
}
