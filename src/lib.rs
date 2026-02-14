//! ADCS
//! Copyright (c) 2026 Joseph Hobbs
//!
//! A modern software package for blazingly fast simulation of rigid-body mechanics.

mod integrator;
mod physics;
mod state;

use pyo3::prelude::*;

pub use physics::{
    AngularVelocity,
    Quaternion,
};

/// A Python module implemented in Rust.
#[pymodule]
fn adcs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // TODO

    Ok (())
}
