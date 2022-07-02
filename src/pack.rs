use nalgebra::{Matrix4, Scalar};

/// Allows struct data to be packed to a buffer and accessed in GPU kernels
pub trait Pack {
    /// Separates and packs discrete and non discrete data
    fn pack(&self, buffer_f32: &mut Vec<f32>, buffer_u8: &mut Vec<u8>);
}

/// Builds a new struct and implements Pack.
/// Includes getters and a constructor.
#[macro_export]
macro_rules! pack_object {
    (struct $name:ident {
        object_id: $object_id:expr,
        $($field_name:ident: $field_type:ty,)*
    }) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            object_id: u8,
            $($field_name: $field_type,)*
        }

        impl $name {
            pub fn new($($field_name: $field_type,)*) -> $name {
                $name {
                    object_id: $object_id,
                    $($field_name,)*
                }
            }

            pub fn get_object_id(&self) -> u8 {
                return self.object_id;
            }

            $(paste::item! {
                pub fn [< get_$field_name >] (&self) -> &$field_type {
                    return &self.$field_name;
                }
            })*
        }

        impl Pack for $name {
            fn pack(&self, buffer_f32: &mut Vec<f32>, buffer_u8: &mut Vec<u8>) {
                $object_id.pack(buffer_f32, buffer_u8);
                $(self.$field_name.pack(buffer_f32, buffer_u8);)*
            }
        }
    };
}

/// Packs structs without an id
#[macro_export]
macro_rules! pack_struct {
    (struct $name:ident {
        $($field_name:ident: $field_type:ty,)*
    }) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            $($field_name: $field_type,)*
        }

        impl $name {
            pub fn new($($field_name: $field_type,)*) -> $name {
                $name {
                    $($field_name,)*
                }
            }

            $(paste::item! {
                pub fn [< get_$field_name >] (&self) -> &$field_type {
                    return &self.$field_name;
                }
            })*
        }

        impl Pack for $name {
            fn pack(&self, buffer_f32: &mut Vec<f32>, buffer_u8: &mut Vec<u8>) {
                $(self.$field_name.pack(buffer_f32, buffer_u8);)*
            }
        }
    };
}

impl Pack for u8 {
    fn pack(&self, _buffer_f32: &mut Vec<f32>, buffer_u8: &mut Vec<u8>) {
        buffer_u8.push(*self);
    }
}

impl Pack for f32 {
    fn pack(&self, buffer_f32: &mut Vec<f32>, _buffer_u8: &mut Vec<u8>) {
        buffer_f32.push(*self);
    }
}

impl<T: Pack + Scalar> Pack for Matrix4<T> {
    fn pack(&self, buffer_f32: &mut Vec<f32>, buffer_u8: &mut Vec<u8>) {
        // Matrix is transposed because as_slice reads row by row
        for x in self.transpose().as_slice() {
            x.pack(buffer_f32, buffer_u8);
        }
    }
}

impl Pack for bool {
    fn pack(&self, _buffer_f32: &mut Vec<f32>, buffer_u8: &mut Vec<u8>) {
        buffer_u8.push(*self as u8);
    }
}
