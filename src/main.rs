use sdl2::{
    EventPump, event::Event, keyboard::Keycode, pixels::Color, render::Canvas, video::Window,
};

fn setup() {}

fn process_input(event_pump: &mut EventPump, is_running: &mut bool) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => {
                *is_running = false;
            }
            _ => {}
        }
    }
}

fn update() {}

fn render(canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.clear();
    canvas.present();
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("", 800, 600)
        .position_centered()
        .borderless()
        .build()
        .unwrap();

    let mut is_running = true;
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    setup();

    while is_running {
        process_input(&mut event_pump, &mut is_running);
        update();
        render(&mut canvas);
    }
}
