//! ADCS
//! Copyright (c) 2026 Joseph Hobbs
//!
//! Rotation Kinematics

use std::ops::{
    Add,
    Sub,
    Mul,
    Div,
};

use pyo3::prelude::*;

#[pyclass]
#[derive(Clone, Copy, Debug)]
/// Quaternion.
pub struct Quaternion {
    /// Scalar part of the quaternion.
    pub w: f32,

    /// Vector part of the quaternion, X coordinate.
    pub x: f32,

    /// Vector part of the quaternion, Y coordinate.
    pub y: f32,

    /// Vector part of the quaternion, Z coordinate.
    pub z: f32,
}

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
