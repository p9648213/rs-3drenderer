use crate::AppState;

pub fn draw_grid(color_buffer: &mut [u32], app: &mut AppState) {
    for y in (0..app.window_height).step_by(10) {
        for x in (0..app.window_width).step_by(10) {
            color_buffer[app.window_width * y + x] = 0xFF333333;
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

pub fn clear_color_buffer(color_buffer: &mut [u32], color: u32, app: &mut AppState) {
    for y in 0..app.window_height {
        for x in 0..app.window_width {
            color_buffer[app.window_width * y + x] = color;
        }
    }
}
