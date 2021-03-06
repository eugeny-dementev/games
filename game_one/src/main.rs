extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,   // Rotation for the square.
    shift: f64, // Shift for moving from left to right
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.1, 0.5, 0.1, 1.0];
        const BLUE:  [f32; 4] = [0.1, 0.1, 0.5, 0.9];
        const RED:   [f32; 4] = [0.5, 0.1, 0.1, 1.0];

        let square = rectangle::square(0.0, 0.0, 500.0);
        let square2 = rectangle::square(0.0, 0.0, 400.0);

        let rotation = self.rotation;

        let (x, y) = (
            ((args.width / 2) as f64) + self.shift,
            ((args.height / 2) as f64) + self.shift
        );

        self.shift += 1.0;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            let transform = c.transform.trans(x, y)
                                       .rot_rad(rotation)
                                       .trans(-250.0, -250.0);

            let transform2 = c.transform.trans(x, y)
                                        .rot_rad(-rotation)
                                        .trans(-250.0, -250.0);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);
            rectangle(BLUE, square2, transform2, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "game one",
            [1920, 1080]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
        shift: -250.0,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}