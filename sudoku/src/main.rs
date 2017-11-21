extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

pub use gameboard::GameBoard;
pub use gameboard_controller::GameBoardController;
pub use gameboard_view::{GameBoardView, GameBoardViewSettings};

mod gameboard;
mod gameboard_view;
mod gameboard_controller;

use piston::window::WindowSettings;
use piston::event_loop::{EventLoop, EventSettings, Events};
use piston::input::RenderEvent;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};

fn main() {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Sudoku", [512; 2])
        .opengl(opengl)
        .exit_on_esc(true);

    let mut window: GlutinWindow = settings.build().expect("Could not create window");

    let event_settings = EventSettings::new().lazy(true);
    let mut events = Events::new(event_settings);
    let mut gl = GlGraphics::new(opengl);

    let gameboard = GameBoard::new();
    let mut gameboard_controller = GameBoardController::new(gameboard);
    let gameboard_view_settings = GameBoardViewSettings::new();
    let gameboard_view = GameBoardView::new(gameboard_view_settings);

    while let Some(e) = events.next(&mut window) {
        gameboard_controller.event(&e);
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                use graphics::clear;

                clear([1.0; 4], g);
                gameboard_view.draw(&gameboard_controller, &c, g);
            });
        }
    }
}
