use crate::AppState;

pub fn draw_pixel(x: usize, y: usize, color: u32, app: &AppState, color_buffer: &mut [u32]) {
    if x < app.window_width && y < app.window_height {
        color_buffer[(app.window_width * y) + x] = color;
    }
}

pub fn draw_grid(color_buffer: &mut [u32], app: &mut AppState) {
    for y in (0..app.window_height).step_by(10) {
        for x in (0..app.window_width).step_by(10) {
            draw_pixel(x, y, 0xFF333333, app, color_buffer);
        }
    }
}

pub fn draw_rect(
    color_buffer: &mut [u32],
    app: &mut AppState,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    color: u32,
) {
    for i in y..(y + height) {
        for j in x..(x + width) {
            color_buffer[(app.window_width * i as usize) + j as usize] = color;
        }
    }
}

pub fn render_color_buffer(color_buffer: &mut [u32], app: &mut AppState) {
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
}

pub fn clear_color_buffer(color_buffer: &mut [u32], color: u32, app: &mut AppState) {
    for y in 0..app.window_height {
        for x in 0..app.window_width {
            color_buffer[app.window_width * y + x] = color;
        }
    }
}
