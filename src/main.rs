mod display;
mod vector;

use display::{clear_color_buffer, draw_grid, draw_rect, render_color_buffer};
use sdl2::{
    EventPump, Sdl,
    event::Event,
    keyboard::Keycode,
    pixels::PixelFormatEnum,
    render::{Canvas, Texture},
    video::Window,
};
use std::sync::OnceLock;
use vector::{Vec2, Vec3};

struct AppState<'a> {
    context: Sdl,
    canvas: Canvas<Window>,
    is_running: bool,
    color_buffer_texture: Texture<'a>,
    window_width: usize,
    window_height: usize,
    event_pump: EventPump,
    camera_position: Vec3,
    cube_rotation: Vec3,
    previous_frame_time: i32,
}

static COLOR_BUFFER_SIZE: OnceLock<usize> = OnceLock::new();
const N_POINTS: usize = 9 * 9 * 9;
const FOV_FACTOR: f64 = 640.0;
const FPS: i32 = 30;
const FRAME_TARGET_TIME: i32 = 1000 / FPS;

fn process_input(app: &mut AppState) {
    for event in app.event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => {
                app.is_running = false;
            }
            _ => {}
        }
    }
}

fn project(point: Vec3) -> Vec2 {
    Vec2 {
        x: FOV_FACTOR * point.x / point.z,
        y: FOV_FACTOR * point.y / point.z,
    }
}

fn update(
    cube_points: &mut [Vec3; N_POINTS],
    projected_points: &mut [Vec2; N_POINTS],
    app: &mut AppState,
) {
    let time_to_wait =
        FRAME_TARGET_TIME - (app.context.timer().unwrap().ticks() as i32 - app.previous_frame_time);

    if time_to_wait > 0 && time_to_wait <= FRAME_TARGET_TIME {
        app.context.timer().unwrap().delay(time_to_wait as u32);
    }

    app.cube_rotation.x += 0.01;
    app.cube_rotation.y += 0.01;
    app.cube_rotation.z += 0.01;

    for i in 0..N_POINTS {
        let mut transformed_point = Vec3::rotate_x(cube_points[i], app.cube_rotation.x);
        transformed_point = Vec3::rotate_y(transformed_point, app.cube_rotation.y);
        transformed_point = Vec3::rotate_z(transformed_point, app.cube_rotation.z);

        transformed_point.z -= app.camera_position.z;

        projected_points[i] = project(transformed_point);
    }
}

fn render(color_buffer: &mut [u32], app: &mut AppState, projected_point: &[Vec2; N_POINTS]) {
    draw_grid(color_buffer, app);

    (0..N_POINTS).for_each(|i| {
        draw_rect(
            color_buffer,
            app,
            (projected_point[i].x + (app.window_width as f64 / 2.0)) as u32,
            (projected_point[i].y + (app.window_height as f64 / 2.0)) as u32,
            4,
            4,
            0xFFFFFF00,
        );
    });

    render_color_buffer(color_buffer, app);
    clear_color_buffer(color_buffer, 0xFF000000, app);

    app.canvas.present();
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let display_mode = video_subsystem.display_mode(0, 0).unwrap();

    let window_width = display_mode.w as usize;
    let window_height = display_mode.h as usize;

    let window = video_subsystem
        .window("", display_mode.w as u32, display_mode.h as u32)
        .position_centered()
        .borderless()
        .fullscreen()
        .build()
        .unwrap();

    let is_running = true;
    let canvas = window.into_canvas().build().unwrap();
    let texture_creature = canvas.texture_creator();
    let event_pump = sdl_context.event_pump().unwrap();

    let color_buffer_texture = texture_creature
        .create_texture_streaming(
            PixelFormatEnum::ARGB8888,
            display_mode.w as u32,
            display_mode.h as u32,
        )
        .map_err(|e| e.to_string())?;

    let mut cube_points: [Vec3; N_POINTS] = [Vec3::default(); N_POINTS];
    let mut projected_points: [Vec2; N_POINTS] = [Vec2::default(); N_POINTS];
    let mut point_count = 0;

    let mut x = -1.0;
    while x <= 1.0 {
        let mut y = -1.0;
        while y <= 1.0 {
            let mut z = -1.0;
            while z <= 1.0 {
                cube_points[point_count] = Vec3 { x, y, z };
                point_count += 1;
                z += 0.25;
            }
            y += 0.25;
        }
        x += 0.25;
    }

    let mut app = AppState {
        context: sdl_context,
        canvas,
        event_pump,
        is_running,
        color_buffer_texture,
        window_height,
        window_width,
        camera_position: Vec3 {
            x: 0.0,
            y: 0.0,
            z: -5.0,
        },
        cube_rotation: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        previous_frame_time: 0,
    };

    let max_size = *COLOR_BUFFER_SIZE.get_or_init(|| window_height * window_width);

    let mut color_buffer = vec![0x00000000; max_size].into_boxed_slice();

    while app.is_running {
        process_input(&mut app);
        update(&mut cube_points, &mut projected_points, &mut app);
        render(&mut color_buffer, &mut app, &projected_points);
    }

    Ok(())
}
