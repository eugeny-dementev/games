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
        use piston::input::{Button, MouseButton};

        if let Some(position) = e.mouse_cursor_args() {
            self.cursor_pos = position;

            let x = self.cursor_pos[0] - gameboard_view_position[0];
            let y = self.cursor_pos[1] - gameboard_view_position[1];

            if x >= 0.0 && x < size && y >= 0.0 && y < size {
                // Compute the cell position.
                let cell_x = (x / size * 9.0) as usize;
                let cell_y = (y / size * 9.0) as usize;
                self.pointed_cell = Some([cell_x, cell_y]);
            } else {
                self.pointed_cell = None;
            }
        }

        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            self.selected_cell = self.pointed_cell;

            let x = self.cursor_pos[0] - gameboard_view_position[0];
            let y = self.cursor_pos[1] - gameboard_view_position[1];

            if x >= 0.0 && x < size && y >= 0.0 && y < size {
                // Compute the cell position.
                let cell_x = (x / size * 9.0) as usize;
                let cell_y = (y / size * 9.0) as usize;
                self.selected_cell = Some([cell_x, cell_y]);
            } else {
                self.selected_cell = None;
            }
        }
    }
}
