//! ADCS
//! Copyright (c) 2026 Joseph Hobbs
//! 
//! Rigid-body damper.

use pyo3::prelude::*;

use crate::{
    AngularVelocity,
    Inertia,
};

#[pyclass]
#[derive(Clone, Copy, Debug)]
/// Kane damper.
/// 
/// The _Kane damper_ is a simple model of rigid-body damping used to account
/// for energy dissipation in rotating rigid bodies.
/// 
/// A conceptual model of the Kane damper is a solid sphere inside the rigid
/// body suspended in a viscous fluid.  As the rigid body rotates, the solid
/// sphere begins to rotate along with it.  The torque due to viscous forces
/// is proportional to the vector difference of the sphere's angular velocity
/// and the rigid body's angular velocity.
pub struct KaneDamper {
    #[pyo3(get, set)]
    /// Damper inertia.
    pub inertia: Inertia,

    #[pyo3(get, set)]
    /// Damping coefficient.
    pub coefficient: f64,

    #[pyo3(get, set)]
    /// Damper angular velocity.
    pub angular_velocity: AngularVelocity,
}

#[pymethods]
impl KaneDamper {
    #[new]
    /// Construct a new Kane damper, given a damper inertia and damping coefficient.
    /// 
    /// Note that, when constructing a Kane damper, a _scalar_ inertia is specified.
    /// This is because the Kane damper model of energy dissipation conceptually
    /// models the damper as a solid sphere.  Because the inertia of a sphere is equal
    /// about any axis, a positive scalar is sufficient.
    pub fn new(inertia: f64, coefficient: f64) -> Self {
        let inertia = Inertia::new(
            inertia,
            inertia,
            inertia,
            0.0,
            0.0,
            0.0,
        );
        Self {
            inertia,
            coefficient,
            angular_velocity: AngularVelocity::new(0.0, 0.0, 0.0),
        }
    }
}
