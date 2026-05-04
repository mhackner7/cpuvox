use crate::interval::Interval;
use crate::ivec3::IVec3;
use crate::ray::{Ray, RayI};
use crate::vec3::Vec3;

type Material = u8;

pub struct HitInfo {
    pub normal: Vec3,
    pub distance: f32,
    pub material: Material,
    pub is_valid: bool,
}

impl HitInfo {
    pub const DUD: HitInfo = HitInfo::new(Vec3::DEFAULT, 1e30, 0, false);

    #[inline(always)]
    pub const fn new(normal: Vec3, distance: f32, material: Material, is_valid: bool) -> HitInfo {
        HitInfo {
            normal,
            distance,
            material,
            is_valid,
        }
    }

    #[inline(always)]
    pub const fn exit_volume_at(distance: f32) -> HitInfo {
        HitInfo {
            distance,
            ..HitInfo::DUD
        }
    }
}

/*
    ro - ray origin
    id - inverse ray direction
    bmin - box starting coord
    bmax - box ending coord
    (bottom corner to opposite top corner)
*/
#[inline(always)]
pub fn aabb(ro: Vec3, id: Vec3, bmin: Vec3, bmax: Vec3) -> f32 {
    let x1 = (bmin.x - ro.x) * id.x;
    let x2 = (bmax.x - ro.x) * id.x;

    let mut tmin = f32::min(x1, x2);
    let mut tmax = f32::max(x1, x2);

    let y1 = (bmin.y - ro.y) * id.y;
    let y2 = (bmax.y - ro.y) * id.y;

    tmin = f32::max(tmin, y1.min(y2));
    tmax = f32::min(tmax, y1.max(y2));

    let z1 = (bmin.z - ro.z) * id.z;
    let z2 = (bmax.z - ro.z) * id.z;

    tmin = f32::max(tmin, z1.min(z2));
    tmax = f32::min(tmax, z1.max(z2));

    tmin = tmin.max(0.0);

    if tmax >= tmin {
        tmin;
    } else {
        return f32::INFINITY;
    }
}
