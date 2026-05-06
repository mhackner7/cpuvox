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

use crate::{
    camera::Camera,
    input::{self, *},
    multigrid::MultiGrid,
    player::Player,
    world::World,
};
use pixels::{Pixels, SurfaceTexture, wgpu::Device};
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use std::{default, sync::Arc};
use winit::{
    event::{self, DeviceEvent, Event},
    keyboard::PhysicalKey,
    window::Window,
};

pub struct Engine {
    pub pixels: Pixels<'static>,
    pub render_distance: f64, // in terms of voxels NOT chunks
    pub render_resolution: (u32, u32),
    pub render_scale: f64, // percentage

    // pub world: World,

    // pub player: Player,
    pub camera: Camera,
    pub input_handler: InputHandler,
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
            input_handler: Default::default(),
        }
    }

    pub fn update(&mut self) {}

    pub fn tick(&mut self) {}

    pub fn shutdown(&mut self) {}

    pub fn save(&mut self) {}

    pub fn load(&mut self) {}

    pub fn resize_buffer(&mut self, new_x: usize, new_y: usize) {}

    pub fn render(&mut self) {
        self.camera.update();
        let pixel_00 = self.camera.pixel_00;
    }

    pub fn run_input_queue(&mut self) {}

    pub fn device_event(&mut self, event: DeviceEvent) {
        match event {
            DeviceEvent::MouseMotion { delta } => {
                self.input_handler.mouse_motion(&mut self.camera, delta);
            }
            DeviceEvent::MouseWheel { delta } => {
                self.input_handler.mouse_wheel(delta);
            }
            DeviceEvent::Key(key) => {
                if let PhysicalKey::Code(k) = key.physical_key {
                    let k = PressedInput::Key(k);
                    let ctrl = self.input_handler.keymap.find(k);
                    if ctrl != Control::Invalid {
                        self.input_handler.keyqueue.push_back((ctrl, key.state));
                    }
                }
            }
            _ => {}
        }
    }
}
