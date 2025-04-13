mod display;
mod mesh;
mod triangle;
mod vector;

use display::{clear_color_buffer, draw_grid, draw_rect, render_color_buffer};
use mesh::{MESH_FACES, MESH_VERTICES, N_MESH_FACES};
use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::PixelFormatEnum,
    render::{Canvas, Texture},
    video::Window,
    EventPump, Sdl,
};
use std::sync::OnceLock;
use triangle::Triangle;
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
    triangle_to_render: [Triangle; N_MESH_FACES],
}

static COLOR_BUFFER_SIZE: OnceLock<usize> = OnceLock::new();
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

fn update(app: &mut AppState) {
    let time_to_wait =
        FRAME_TARGET_TIME - (app.context.timer().unwrap().ticks() as i32 - app.previous_frame_time);

    if time_to_wait > 0 && time_to_wait <= FRAME_TARGET_TIME {
        app.context.timer().unwrap().delay(time_to_wait as u32);
    }

    app.previous_frame_time = app.context.timer().unwrap().ticks() as i32;

    app.cube_rotation.x += 0.01;
    app.cube_rotation.y += 0.01;
    app.cube_rotation.z += 0.01;

    for (i, mesh_face) in MESH_FACES.iter().enumerate() {
        let mut face_vertices = [Vec3::default(), Vec3::default(), Vec3::default()];

        face_vertices[0] = MESH_VERTICES[mesh_face.a as usize - 1];
        face_vertices[1] = MESH_VERTICES[mesh_face.b as usize - 1];
        face_vertices[2] = MESH_VERTICES[mesh_face.c as usize - 1];

        let mut projected_triangle = Triangle {
            point: [Vec2::default(), Vec2::default(), Vec2::default()],
        };

        for (j, _) in face_vertices.iter().enumerate() {
            let mut transformed_vertex = face_vertices[j];

            transformed_vertex = Vec3::rotate_x(transformed_vertex, app.cube_rotation.x);
            transformed_vertex = Vec3::rotate_y(transformed_vertex, app.cube_rotation.y);
            transformed_vertex = Vec3::rotate_z(transformed_vertex, app.cube_rotation.z);

            transformed_vertex.z -= app.camera_position.z;

            let mut projected_point = project(transformed_vertex);

            projected_point.x += app.window_width as f64 / 2.0;
            projected_point.y += app.window_height as f64 / 2.0;

            projected_triangle.point[j] = projected_point;
        }

        app.triangle_to_render[i] = projected_triangle;
    }
}

fn render(color_buffer: &mut [u32], app: &mut AppState) {
    draw_grid(color_buffer, app);

    for triangle in app.triangle_to_render.clone() {
        draw_rect(
            color_buffer,
            app,
            triangle.point[0].x as u32,
            triangle.point[0].y as u32,
            3,
            3,
            0xFFFFFF00,
        );
        draw_rect(
            color_buffer,
            app,
            triangle.point[1].x as u32,
            triangle.point[1].y as u32,
            3,
            3,
            0xFFFFFF00,
        );
        draw_rect(
            color_buffer,
            app,
            triangle.point[2].x as u32,
            triangle.point[2].y as u32,
            3,
            3,
            0xFFFFFF00,
        );
    }

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
        triangle_to_render: [
            Triangle {
                point: [Vec2::default(), Vec2::default(), Vec2::default()],
            },
            Triangle {
                point: [Vec2::default(), Vec2::default(), Vec2::default()],
            },
            Triangle {
                point: [Vec2::default(), Vec2::default(), Vec2::default()],
            },
            Triangle {
                point: [Vec2::default(), Vec2::default(), Vec2::default()],
            },
            Triangle {
                point: [Vec2::default(), Vec2::default(), Vec2::default()],
            },
            Triangle {
                point: [Vec2::default(), Vec2::default(), Vec2::default()],
            },
            Triangle {
                point: [Vec2::default(), Vec2::default(), Vec2::default()],
            },
            Triangle {
                point: [Vec2::default(), Vec2::default(), Vec2::default()],
            },
            Triangle {
                point: [Vec2::default(), Vec2::default(), Vec2::default()],
            },
            Triangle {
                point: [Vec2::default(), Vec2::default(), Vec2::default()],
            },
            Triangle {
                point: [Vec2::default(), Vec2::default(), Vec2::default()],
            },
            Triangle {
                point: [Vec2::default(), Vec2::default(), Vec2::default()],
            },
        ],
    };

    let max_size = *COLOR_BUFFER_SIZE.get_or_init(|| window_height * window_width);

    let mut color_buffer = vec![0x00000000; max_size].into_boxed_slice();

    while app.is_running {
        process_input(&mut app);
        update(&mut app);
        render(&mut color_buffer, &mut app);
    }

    Ok(())
}
