use crate::{interval::Interval, ivec3::IVec3, ray::Ray, rgb::Rgba, vec3::*};

pub type Material = u8;
const TOTAL_VOXELS: usize = 10;

pub struct Voxel {
    pub material: Material,
}

#[derive(Debug, Copy, Clone, Default)]
pub struct VoxelInfo {
    pub dimensions: Vec3,
    pub color: Rgba,
    pub material: Material,
    pub opacity: u8,    // 0-10 value -> increments of 10%
    pub durability: u8, // 0-10 value -> increments of 10%
    pub viscosity: u8,  // 0-3 value, 0 = solid, 3 = water
    pub gravitates: bool,
    pub emits_light: bool,
    pub is_electric: bool,
    pub is_interactive: bool,
    pub is_intangible: bool,
}

impl VoxelInfo {
    const DEFAULT: VoxelInfo = VoxelInfo {
        dimensions: Vec3::DEFAULT,
        color: Rgba::new(0, 0, 0, 0),
        material: 0,
        opacity: 0,
        durability: 0,
        viscosity: 0,
        gravitates: false,
        emits_light: false,
        is_electric: false,
        is_interactive: false,
        is_intangible: false,
    };

    pub const ARRAY: [VoxelInfo; TOTAL_VOXELS] = VoxelInfo::init();

    const fn init() -> [VoxelInfo; TOTAL_VOXELS] {
        [VoxelInfo::DEFAULT; TOTAL_VOXELS]
    }

    #[inline(always)]
    pub const fn is_transparent(self) -> bool {
        self.opacity < 10
    }
    #[inline(always)]
    pub const fn opacity_float(self) -> f32 {
        100.0 / self.opacity as f32 * 10.0
    }

    #[inline(always)]
    pub const fn is_liquid(self) -> bool {
        self.viscosity == 0
    }
}
