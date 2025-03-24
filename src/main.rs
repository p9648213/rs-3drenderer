use std::sync::OnceLock;

use sdl2::{
    EventPump,
    event::Event,
    keyboard::Keycode,
    pixels::{Color, PixelFormatEnum},
    render::{Canvas, Texture},
    video::Window,
};

struct AppState<'a> {
    canvas: Canvas<Window>,
    is_running: bool,
    color_buffer_texture: Texture<'a>,
    window_width: usize,
    window_height: usize,
    event_pump: EventPump,
}

static COLOR_BUFFER_SIZE: OnceLock<usize> = OnceLock::new();

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

fn update() {}

fn render(color_buffer: &mut [u32], app: &mut AppState) {
    app.canvas.set_draw_color(Color::RGB(255, 0, 0));
    app.canvas.clear();

    app.color_buffer_texture
        .update(
            None,
            bytemuck::cast_slice(color_buffer),
            app.window_width * size_of::<u32>(),
        )
        .expect("Failed to update color buffer texture");

    app.canvas
        .copy(&app.color_buffer_texture, None, None)
        .expect("Filed to copy color buffer texture");

    clear_color_buffer(color_buffer, 0xFFFFFF00, app);

    app.canvas.present();
}

fn clear_color_buffer(color_buffer: &mut [u32], color: u32, app: &mut AppState) {
    for y in 0..app.window_height {
        for x in 0..app.window_width {
            color_buffer[app.window_width * y + x] = color;
        }
    }
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
        canvas,
        event_pump,
        is_running,
        color_buffer_texture,
        window_height,
        window_width,
    };

    let max_size = *COLOR_BUFFER_SIZE.get_or_init(|| window_height * window_width);

    let mut color_buffer = vec![0xFFFFFF00; max_size].into_boxed_slice();

    while app.is_running {
        process_input(&mut app);
        update();
        render(&mut color_buffer, &mut app);
    }

    Ok(())
}
