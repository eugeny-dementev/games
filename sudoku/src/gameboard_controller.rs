use piston::input::GenericEvent;

use GameBoard;

pub struct GameBoardController {
    pub gameboard: GameBoard,
}

impl GameBoardController {
    pub fn new(gameboard: GameBoard) -> GameBoardController {
        GameBoardController { gameboard }
    }

    pub fn event<E: GenericEvent>(&mut self, e: &E) {}
}
