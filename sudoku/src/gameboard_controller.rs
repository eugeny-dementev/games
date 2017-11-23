use piston::input::GenericEvent;

use GameBoard;

pub struct GameBoardController {
    pub gameboard: GameBoard,
    pub selected_cell: Option<[usize; 2]>,
    pub pointed_cell: Option<[usize; 2]>,
    pub cursor_pos: [f64; 2],
}

impl GameBoardController {
    pub fn new(gameboard: GameBoard) -> GameBoardController {
        GameBoardController {
            gameboard,
            selected_cell: None,
            pointed_cell: None,
            cursor_pos: [0.0; 2],
        }
    }

    pub fn event<E: GenericEvent>(&mut self, gameboard_view_position: [f64; 2], size: f64, e: &E) {
        use piston::input::{Key, Button, MouseButton};

        if let Some(position) = e.mouse_cursor_args() {
            self.cursor_pos = position;

            let x = self.cursor_pos[0] - gameboard_view_position[0];
            let y = self.cursor_pos[1] - gameboard_view_position[1];

            self.pointed_cell = get_cell(x, y, size);
        }

        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            self.selected_cell = self.pointed_cell;
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            if let Some(selected_cell) = self.selected_cell {
                match key {
                    Key::D1 => self.gameboard.set(selected_cell, 1),
                    Key::D2 => self.gameboard.set(selected_cell, 2),
                    Key::D3 => self.gameboard.set(selected_cell, 3),
                    Key::D4 => self.gameboard.set(selected_cell, 4),
                    Key::D5 => self.gameboard.set(selected_cell, 5),
                    Key::D6 => self.gameboard.set(selected_cell, 6),
                    Key::D7 => self.gameboard.set(selected_cell, 7),
                    Key::D8 => self.gameboard.set(selected_cell, 8),
                    Key::D9 => self.gameboard.set(selected_cell, 9),
                    _ => {}
                }
            }
        }
    }
}

fn get_cell(x: f64, y: f64, size: f64) -> Option<[usize; 2]> {
    if x >= 0.0 && x < size && y >= 0.0 && y < size {
        // Compute the cell position.
        let cell_x = (x / size * 9.0) as usize;
        let cell_y = (y / size * 9.0) as usize;

        return Some([cell_x, cell_y]);
    }

    None
}
