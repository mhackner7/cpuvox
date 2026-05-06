use crate::{camera::Camera, input::InputHandler, vec3::Vec3, voxel::Material};
use winit::keyboard::SmolStr;

pub struct Player {
    pub name: SmolStr,
    pub position: Vec3,
    pub velocity: Vec3,
    pub move_speed: f32,
    pub hand: Material,
}

impl Player {}
