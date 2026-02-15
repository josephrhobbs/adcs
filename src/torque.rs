//! ADCS
//! Copyright (c) 2026 Joseph Hobbs
//!
//! Torque type.

use std::ops::{
    Add,
    Sub,
    Neg,
};

use pyo3::prelude::*;
use pyo3::types::PyType;

#[pyclass]
#[derive(Clone, Copy, Debug)]
/// Torque vector.
///
/// Note that torque vectors are, by default, given in the _body frame_.
pub struct Torque {
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
impl Torque {
    #[new]
    /// Construct a new torque vector.
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
            "Torque({}, {}, {})",
            self.x,
            self.y,
            self.z,
        )
    }

    /// Add two torque vectors.
    fn __add__(&self, other: Self) -> Self {
        *self + other
    }

    /// Subtract two torque vectors.
    fn __sub__(&self, other: Self) -> Self {
        *self - other
    }

    /// Negate an torque vector.
    fn __neg__(&self) -> Self {
        -(*self)
    }
}

impl Add<Torque> for Torque {
    type Output = Torque;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub<Torque> for Torque {
    type Output = Torque;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Neg for Torque {
    type Output = Torque;
    
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
