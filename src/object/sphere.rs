use crate::{pack_object, Material, Object, Pack, Transform};

// Sphere object which can be added to scenes
pack_object! {
    struct Sphere {
        object_id: 0,
        transform: Transform,
        material: Material,
    }
}

impl Sphere {
    pub fn default() -> Sphere {
        Sphere::new(Transform::identity(), Material::default())
    }
}
