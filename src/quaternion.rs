//! ADCS
//! Copyright (c) 2026 Joseph Hobbs
//!
//! Quaternion type.

use std::ops::{
    Add,
    Mul,
    Neg,
    Sub,
};

use pyo3::prelude::*;

#[pyclass]
#[derive(Clone, Copy, Debug)]
/// Quaternion.
pub struct Quaternion {
    #[pyo3(get, set)]
    /// Scalar part of the quaternion.
    pub w: f32,

    #[pyo3(get, set)]
    /// Vector part of the quaternion, X coordinate.
    pub x: f32,

    #[pyo3(get, set)]
    /// Vector part of the quaternion, Y coordinate.
    pub y: f32,

    #[pyo3(get, set)]
    /// Vector part of the quaternion, Z coordinate.
    pub z: f32,
}

#[pymethods]
impl Quaternion {
    #[new]
    /// Construct a new quaternion.
    pub fn new(w: f32, x: f32, y: f32, z: f32) -> Self {
        Self {
            w,
            x,
            y,
            z,
        }
    }

    /// Compute the norm of this quaternion.
    fn norm(&self) -> f32 {
        (
            self.w * self.w +
            self.x * self.x +
            self.y * self.y +
            self.z * self.z
        ).sqrt()
    }

    /// Add two quaternions.
    fn __add__(&self, other: Self) -> Self {
        *self + other
    }

    /// Subtract two quaternions.
    fn __sub__(&self, other: Self) -> Self {
        *self - other
    }

    /// Multiply two quaternions.
    fn __mul__(&self, other: Self) -> Self {
        *self * other
    }

    /// Return the quaternion inverse of this quaternion.
    fn inv(&self) -> Self {
        let normsq = self.norm().powi(2);
        Self {
            w: self.w / normsq,
            x: -self.x / normsq,
            y: -self.y / normsq,
            z: -self.z / normsq,
        }
    }

    /// Return a human-readable string for this quaternion.
    fn __str__(&self) -> String {
        format!(
            "{:.6} + i{:.6} + j{:.6} + k{:.6}",
            self.w,
            self.x,
            self.y,
            self.z,
        )
    }

    /// Return a Pythonic representation of this quaternion.
    fn __repr__(&self) -> String {
        format!(
            "Quaternion({}, {}, {}, {})",
            self.w,
            self.x,
            self.y,
            self.z,
        )
    }
}

impl Add<Quaternion> for Quaternion {
    type Output = Quaternion;

    fn add(self, other: Self) -> Self::Output {
        Self {
            w: self.w + other.w,
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub<Quaternion> for Quaternion {
    type Output = Quaternion;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            w: self.w - other.w,
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<Quaternion> for Quaternion {
    type Output = Quaternion;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            w: self.w * other.w + self.x * other.x + self.y * other.y + self.z * other.z,
            x: self.w * other.x + other.w * self.x + self.y * other.z - self.z * other.y,
            y: self.w * other.y + other.w * self.y + self.z * other.x - self.x * other.z,
            z: self.w * other.z + other.w * self.z + self.x * other.y - self.y * other.x,
        }
    }
}

impl Neg for Quaternion {
    type Output = Quaternion;
    
    fn neg(self) -> Self::Output {
        Self {
            w: -self.w,
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
