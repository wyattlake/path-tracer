use nalgebra::Matrix3;

pub trait Pack {
    fn pack_f32(&self, buffer_f32: &mut Vec<f32>);
    fn pack_i32(&self, buffer_i32: &mut Vec<i32>);
}

impl Pack for f32 {
    fn pack_f32(&self, buffer_f32: &mut Vec<f32>) {
        buffer_f32.push(*self);
    }

    fn pack_i32(&self, _buffer_i32: &mut Vec<i32>) {
        unreachable!()
    }
}

impl<T: Pack> Pack for Matrix3<T> {
    fn pack_f32(&self, buffer_f32: &mut Vec<f32>) {
        for x in self.as_slice() {
            x.pack_f32(buffer_f32);
        }
    }

    fn pack_i32(&self, _buffer_i32: &mut Vec<i32>) {
        unreachable!()
    }
}
