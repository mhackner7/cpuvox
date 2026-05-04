#[derive(Copy, Clone, Debug, Default)]
pub struct Mask {
    bits: u64,
}

impl Mask {
    pub const XYZ_FROM_MORTON: [(u8, u8, u8); 64] = Mask::arr_m_to_xyz();

    const fn arr_m_to_xyz() -> [(u8, u8, u8); 64] {
        let mut arr = [(0, 0, 0); 64];
        let mut i = 0;
        while i < 64 {
            arr[i] = Mask::index_from_morton(i as u8);
            i = i + 1;
        }
        arr
    }

    pub const LIN_XYZ_TO_M: [u8; 64] = Mask::arr_linxyz_to_m();

    const fn arr_linxyz_to_m() -> [u8; 64] {
        let mut arr = [0; 64];
        let mut z = 0;
        while z < 4 {
            let mut y = 0;

            while y < 4 {
                let mut x = 0;

                while x < 4 {
                    let lin = Mask::lin_xyz(x, y, z) as usize;
                    arr[lin] = Mask::morton_from_index(x, y, z);
                    x += 1
                }

                y += 1
            }

            z += 1
        }

        arr
    }

    #[inline(always)]
    pub const fn lin_xyz(x: u8, y: u8, z: u8) -> u8 {
        (z << 4) | (y << 2) | x
    }

    #[inline(always)]
    pub const fn morton_from_index(x: u8, y: u8, z: u8) -> u8 {
        debug_assert!(
            x < 4,
            "x should be one of 0 | 1 | 2 | 3, its a 0-indexed 4x4x4 volume"
        );
        debug_assert!(
            y < 4,
            "y should be one of 0 | 1 | 2 | 3, its a 0-indexed 4x4x4 volume"
        );
        debug_assert!(
            z < 4,
            "z should be one of 0 | 1 | 2 | 3, its a 0-indexed 4x4x4 volume"
        );

        /*
            Morton Order from xyz
            2 bits per coord, 4x4x4 volume
            zyx.1 zyx.0
            only 6 bits are filled to a maximum of 64 possible combinations
            bit [1] is shifted by s - 1, due to already being one bit higher
            bit [0] shifted normally
        */

        ((z & 0b10u8) << 4)
            | ((y & 0b10u8) << 3)
            | ((x & 0b10u8) << 2)
            | ((z & 0b1u8) << 2)
            | ((y & 0b1u8) << 1)
            | (x & 0b1u8)
    }
    #[inline(always)]
    pub const fn index_from_morton(mi: u8) -> (u8, u8, u8) {
        debug_assert!(mi < 64, "invalid morton index");
        /*
            bits [3] and [0] -> x
            bits [4] and [1] -> y
            bits [5] and [2] -> z
        */
        let x = ((mi >> 2) & 0b10u8) | (mi & 0b1u8);
        let y = ((mi >> 3) & 0b10u8) | (mi >> 1) & 0b1u8;
        let z = ((mi >> 4) & 0b10u8) | (mi >> 2) & 0b1u8;

        (x, y, z)
    }
    #[inline(always)]
    pub fn set_bit(mask: Mask, index: u8) -> Mask {
        Mask {
            bits: mask.bits | 0b1u64 << index,
        }
    }
    pub fn clear_bit(mask: Mask, index: u8) -> Mask {
        Mask {
            bits: mask.bits & !(0b1u64 << index),
        }
    }
    #[inline(always)]
    pub fn active_count(self) -> u32 {
        self.bits.count_ones()
    }

    #[inline(always)]
    pub fn vec_index(self, mi: u8) -> usize {
        /*
            example: 1001 0001
            7th, 4th and 0th active
            0 -> 0th
            4 -> 4th
            >>0, popcount -> 3, -1 -> 2, maps to Vec[2]
            >>4, popcount -> 2, -1 -> 1, maps to Vec[1]
        */

        (self.bits >> mi).count_ones() as usize - 1
    }

    #[inline(always)]
    pub fn is_active_at(self, mi: u8) -> bool {
        (self.bits >> mi) & 0b1u64 == 1
    }
}
