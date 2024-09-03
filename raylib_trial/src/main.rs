// use ffi::Rectangle;
use raylib::prelude::*;

fn main() {
    // let (mut rl, thread) = raylib::init().size(960, 540).title("My App!").build();
    //
    // while !rl.window_should_close() {
    //     let mut d = rl.begin_drawing(&thread);
    //
    //     d.clear_background(Color::BLACK);
    //     d.draw_text(
    //         "App works! Will use this to demonstrate NEURAL NETWORK VISUALIZER",
    //         12,
    //         12,
    //         20,
    //         Color::WHITE,
    //     );
    //     d.draw_line(100, 100, 200, 100, Color::WHITE);
    //     d.draw_line(100, 100, 200, 200, Color::WHITE);
    //     d.draw_line(100, 200, 200, 100, Color::WHITE);
    //     d.draw_line(100, 200, 200, 200, Color::WHITE);
    //     d.draw_circle(200, 100, 18.0, Color::BLUE);
    //     d.draw_circle(100, 100, 18.0, Color::RED);
    //     d.draw_circle(200, 200, 18.0, Color::BLUE);
    //     d.draw_circle(100, 200, 18.0, Color::RED);
    // }
    let (mut rl, thread) = raylib::init()
        .size(800, 450)
        .title("Raylib Button Example")
        .build();

    let mut should_quit = false;
    let rect = Rectangle {
        x: 300.0,
        y: 200.0,
        width: 200.0,
        height: 50.0,
    };

    while !rl.window_should_close() && !should_quit {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::RAYWHITE);

        // Create a button
        if d.gui_button(rect, Some(rstr!("Quit"))) {
            should_quit = true;
        }

        d.draw_text("Click the button to quit", 250, 150, 20, Color::BLACK);
    }
}
