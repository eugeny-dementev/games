use piston::input::GenericEvent;

use GameBoard;

pub struct GameBoardController {
    pub gameboard: GameBoard,
    pub selected_cell: Option<[usize; 2]>,
    pub cursor_pos: [f64; 2],
}

impl GameBoardController {
    pub fn new(gameboard: GameBoard) -> GameBoardController {
        GameBoardController {
            gameboard,
            selected_cell: None,
            cursor_pos: [0.0; 2],
        }
    }

    pub fn event<E: GenericEvent>(&mut self, e: &E) {
        if let Some(pos) = e.mouse_cursor_args() {
            self.cursor_pos = pos;

            println!("sursor pos: {:?}", self.cursor_pos);
        }
    }
}
