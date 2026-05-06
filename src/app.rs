use pixels::{Pixels, SurfaceTexture};
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::error::EventLoopError;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};

use crate::engine::Engine;
use crate::rgb::Rgba;

pub struct App<'a> {
    window: Option<Arc<Window>>,
    pixel_buffer: Option<Pixels<'a>>,
    width: usize,
    height: usize,
    tile_size: usize,
    render_scale: f32,
    render_width: usize,
    render_height: usize,
    engine: Engine, // scale_factor: u32,
                    // camera: Camera,
                    // raytracer : RayTracer,
                    // world : World,
                    // user_info : Info,
                    // cursor : Cursor,
                    //
}

impl App<'_> {
    pub fn new() -> Self {
        App {
            window: None,
            pixel_buffer: None,
            width: 0,
            height: 0,
            tile_size: 0,
            render_scale: 0.0,
            render_width: 0,
            render_height: 0,
            engine: Engine::new(surface_texture, render_width, render_height, render_scale),
        }
    }

    pub fn build_app(self) -> Self {
        //add assertions here to check app was constructed properly,
        //probably find a simpler way to do this too
        self
    }

    pub fn with_resolution(mut self, width: usize, height: usize) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn with_render_scale(mut self, scale: f32) -> Self {
        self.render_scale = scale;
        self.render_width = (self.width as f32 * scale) as usize;
        self.render_height = (self.height as f32 * scale) as usize;
        self
    }

    // pub fn build_camera(mut self, samples_per_pixel: i32, max_depth: i32) -> Self {
    //     self
    // }

    pub fn with_tile_size(mut self, size: usize) -> Self {
        self.tile_size = size;
        self
    }

    pub fn new_event_loop() -> Result<EventLoop<()>, EventLoopError> {
        let el = EventLoop::new()?;
        el.set_control_flow(ControlFlow::Poll);
        Ok(el)
    }

    fn resize_to(&mut self, size: PhysicalSize<u32>) {
        let width = size.width;
        let height = size.height;
        let px_ref = self.pixel_buffer.as_mut().unwrap();

        let render_width = (width as f32 * self.render_scale) as u32;
        let render_height = (height as f32 * self.render_scale) as u32;

        if let Err(e) = px_ref.resize_surface(width, height) {
            eprintln!("pixel surface resizing error: {e}");
        }

        if let Err(e) = px_ref.resize_buffer(render_width, render_height) {
            eprintln!("pixel buffer resizing error: {e}");
        }

        self.width = width as usize;
        self.height = height as usize;
    }

    fn shutdown(&mut self) {
        eprintln!("Window Closing");
    }

    fn update(&mut self) {}

    fn render(&mut self) {
        let ptr = PixelPointer(self.pixel_buffer.as_mut().unwrap().frame_mut().as_mut_ptr());
        debug_assert!(ptr.0 as usize % 4 == 0);

        let render_width = self.render_width;
        let render_height = self.render_height;
        let buffer_length = render_width * render_height * 4;

        let tile_count_x = render_width.div_ceil(self.tile_size);
        let tile_count_y = render_height.div_ceil(self.tile_size);

        let stride = render_width * 4;
        let offset = 4;

        (0..tile_count_x * tile_count_y)
            .into_par_iter()
            .with_min_len(16)
            .for_each(|i| {
                let tile_start_x = (i % tile_count_x) * self.tile_size;
                let tile_start_y = (i / tile_count_x) * self.tile_size;

                let index_start = (tile_start_x * offset) + (tile_start_y * stride);

                let tile_width = (render_width - tile_start_x).min(self.tile_size);
                let tile_height = (render_height - tile_start_y).min(self.tile_size);

                for pixel_offset_y in 0..tile_height {
                    let mut idx = ((pixel_offset_y * stride) + index_start) as usize;

                    for pixel_offset_x in 0..tile_width {
                        debug_assert!(
                            idx + 3 < buffer_length,
                            "Damn you fucked up your indexing math"
                        );

                        unsafe {
                            let pixel_x = tile_start_x + pixel_offset_x;
                            let pixel_y = tile_start_y + pixel_offset_y;

                            // let colors = camera.trace_pixel(&mut rng, world, pixel_x, pixel_y);
                            // *(ptr.add(idx + 0)) = colors.r;
                            // *(ptr.add(idx + 1)) = colors.g;
                            // *(ptr.add(idx + 2)) = colors.b;
                            // *(ptr.add(idx + 3)) = colors.a;

                            let colors = Rgba::new(
                                (255 * pixel_x / render_width) as u8,
                                0,
                                (255 * pixel_y / render_height) as u8,
                                255,
                            )
                            .compact();

                            *(ptr.add(idx) as *mut u32) = colors;
                        }

                        idx = idx + 4;
                    }
                }
            });

        self.pixel_buffer.as_mut().unwrap().render().unwrap();
    }
}

impl ApplicationHandler for App<'_> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let size = PhysicalSize::new(self.width as u32, self.height as u32);
        // let resize_inc = PhysicalSize::new(16, 16);

        let attributes = WindowAttributes::default()
            .with_title("CPU voxel ray-tracer test")
            .with_active(false)
            .with_inner_size(size)
            // .with_resize_increments(resize_inc)
            .with_resizable(true)
            .with_decorations(true)
            .with_visible(true);

        eprintln!("Opening Window");
        let window = Arc::new(event_loop.create_window(attributes).unwrap());

        eprintln!("Opening Pixel Buffer");
        let surface_texture = SurfaceTexture::new(size.width, size.height, window.clone());
        self.pixel_buffer = Some(Pixels::new(size.width, size.height, surface_texture).unwrap());

        self.window = Some(window);

        self.window.as_ref().unwrap().request_redraw();

        self.width = size.width as usize;
        self.height = size.height as usize;
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::Resized(new_size) => {
                self.resize_to(new_size);
            }
            WindowEvent::CloseRequested => {
                self.shutdown();
                event_loop.exit();
            }

            // WindowEvent::CursorMoved {
            //     device_id,
            //     position,
            // } => {}

            // WindowEvent::KeyboardInput {
            //     device_id,
            //     event,
            //     is_synthetic,
            // } => {}
            WindowEvent::RedrawRequested => {
                let frame_start = std::time::Instant::now();

                self.update();
                self.render();

                self.window.as_ref().unwrap().request_redraw();

                let elapsed = (std::time::Instant::now() - frame_start).as_millis();
                let fps = 1000.0 / elapsed as f32;
                eprintln!("Frame Time: {}ms \t FPS: {:.2}  ", elapsed, fps);
            }

            _ => (),
        }
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        // save and close engine
    }

    fn device_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        self.engine.device_event(event);
    }
}

//
//
//
//
//
//
//
//
//
//
// utility stuff, not needed outside this file so relegated to the pit

struct PixelPointer(*mut u8);
impl PixelPointer {
    unsafe fn add(&self, offset: usize) -> *mut u8 {
        unsafe { self.0.add(offset) }
    }
}

unsafe impl Send for PixelPointer {}
unsafe impl Sync for PixelPointer {}
