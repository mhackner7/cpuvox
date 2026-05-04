use crate::ivec3::IVec3;
use crate::ray::*;
use crate::traversable::*;
use crate::vec3::Vec3;

type Material = u8;

#[derive(Debug, Copy, Clone)]
pub struct Brick {
    pub voxels: [Material; Brick::BRICK_SIZE],
}

impl Brick {
    pub const BRICK_WIDTH: usize = 8;
    pub const BRICK_SIZE: usize = Self::BRICK_WIDTH * Self::BRICK_WIDTH * Self::BRICK_WIDTH;

    pub fn traverse(&self, rayi: RayI, entry_pos: Vec3, entry_distance: f32) -> HitInfo {
        const WIDTH: i32 = Brick::BRICK_WIDTH as i32;
        const MAX_DEPTH: usize = Brick::BRICK_WIDTH * 3;

        let step: IVec3 = rayi.direction.signs().into();

        let delta = rayi.inv_dir.abs();

        let mut distance_to_next = Vec3::tmax_from(step, entry_pos, rayi.inv_dir);

        let mut position = IVec3::clamp_to(IVec3::from(entry_pos), 0, WIDTH - 1);

        let mut axis = entry_pos.max_axis();

        for _ in 0..MAX_DEPTH {
            let flat_index: usize =
                (position.x + (position.y * WIDTH) + (position.z * WIDTH * WIDTH)) as usize;

            let voxel = self.voxels[flat_index];

            if voxel != 0 {
                return HitInfo::new(
                    Vec3::get_face_normal(axis, -step[axis]),
                    entry_distance + distance_to_next[axis] - delta[axis],
                    voxel,
                    true,
                );
            }

            axis = distance_to_next.min_axis();

            position[axis] += step[axis];

            if position[axis] < 0 || WIDTH <= position[axis] {
                return HitInfo::exit_volume_at(
                    entry_distance + distance_to_next[axis] - delta[axis],
                );
            }

            distance_to_next[axis] += delta[axis];
        }

        return HitInfo::DUD;
    }
}
