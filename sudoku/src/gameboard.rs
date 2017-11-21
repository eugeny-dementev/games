const SIZE: usize = 9;

pub struct GameBoard {
    pub cells: [[u8; SIZE]; SIZE],
}

impl GameBoard {
    pub fn new() -> GameBoard {
        GameBoard {
            cells: [[0; SIZE]; SIZE],
        }
    }
}
