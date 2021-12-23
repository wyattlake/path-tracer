use nalgebra::Matrix3;

use crate::{pack_object, Pack, Transform};

use super::Object;

// Sphere object which can be added to scenes
pack_object! {
    struct Sphere {
        pack_id: 0,
        transform: Transform,
        invisible: bool,
    }
}

impl Sphere {
    pub fn default() -> Sphere {
        Sphere::new(Transform::new(Matrix3::identity()), false)
    }
}

impl Object for Sphere {}
