use nalgebra::Matrix3;

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
        pack_id: $pack_id:expr,
        $($field_name:ident: $field_type:ty,)*
    }) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            pack_id: u8,
            $($field_name: $field_type,)*
        }

        impl $name {
            pub fn new($($field_name: $field_type,)*) -> $name {
                $name {
                    pack_id: $pack_id,
                    $($field_name,)*
                }
            }

            pub fn get_pack_id(&self) -> u8 {
                return self.pack_id;
            }

            $(paste::item! {
                pub fn [< get_$field_name >] (&self) -> &$field_type {
                    return &self.$field_name;
                }
            })*
        }

        impl Pack for $name {
            fn pack(&self, buffer_f32: &mut Vec<f32>, buffer_u8: &mut Vec<u8>) {
                $pack_id.pack(buffer_f32, buffer_u8);
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

impl<T: Pack> Pack for Matrix3<T> {
    fn pack(&self, buffer_f32: &mut Vec<f32>, buffer_u8: &mut Vec<u8>) {
        for x in self.as_slice() {
            x.pack(buffer_f32, buffer_u8);
        }
    }
}

impl Pack for bool {
    fn pack(&self, _buffer_f32: &mut Vec<f32>, buffer_u8: &mut Vec<u8>) {
        buffer_u8.push(*self as u8);
    }
}
