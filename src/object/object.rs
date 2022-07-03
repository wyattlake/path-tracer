use std::fmt::Debug;

use crate::{Material, Pack, Transform};

// All of the essential components to being an object
pub trait Object: Debug + Pack {
    fn get_object_id(&self) -> u8;

    fn get_transform(&self) -> &Transform;
    fn set_transform(&mut self, transform: Transform);

    fn get_material(&self) -> &Material;
    fn set_material(&mut self, material: Material);
}
