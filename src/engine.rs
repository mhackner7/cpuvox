// TODO
// 1. move all core engine stuff out of app.rs
//
// 2. setup basic chunk
// 3. get camera and input working
// 4. move closer to real demo
//
// further along:
// 1. get world setup
// 2. get terrain generation setup
//

use crate::{camera::Camera, input::Input, multigrid::MultiGrid, player::Player, world::World};
use pixels::{Pixels, SurfaceTexture};
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use std::{default, sync::Arc};
use winit::window::Window;

pub struct Engine {
    pub pixels: Pixels<'static>,
    pub render_distance: f64, // in terms of voxels NOT chunks
    pub render_resolution: (u32, u32),
    pub render_scale: f64, // percentage

    // pub world: World,

    // pub player: Player,
    pub camera: Camera,
    pub input: Input,
}

impl Engine {
    pub fn new(
        surface_texture: SurfaceTexture<Window>,
        render_width: u32,
        render_height: u32,
        render_scale: f64,
    ) -> Self {
        Self {
            pixels: Pixels::new(render_width, render_height, surface_texture).unwrap(),
            render_distance: MultiGrid::BRICKS_PER_AXIS as f64 * 4.0,
            render_resolution: (render_width, render_height),
            render_scale,
            // world: (),
            // player: (),
            camera: Default::default(),
            input: Default::default(),
        }
    }

    pub fn update(&mut self) {}

    pub fn tick(&mut self) {}

    pub fn shutdown(&mut self) {}

    pub fn save(&mut self) {}

    pub fn load(&mut self) {}

    pub fn resize_buffer(&mut self, new_x: usize, new_y: usize) {}

    pub fn render(&mut self) {
        let pixel_00 = self.camera.pixel_00;
    }
}
