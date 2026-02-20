//! ADCS
//! Copyright (c) 2026 Joseph Hobbs
//!
//! Angular Velocity type.

use std::ops::{
    Add,
    Sub,
    Neg,
};

use pyo3::prelude::*;

use crate::{
    Inertia,
    Quaternion,
    Torque,
};

#[pyclass]
#[derive(Clone, Copy, Debug)]
/// Angular velocity vector.
///
/// Note that angular velocity vectors are, by default, given in the _body frame_.
pub struct AngularVelocity {
    #[pyo3(get, set)]
    /// X coordinate.
    pub x: f64,

    #[pyo3(get, set)]
    /// Y coordinate.
    pub y: f64,

    #[pyo3(get, set)]
    /// Z coordinate.
    pub z: f64,
}

#[pymethods]
impl AngularVelocity {
    #[new]
    /// Construct a new angular velocity vector.
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            x,
            y,
            z,
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
    pub fn scale(&self, s: f64) -> Self {
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
            "AngularVelocity({}, {}, {})",
            self.x,
            self.y,
            self.z,
        )
    }

    /// Add two angular velocity vectors.
    fn __add__(&self, other: Self) -> Self {
        *self + other
    }

    /// Subtract two angular velocity vectors.
    fn __sub__(&self, other: Self) -> Self {
        *self - other
    }

    /// Negate an angular velocity vector.
    fn __neg__(&self) -> Self {
        -(*self)
    }

    /// Given an input torque and inertia tensor, determine an angular acceleration vector.
    /// 
    /// The time derivative of angular velocity, in the general case, is given by
    /// ```
    /// omega_dot = inv(J) @ ( torque - omega.cross(J @ omega) )
    /// ```
    pub fn diff(&self, inertia: Inertia, torque: Torque) -> Self {
        // Angular momentum components
        let hx = inertia.j1*self.x + inertia.j6*self.y + inertia.j5*self.z;
        let hy = inertia.j6*self.x + inertia.j2*self.y + inertia.j4*self.z;
        let hz = inertia.j5*self.x + inertia.j4*self.y + inertia.j3*self.z;

        // Torque-free contribution to "fictitious torque"
        let torque_free = -Torque::new(
            self.y*hz - self.z*hy,
            self.z*hx - self.x*hz,
            self.x*hy - self.y*hx,
        );

        // Determinant of inertia matrix
        let det = inertia.j1*(
            inertia.j2 * inertia.j3 - inertia.j4.powi(2)
        ) + inertia.j6*(
            inertia.j4 * inertia.j5 - inertia.j3 * inertia.j6
        ) + inertia.j5*(
            inertia.j4 * inertia.j6 - inertia.j2 * inertia.j5
        );

        // Components of inverse inertia (Voigt notation)
        let i1 = (inertia.j2*inertia.j3 - inertia.j4.powi(2)) / det;
        let i2 = (inertia.j1*inertia.j3 - inertia.j5.powi(2)) / det;
        let i3 = (inertia.j1*inertia.j2 - inertia.j6.powi(2)) / det;
        let i4 = (inertia.j5*inertia.j6 - inertia.j1*inertia.j4) / det;
        let i5 = (inertia.j4*inertia.j6 - inertia.j2*inertia.j5) / det;
        let i6 = (inertia.j5*inertia.j4 - inertia.j3*inertia.j6) / det;

        // Torque and torque-free components
        let t = torque + torque_free;
    
        Self {
            x: i1*t.x + i6*t.y + i5*t.z,
            y: i6*t.x + i2*t.y + i4*t.z,
            z: i5*t.x + i4*t.y + i3*t.z,
        }
    }
}

impl Add<AngularVelocity> for AngularVelocity {
    type Output = AngularVelocity;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub<AngularVelocity> for AngularVelocity {
    type Output = AngularVelocity;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Neg for AngularVelocity {
    type Output = AngularVelocity;
    
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
