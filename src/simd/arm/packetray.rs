use crate::{
    ray::Ray,
    simd::arm::{packetvec::Packed_Vec3_x4, simdf32::SimdF32, simdi32::SimdI32, vmask::VMask},
    vec3::Vec3,
};

#[repr(C)]
#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct Packet_Ray {
    pub origins: Packed_Vec3_x4,
    pub directions: Packed_Vec3_x4,
    pub inverse_dirs: Packed_Vec3_x4,
}

impl Packet_Ray {
    pub fn new(origins: [Vec3; 4], directions: [Vec3; 4]) -> Self {
        let origins_x: [f32; 4] = [origins[0].x, origins[1].x, origins[2].x, origins[3].x];
        let origins_y: [f32; 4] = [origins[0].y, origins[1].y, origins[2].y, origins[3].y];
        let origins_z: [f32; 4] = [origins[0].z, origins[1].z, origins[2].z, origins[3].z];

        let directions_x: [f32; 4] = [
            directions[0].x,
            directions[1].x,
            directions[2].x,
            directions[3].x,
        ];

        let directions_y: [f32; 4] = [
            directions[0].y,
            directions[1].y,
            directions[2].y,
            directions[3].y,
        ];

        let directions_z: [f32; 4] = [
            directions[0].z,
            directions[1].z,
            directions[2].z,
            directions[3].z,
        ];

        let inverses = [
            1.0 / directions[0],
            1.0 / directions[1],
            1.0 / directions[2],
            1.0 / directions[3],
        ];

        let inverses_x: [f32; 4] = [inverses[0].x, inverses[1].x, inverses[2].x, inverses[3].x];
        let inverses_y: [f32; 4] = [inverses[0].y, inverses[1].y, inverses[2].y, inverses[3].y];
        let inverses_z: [f32; 4] = [inverses[0].z, inverses[1].z, inverses[2].z, inverses[3].z];

        Self {
            origins: Packed_Vec3_x4::new(origins_x, origins_y, origins_z),
            directions: Packed_Vec3_x4::new(directions_x, directions_y, directions_z),
            inverse_dirs: Packed_Vec3_x4::new(inverses_x, inverses_y, inverses_z),
        }
    }
}
