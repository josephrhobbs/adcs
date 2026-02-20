//! ADCS
//! Copyright (c) 2026 Joseph Hobbs
//!
//! Angular Momentum type.

use std::ops::{
    Add,
    Sub,
    Neg,
};

use pyo3::prelude::*;
use pyo3::types::PyType;

use crate::{
    AngularVelocity,
    Inertia,
    Quaternion,
};

#[pyclass]
#[derive(Clone, Copy, Debug)]
/// Angular momentum vector.
///
/// Note that angular momentum vectors are, by default, given in the _body frame_.
pub struct AngularMomentum {
    #[pyo3(get, set)]
    /// X coordinate.
    pub x: f32,

    #[pyo3(get, set)]
    /// Y coordinate.
    pub y: f32,

    #[pyo3(get, set)]
    /// Z coordinate.
    pub z: f32,
}

#[pymethods]
impl AngularMomentum {
    #[new]
    /// Construct a new angular momentum vector.
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x,
            y,
            z,
        }
    }

    #[classmethod]
    /// Construct the zero vector.
    pub fn zero(_cls: &Bound<'_, PyType>) -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    #[classmethod]
    /// Construct an angular momentum vector from an inertia tensor
    /// and an angular velocity.
    pub fn product(
        _cls: &Bound<'_, PyType>,
        inertia: Inertia,
        angular_velocity: AngularVelocity,
    ) -> Self {
        Self {
            x: inertia.j1*angular_velocity.x + inertia.j6*angular_velocity.y + inertia.j5*angular_velocity.z,
            y: inertia.j6*angular_velocity.x + inertia.j2*angular_velocity.y + inertia.j4*angular_velocity.z,
            z: inertia.j5*angular_velocity.x + inertia.j4*angular_velocity.y + inertia.j3*angular_velocity.z,
        }
    }

    /// Rotate this vector by a given unit quaternion.
    pub fn rotate(&self, q: Quaternion) -> Self {
        let v = Quaternion::new(
            0.0,
            self.x,
            self.y,
            self.z,
        );

        // Rotate
        let rotated = q * v * q.inv();

        Self {
            x: rotated.x,
            y: rotated.y,
            z: rotated.z,
        }
    }

    /// Scale this vector by a given scalar.
    pub fn scale(&self, s: f32) -> Self {
        Self {
            x: s * self.x,
            y: s * self.y,
            z: s * self.z,
        }
    }

    /// Return a human-readable string for this vector.
    fn __str__(&self) -> String {
        format!(
            "i{:.6} + j{:.6} + k{:.6}",
            self.x,
            self.y,
            self.z,
        )
    }

    /// Return a Pythonic representation of this vector.
    fn __repr__(&self) -> String {
        format!(
            "AngularMomentum({}, {}, {})",
            self.x,
            self.y,
            self.z,
        )
    }

    /// Add two angular momentum vectors.
    fn __add__(&self, other: Self) -> Self {
        *self + other
    }

    /// Subtract two angular momentum vectors.
    fn __sub__(&self, other: Self) -> Self {
        *self - other
    }

    /// Negate an angular momentum vector.
    fn __neg__(&self) -> Self {
        -(*self)
    }
}

impl Add<AngularMomentum> for AngularMomentum {
    type Output = AngularMomentum;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub<AngularMomentum> for AngularMomentum {
    type Output = AngularMomentum;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Neg for AngularMomentum {
    type Output = AngularMomentum;
    
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
