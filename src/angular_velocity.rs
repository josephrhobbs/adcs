//! ADCS
//! Copyright (c) 2026 Joseph Hobbs
//!
//! Angular Velocity type.

use pyo3::prelude::*;

#[pyclass]
#[derive(Clone, Copy, Debug)]
/// Angular velocity vector.
pub struct AngularVelocity {
    /// X coordinate.
    pub x: f32,

    /// Y coordinate.
    pub y: f32,

    /// Z coordinate.
    pub z: f32,
}
