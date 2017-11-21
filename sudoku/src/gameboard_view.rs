use graphics::types::Color;
use graphics::{Context, Graphics};

use GameBoardController;

pub struct GameBoardViewSettings {
    /// Position from left-top corner.
    pub position: [f64; 2],
    /// Size of gameboard along horizontal and vertical edge.
    pub size: f64,
    /// Background color.
    pub background_color: Color,
    /// Border color.
    pub border_color: Color,
    /// Edge color around the whole board.
    pub board_edge_color: Color,
    /// Edge color between 3x3 sections.
    pub section_edge_color: Color,
    /// Edge color between cells.
    pub cell_edge_color: Color,
    /// Edge radius around the whole board.
    pub board_edge_radius: f64,
    /// Edge radius between the 3x3 sections.
    pub section_edge_radius: f64,
    /// Edge radius between cells.
    pub cell_edge_radius: f64,
}

impl GameBoardViewSettings {
    /// Creates new gameboard view settings.
    pub fn new() -> GameBoardViewSettings {
        GameBoardViewSettings {
            position: [10.0; 2],
            size: 400.0,
            background_color: [0.8, 0.8, 1.0, 1.0],
            border_color: [0.0, 0.0, 0.2, 1.0],
            board_edge_color: [0.0, 0.0, 0.2, 1.0],
            section_edge_color: [0.0, 0.0, 0.2, 1.0],
            cell_edge_color: [0.0, 0.0, 0.2, 1.0],
            board_edge_radius: 3.0,
            section_edge_radius: 2.0,
            cell_edge_radius: 1.0,
        }
    }
}

pub struct GameBoardView {
    pub settings: GameBoardViewSettings,
}

impl GameBoardView {
    pub fn new(settings: GameBoardViewSettings) -> GameBoardView {
        GameBoardView { settings }
    }

    pub fn draw<G: Graphics>(&self, controller: &GameBoardController, c: &Context, g: &mut G) {}
}
