//! ADCS
//! Copyright (c) 2026 Joseph Hobbs
//!
//! Inertia Tensor.

use pyo3::prelude::*;

#[pyclass]
#[derive(Clone, Copy, Debug)]
/// Inertia tensor.
/// 
/// The inertia tensor can be written as a symmetric, positive semi-definite matrix of dimension 3.
/// 
/// In this package, to conserve memory, we adopt "Voigt notation" to write the tensor
/// ```
/// J = [[ J11 J12 J13 ]
///      [ J21 J22 J23 ]
///      [ J31 J32 J33 ]]
/// ```
/// in terms of its six degrees of freedom, like so.
/// ```
/// J = [[ J1 J6 J5 ]
///      [ J6 J2 J4 ]
///      [ J5 J4 J3 ]]
/// ```
pub struct Inertia {
    #[pyo3(get, set)]
    pub j1: f64,

    #[pyo3(get, set)]
    pub j2: f64,

    #[pyo3(get, set)]
    pub j3: f64,

    #[pyo3(get, set)]
    pub j4: f64,

    #[pyo3(get, set)]
    pub j5: f64,

    #[pyo3(get, set)]
    pub j6: f64,
}

#[pymethods]
impl Inertia {
    #[new]
    /// Construct a new inertia tensor.
    pub fn new(j1: f64, j2: f64, j3: f64, j4: f64, j5: f64, j6: f64) -> Self {
        Self {
            j1,
            j2,
            j3,
            j4,
            j5,
            j6,
        }
    }

    /// Return a human-readable string for this inertia tensor.
    fn __str__(&self) -> String {
        format!(
            "[{:>10.6} {:>10.6} {:>10.6}]\n[{:>10.6} {:>10.6} {:>10.6}]\n[{:>10.6} {:>10.6} {:>10.6}]",
            self.j1,
            self.j6,
            self.j5,
            self.j6,
            self.j2,
            self.j4,
            self.j5,
            self.j4,
            self.j3,
        )
    }

    /// Return a Pythonic representation of this inertia tensor.
    fn __repr__(&self) -> String {
        format!(
            "Inertia({}, {}, {}, {}, {}, {})",
            self.j1,
            self.j2,
            self.j3,
            self.j4,
            self.j5,
            self.j6,
        )
    }
}
