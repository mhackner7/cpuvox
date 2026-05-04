use crate::simd::arm::simdf32::SimdF32;
use std::arch::aarch64::*;

pub struct SimdI32 {
    pub data: int32x4_t,
}

impl SimdI32 {
    #[inline(always)]
    pub fn to_array(self) -> [i32; 4] {
        unsafe {
            let mut a = [0i32; 4];
            vst1q_s32(a.as_mut_ptr(), self.data);
            return a;
        }
    }
    #[inline(always)]
    pub fn reinterpret_as_i32(self) -> SimdF32 {
        unsafe { vreinterpretq_f32_s32(self.data).into() }
    }

    #[inline(always)]
    pub fn to_f32(self) -> SimdF32 {
        unsafe { vcvtq_f32_s32(self.data).into() }
    }
}

impl From<int32x4_t> for SimdI32 {
    #[inline(always)]
    fn from(value: int32x4_t) -> Self {
        Self { data: value }
    }
}

impl From<[i32; 4]> for SimdI32 {
    #[inline(always)]
    fn from(value: [i32; 4]) -> Self {
        unsafe {
            Self {
                data: vld1q_s32(value.as_ptr()),
            }
        }
    }
}

impl From<SimdI32> for [i32; 4] {
    #[inline(always)]
    fn from(value: SimdI32) -> Self {
        value.to_array()
    }
}

impl From<i32> for SimdI32 {
    fn from(value: i32) -> Self {
        unsafe {
            Self {
                data: vdupq_n_s32(value),
            }
        }
    }
}
