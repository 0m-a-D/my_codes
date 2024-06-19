use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("My App!").build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        d.draw_text(
            "App works! Will use this to demonstrate multiple stuff",
            12,
            12,
            20,
            Color::BLACK,
        );
    }
}
