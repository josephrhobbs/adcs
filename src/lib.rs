//! ADCS
//! Copyright (c) 2026 Joseph Hobbs
//!
//! A modern software package for blazingly fast simulation of rigid-body mechanics.

mod angular_velocity;
mod integrator;
mod quaternion;
mod state;

use pyo3::prelude::*;

pub use angular_velocity::AngularVelocity;
pub use quaternion::Quaternion;

/// A Python module implemented in Rust.
#[pymodule]
fn adcs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Quaternion>()?;

    Ok (())
}
