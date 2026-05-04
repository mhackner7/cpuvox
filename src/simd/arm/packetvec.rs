use crate::simd::arm::simdf32::*;

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub struct Packed_Vec3_x4 {
    pub xs: SimdF32,
    pub ys: SimdF32,
    pub zs: SimdF32,
}

impl Packed_Vec3_x4 {
    #[inline(always)]
    pub fn new(xs: [f32; 4], ys: [f32; 4], zs: [f32; 4]) -> Self {
        Self {
            xs: xs.into(),
            ys: ys.into(),
            zs: zs.into(),
        }
    }
}
