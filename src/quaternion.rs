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
use pyo3::types::PyType;

use crate::AngularVelocity;

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

    #[classmethod]
    /// Construct the unit quaternion.
    pub fn unit(_cls: &Bound<'_, PyType>) -> Self {
        Self {
            w: 1.0,
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    #[classmethod]
    /// Construct a new quaternion from a vector.
    /// 
    /// The resultant quaternion will have zero scalar part and specified vector part.
    pub fn from_vector(_cls: &Bound<'_, PyType>, x: f32, y: f32, z: f32) -> Self {
        Self {
            w: 0.0,
            x,
            y,
            z,
        }
    }

    #[classmethod]
    /// Construct a new quaternion from an angle and axis vector.
    ///
    /// Note that the angle should be in radians and the axis should be a unit normal vector.
    /// If the axis does not have unit norm, then the axis will be rescaled automatically.
    pub fn from_rotation(cls: &Bound<'_, PyType>, angle: f32, x: f32, y: f32, z: f32) -> Self {
        let c = (angle/2.0).cos();
        let s = (angle/2.0).sin();
        let a = Self::from_vector(cls, x, y, z).normalize();
        
        Self {
            w: c,
            x: s * a.x,
            y: s * a.y,
            z: s * a.z,
        }
    }

    /// Compute the norm of this quaternion.
    pub fn norm(&self) -> f32 {
        (
            self.w * self.w +
            self.x * self.x +
            self.y * self.y +
            self.z * self.z
        ).sqrt()
    }

    /// Scale this quaternion by a given scalar.
    pub fn scale(&self, s: f32) -> Self {
        Self {
            w: s * self.w,
            x: s * self.x,
            y: s * self.y,
            z: s * self.z,
        }
    }

    /// Return the unit quaternion in the direction of this quaternion.
    pub fn normalize(&self) -> Self {
        self.scale(self.norm().powi(-1))
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

    /// Negate a quaternion.
    fn __neg__(&self) -> Self {
        -(*self)
    }

    /// Return the quaternion inverse of this quaternion.
    pub fn inv(&self) -> Self {
        Self {
            w: self.w,
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }.scale(self.norm().powi(-2))
    }

    /// Given an angular velocity vector _in the body frame_, return
    /// the time derivative of this quaternion.
    pub fn diff(&self, angular_velocity: AngularVelocity) -> Self {
        // Lie algebra so(3) element corresponding to angular velocity
        let omega = Self {
            w: 0.0,
            x: angular_velocity.x,
            y: angular_velocity.y,
            z: angular_velocity.z,
        };

        (*self * omega).scale(0.5)
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

    #[getter]
    /// Get the vector part of this quaternion.
    pub fn get_vector(&self) -> (f32, f32, f32) {
        (self.x, self.y, self.z)
    }
    
    #[getter]
    /// Get the scalar part of this quaternion.
    pub fn get_scalar(&self) -> f32 {
        self.w
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
            w: self.w * other.w - self.x * other.x - self.y * other.y - self.z * other.z,
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
