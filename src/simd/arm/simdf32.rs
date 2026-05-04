use std::arch::aarch64::*;

use crate::simd::arm::{simdi32::SimdI32, vmask::VMask};

use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

#[repr(C)]
#[repr(align(16))] // just in case manually align to 16 bytes (or 128 bits)
#[derive(Debug, Clone, Copy)]
pub struct SimdF32 {
    pub data: float32x4_t,
}

impl SimdF32 {
    #[inline(always)]
    pub fn to_array(self) -> [f32; 4] {
        unsafe {
            let mut a = [0.0f32; 4];
            vst1q_f32(a.as_mut_ptr(), self.data);
            return a;
        }
    }

    #[inline(always)]
    pub fn mul_add(add: Self, mul1: Self, mul2: Self) -> Self {
        // a + (b * c) -> add + (mul1 * mul2)
        unsafe { vfmaq_f32(add.data, mul1.data, mul2.data).into() }
    }

    #[inline(always)]
    pub fn add_to_mul(self, mul1: Self, mul2: Self) -> Self {
        // a + (b * c) -> self + (mul1 * mul2)
        unsafe { vfmaq_f32(self.data, mul1.data, mul2.data).into() }
    }

    #[inline(always)]
    pub fn mul_then_add(self, mul2: Self, add: Self) -> Self {
        // a + (b * c) -> add + (self * mul2)
        unsafe { vfmaq_f32(add.data, self.data, mul2.data).into() }
    }

    #[inline(always)]
    pub fn add(self, other: Self) -> Self {
        unsafe { vaddq_f32(self.data, other.data).into() }
    }
    #[inline(always)]
    pub fn mul(self, other: Self) -> Self {
        unsafe { vmulq_f32(self.data, other.data).into() }
    }
    #[inline(always)]
    pub fn sub(self, other: Self) -> Self {
        unsafe { vsubq_f32(self.data, other.data).into() }
    }

    #[inline(always)]
    pub fn recip(self) -> Self {
        unsafe { vrecpeq_f32(self.data).into() }
    }

    #[inline(always)]
    pub fn min(self, other: Self) -> Self {
        unsafe { vminq_f32(self.data, other.data).into() }
    }

    #[inline(always)]
    pub fn max(self, other: Self) -> Self {
        unsafe { vmaxq_f32(self.data, other.data).into() }
    }

    #[inline(always)]
    pub fn cmp_blend(condition_mask: VMask, if_true: Self, if_false: Self) -> Self {
        unsafe { vbslq_f32(condition_mask.data, if_true.data, if_false.data).into() }
    }

    #[inline(always)]
    pub fn is_equal_to(self, other: Self) -> VMask {
        unsafe { vceqq_f32(self.data, other.data).into() }
    }

    #[inline(always)]
    pub fn is_less_than(self, other: Self) -> VMask {
        unsafe { vcltq_f32(self.data, other.data).into() }
    }

    #[inline(always)]
    pub fn is_greater_than(self, other: Self) -> VMask {
        unsafe { vcgtq_f32(self.data, other.data).into() }
    }

    #[inline(always)]
    pub fn is_less_or_equal(self, other: Self) -> VMask {
        unsafe { vcleq_f32(self.data, other.data).into() }
    }

    #[inline(always)]
    pub fn is_greater_or_equal(self, other: Self) -> VMask {
        unsafe { vcgeq_f32(self.data, other.data).into() }
    }

    #[inline(always)]
    pub fn floor(self) -> SimdI32 {
        unsafe { vcvtq_s32_f32(self.data).into() }
    }

    #[inline(always)]
    pub fn reinterpret_as_i32(self) -> SimdI32 {
        unsafe { vreinterpretq_s32_f32(self.data).into() }
    }

    #[inline(always)]
    pub fn to_i32(self) -> SimdI32 {
        unsafe { vcvtq_s32_f32(self.data).into() }
    }

    #[inline(always)]
    pub fn abs(self) -> Self {
        unsafe { vabsq_f32(self.data).into() }
    }

    // #[inline(always)]
    //     pub fn

    // #[inline(always)]
    //     pub fn

    // #[inline(always)]
    //     pub fn

    // #[inline(always)]
    //     pub fn

    // #[inline(always)]
    //     pub fn

    // #[inline(always)]
    //     pub fn

    // #[inline(always)]
    //     pub fn

    // #[inline(always)]
    //     pub fn

    // #[inline(always)]
    //     pub fn

    // #[inline(always)]
    //     pub fn
}

impl From<[f32; 4]> for SimdF32 {
    #[inline(always)]
    fn from(value: [f32; 4]) -> Self {
        unsafe {
            SimdF32 {
                data: vld1q_f32(value.as_ptr()),
            }
        }
    }
}

impl From<SimdF32> for [f32; 4] {
    #[inline(always)]
    fn from(value: SimdF32) -> Self {
        value.to_array()
    }
}

impl From<f32> for SimdF32 {
    #[inline(always)]
    fn from(value: f32) -> Self {
        unsafe {
            SimdF32 {
                data: vdupq_n_f32(value),
            }
        }
    }
}

impl From<float32x4_t> for SimdF32 {
    #[inline(always)]
    fn from(value: float32x4_t) -> Self {
        SimdF32 { data: value }
    }
}

impl Add<Self> for SimdF32 {
    type Output = SimdF32;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        unsafe { vaddq_f32(self.data, rhs.data).into() }
    }
}

impl Sub<SimdF32> for SimdF32 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: SimdF32) -> Self::Output {
        unsafe { vsubq_f32(self.data, rhs.data).into() }
    }
}

impl Mul<SimdF32> for SimdF32 {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: SimdF32) -> Self::Output {
        unsafe { vmulq_f32(self.data, rhs.data).into() }
    }
}
