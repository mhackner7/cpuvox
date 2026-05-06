use cpu_vox::app::App;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // camera setup
    let samples_per_pixel = 40;
    let max_depth = 25;

    let mut app = App::new()
        .with_resolution(1280, 800)
        .with_tile_size(32)
        .with_render_scale(0.5)
        // .build_camera(samples_per_pixel, max_depth)
        .build_app();

    let event_loop = App::new_event_loop()?.set_control_flow(winit::event_loop::ControlFlow::Poll);
    event_loop.run_app(&mut app)?;

    Ok(())
}
