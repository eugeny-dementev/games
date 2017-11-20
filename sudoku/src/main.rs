extern crate piston;
extern crate glutin_window;

use piston::window::WindowSettings;
use glutin_window::GlutinWindow;

use piston::event_loop::{Events, EventLoop, EventSettings};

fn main() {
    let settings = WindowSettings::new("Sudoku", [512; 2])
        .exit_on_esc(true);
        
    let mut window: GlutinWindow = settings.build()
        .expect("Could not create window");
    let event_settings = EventSettings::new()
        .lazy(true);
    let mut events = Events::new(event_settings);

    while let Some(e) = events.next(&mut window) {

    }
}
