use crate::config::*;
use piston_window::*;
use rectangle::square;

pub fn get_windows() -> PistonWindow {
    WindowSettings::new("Chip8-emulator", [WINDOW_WIDTH, WINDOW_HEIGHT])
        .exit_on_esc(true)
        .build()
        .expect("Cannot open a window")
}

pub fn draw_screen(window: &mut PistonWindow, event: Event, image: [u8; VIDEO_MEMORY_SIZE]) {
    window.draw_2d(&event, |ctx, renderer, _device| {
        clear([1.0, 1.0, 1.0, 1.0], renderer);
        for (i, pixel) in image.iter().enumerate() {
            let (x, y) = get_window_coordinate(i);
            if should_draw_pixel(*pixel) {
                draw_pixel(x, y, ctx, renderer);
            }
        }
    });
}

fn get_window_coordinate(i: usize) -> (f64, f64) {
    let x = (i % VM_LINE_SIZE as usize) as f64 * 10.0;
    let y = (i / VM_LINE_SIZE as usize) as f64 * 10.0;

    (x, y)
}

fn should_draw_pixel(pixel: u8) -> bool {
    pixel > 0
}

fn draw_pixel(x: f64, y: f64, ctx: Context, renderer: &mut G2d) {
    let rectangle = Rectangle::new(color::BLACK);
    let dims = square(x, y, 10.0);
    rectangle.draw(dims, &ctx.draw_state, ctx.transform, renderer);
}
