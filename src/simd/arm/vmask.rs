use std::arch::aarch64::*;

pub struct VMask {
    pub data: uint32x4_t,
}

impl VMask {
    #[inline(always)]
    pub fn to_array(self) -> [u32; 4] {
        unsafe {
            let mut a = [0u32; 4];
            vst1q_u32(a.as_mut_ptr(), self.data);
            return a;
        }
    }
}

impl From<uint32x4_t> for VMask {
    #[inline(always)]
    fn from(value: uint32x4_t) -> Self {
        VMask { data: value }
    }
}

impl From<[u32; 4]> for VMask {
    #[inline(always)]
    fn from(value: [u32; 4]) -> Self {
        unsafe {
            Self {
                data: vld1q_u32(value.as_ptr()),
            }
        }
    }
}

impl From<VMask> for [u32; 4] {
    #[inline(always)]
    fn from(value: VMask) -> Self {
        value.to_array()
    }
}

impl From<u32> for VMask {
    fn from(value: u32) -> Self {
        unsafe {
            Self {
                data: vdupq_n_u32(value),
            }
        }
    }
}
