use std::cmp::max;

use crate::vec3::Vec3;

// USES A LEFT HANDED COORDINATE SYSTEM

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct Camera {
    pub center: Vec3, // camera origin
    pub front: Vec3,  // direction to screen center
    pub right: Vec3,  // orthogonal right
    pub up: Vec3,     // going out of top of head

    // with and height as offsets
    pub width_vector: Vec3,
    pub height_vector: Vec3,

    // center of lower left pixel
    pub pixel_00: Vec3,

    // space between pixels
    pub pixel_delta_x: Vec3,
    pub pixel_delta_y: Vec3,

    //width and height of the render resolution (not screen)
    pub render_width: usize,
    pub render_height: usize,

    //camera characteristics//
    pub aspect_ratio: f32,
    pub viewport_height: f32,
    pub viewport_width: f32,
    pub focal_length: f32,

    // stored as radians
    pub fovy: f32,
    pub pitch: f32,
    pub yaw: f32,
}

impl Camera {
    const WORLD_UP: Vec3 = Vec3::Y_POS_NORMAL;

    pub fn new(
        center: Vec3,
        render_width: usize,
        render_height: usize,
        fovy: f32,
        aspect_ratio: f32,
        pitch: f32,
        yaw: f32,
    ) -> Self {
        let mut cam = Self {
            center,
            render_width,
            render_height,
            fovy: fovy.to_radians(),
            aspect_ratio,
            focal_length: 1.0,
            pitch: pitch.to_radians(),
            yaw: yaw.to_radians(),
            ..Default::default()
        };

        cam.update(0.0, 0.0, 0.0, 0.0);

        return cam;
    }

    pub fn update(&mut self, x_delta: f32, y_delta: f32, sens: f32, dt: f32) {
        const UP_CLAMP: f32 = f32::to_radians(89.99);
        const DOWN_CLAMP: f32 = f32::to_radians(-89.99);

        let x = x_delta * sens * dt;
        let y = y_delta * sens * dt;
        self.pitch += y.max(UP_CLAMP).min(DOWN_CLAMP);
        self.yaw += x;

        self.front = Vec3::new(
            self.yaw.sin() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.cos() * self.pitch.cos(),
        );

        self.right = Camera::WORLD_UP.cross(self.front).normalize();
        self.up = self.front.cross(self.right).normalize();

        self.update_viewport();
    }

    pub fn look_at(&mut self, target: Vec3) {
        const UP_CLAMP: f32 = f32::to_radians(89.99);
        const DOWN_CLAMP: f32 = f32::to_radians(-89.99);

        self.front = (target - self.center).normalize();

        self.right = Camera::WORLD_UP.cross(self.front).normalize();

        self.up = self.front.cross(self.right).normalize();

        self.update_viewport();
    }

    fn update_viewport(&mut self) {
        let t = (self.fovy / 2.0).tan();
        self.viewport_height = 2.0 * t * self.focal_length;
        self.viewport_width = self.viewport_height * self.aspect_ratio;

        self.width_vector = self.viewport_width * self.right;
        self.height_vector = self.viewport_height * self.up;

        self.pixel_delta_x = self.width_vector / self.render_width as f32;
        self.pixel_delta_y = self.height_vector / self.render_height as f32;

        self.pixel_00 = self.center + (self.front * self.focal_length)
            // top left math
            - (self.width_vector * 0.5)
            + (self.height_vector * 0.5)
            + (self.pixel_delta_x * 0.5)
            - (self.pixel_delta_y * 0.5);
    }
}

fn normalize_coord(x: u32, y: u32, xmax: u32, ymax: u32) -> (f32, f32) {
    let nx = 1.0 - (x / xmax) as f32;
    let ny = 1.0 - (y / ymax) as f32;
    (nx, ny)
}
