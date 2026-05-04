use crate::brick::Brick;
use crate::ivec3::IVec3;
use crate::mask::Mask;
use crate::ray::*;
use crate::traversable::*;
use crate::vec3::Vec3;

type Material = u8;

pub struct MultiGrid {
    bricks: Vec<Brick>,
    mask: Mask,
    start_xyz: IVec3,
}

/*
    2 Level Heirarchical Grid

    contains:
        base 4x4x4 occupancy grid (64bit mask)
        Vector of Bricks (maximum of 64)

    bitmask shows occupancy for a given space where a brick exists, which is stored in the array

*/
impl MultiGrid {
    pub const CHUNK_VOXEL_WIDTH: usize = MultiGrid::BRICKS_PER_AXIS * Brick::BRICK_WIDTH;
    pub const BRICKS_PER_AXIS: usize = 4;

    pub const GRID_DIMENSION: IVec3 = IVec3::new(
        MultiGrid::CHUNK_VOXEL_WIDTH as i32,
        MultiGrid::CHUNK_VOXEL_WIDTH as i32,
        MultiGrid::CHUNK_VOXEL_WIDTH as i32,
    );
    pub const GRID_DIMENSION_F32: Vec3 = Vec3::new(
        MultiGrid::CHUNK_VOXEL_WIDTH as f32,
        MultiGrid::CHUNK_VOXEL_WIDTH as f32,
        MultiGrid::CHUNK_VOXEL_WIDTH as f32,
    );

    pub fn trace_ray(grid: &MultiGrid, ray: Ray) -> Material {
        let rayi = RayI::from(ray);

        let distance = aabb(
            rayi.origin,
            rayi.inv_dir,
            grid.start_xyz.into(),
            Vec3::from(grid.start_xyz) + MultiGrid::GRID_DIMENSION_F32,
        );

        if distance == f32::INFINITY {
            return 0;
        }

        let hit = grid.traverse(rayi, distance);

        if hit.is_valid {
            return hit.material;
        }

        return HitInfo::DUD.material;
    }

    fn traverse(&self, rayi: RayI, entry_distance: f32) -> HitInfo {
        const MAX_STEPS: usize = MultiGrid::BRICKS_PER_AXIS * 3;
        const GRID_WIDTH: i32 = MultiGrid::BRICKS_PER_AXIS as i32;
        const BRICK_WIDTH: f32 = Brick::BRICK_WIDTH as f32;

        let starting_coords = Vec3::from(self.start_xyz);

        let step: IVec3 = rayi.direction.signs().into();

        let delta = rayi.inv_dir.abs() * BRICK_WIDTH;

        let entry_pos_in_voxels = rayi.at(entry_distance + 0.0001) - starting_coords;
        let entry_pos_in_bricks = entry_pos_in_voxels / BRICK_WIDTH;

        //position in terms of brick space (x|y|z = 0..4)
        let mut position = IVec3::from(entry_pos_in_bricks).clamp_to(0, GRID_WIDTH - 1);

        let mut distance_to_next =
            Vec3::tmax_from(step, entry_pos_in_bricks, rayi.inv_dir * BRICK_WIDTH);

        let mut current_distance = entry_distance;

        /*

        count the trailing zeroes for the mask, perform accelerated dda for that many times
        do first or do in the inner loop?

        at the start:
        for _ in 0..mask.trailing_zeroes()

        bad idea



        */

        for _ in 0..MAX_STEPS {
            let index =
                Mask::lin_xyz(position.x as u8, position.y as u8, position.z as u8) as usize;

            let mi = Mask::LIN_XYZ_TO_M[index];

            if self.mask.is_active_at(mi) {
                let brick_pool_index = self.mask.vec_index(mi);

                let brick = &self.bricks[brick_pool_index];

                let chunk_local_position = rayi.at(current_distance + 0.0001) - starting_coords;

                let brick_entry_position =
                    chunk_local_position - (Vec3::from(position) * BRICK_WIDTH);

                let brick_hit_info = brick.traverse(rayi, brick_entry_position, current_distance);

                if brick_hit_info.is_valid {
                    return brick_hit_info;
                }
            }

            let axis = distance_to_next.min_axis();

            current_distance = distance_to_next[axis];

            let maski = IVec3::axis_mask(axis);
            let maskf = Vec3::axis_mask(axis);

            position += step * maski;

            if position[axis] < 0 || position[axis] >= GRID_WIDTH {
                return HitInfo::exit_volume_at(distance_to_next[axis]);
            }

            distance_to_next += delta * maskf;
        }

        return HitInfo::DUD;
    }

    // pub fn serialize(&self) -> Vec<u8> {
    //     let mut vec: Vec<u8> = Vec::with_capacity(std::mem::size_of_val(&self));

    //     let coords = self.start_xyz.serialize();

    //     return vec;
    // }
}
