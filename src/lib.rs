//! ADCS
//! Copyright (c) 2026 Joseph Hobbs
//!
//! A modern software package for blazingly fast simulation of rigid-body mechanics.

mod angular_velocity;
mod damper;
mod inertia;
mod integrator;
mod quaternion;
mod state;
mod torque;

use pyo3::prelude::*;

pub use angular_velocity::AngularVelocity;
pub use damper::KaneDamper;
pub use inertia::Inertia;
pub use integrator::Integrator;
pub use quaternion::Quaternion;
pub use state::State;
pub use torque::Torque;

#[pymodule]
/// Blazingly fast rigid-body mechanics simulation.
mod adcs {
    #[pymodule_export]
    use crate::AngularVelocity;

    #[pymodule_export]
    use crate::KaneDamper;

    #[pymodule_export]
    use crate::Inertia;

    #[pymodule_export]
    use crate::Quaternion;

    #[pymodule_export]
    use crate::State;

    #[pymodule_export]
    use crate::Torque;

    #[pymodule_export]
    use crate::integrators;
}

#[pymodule]
mod integrators {
    #[pymodule_export]
    use crate::integrator::ForwardEuler;
}
