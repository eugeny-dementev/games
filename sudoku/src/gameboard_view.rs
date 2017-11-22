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
    /// Cursor hovered cell color.
    pub pointed_cell_color: Color,
    /// Border color.
    pub border_color: Color,
    /// Selected cell color.
    pub selected_cell_color: Color,
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
            pointed_cell_color: [0.65, 0.65, 1.0, 1.0],
            selected_cell_color: [0.9, 0.9, 1.0, 1.0],
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

    pub fn draw<G: Graphics>(&self, controller: &GameBoardController, c: &Context, g: &mut G) {
        use graphics::{Line, Rectangle};

        let ref settings = self.settings;
        let board_rect = [
            settings.position[0],
            settings.position[1],
            settings.size,
            settings.size,
        ];

        Rectangle::new(settings.background_color).draw(board_rect, &c.draw_state, c.transform, g);

        if let Some(selected_cell) = controller.selected_cell {
            let pointed_cell_rect = [
                settings.position[0] + selected_cell[0] as f64 / 9.0 * settings.size,
                settings.position[1] + selected_cell[1] as f64 / 9.0 * settings.size,
                44.44,
                44.44,
            ];
            Rectangle::new(settings.selected_cell_color).draw(
                pointed_cell_rect,
                &c.draw_state,
                c.transform,
                g,
            );
        }

        if let Some(pointed_cell) = controller.pointed_cell {
            let pointed_cell_rect = [
                settings.position[0] + pointed_cell[0] as f64 / 9.0 * settings.size,
                settings.position[1] + pointed_cell[1] as f64 / 9.0 * settings.size,
                44.44,
                44.44,
            ];
            Rectangle::new(settings.pointed_cell_color).draw(
                pointed_cell_rect,
                &c.draw_state,
                c.transform,
                g,
            );
        }

        let cell_edge = Line::new(settings.cell_edge_color, settings.cell_edge_radius);
        for i in 0..9 {
            if (i % 3) == 0 {
                continue;
            }

            let x = settings.position[0] + i as f64 / 9.0 * settings.size;
            let y = settings.position[1] + i as f64 / 9.0 * settings.size;
            let x2 = settings.position[0] + settings.size;
            let y2 = settings.position[1] + settings.size;

            let vline = [x, settings.position[1], x, y2];
            cell_edge.draw(vline, &c.draw_state, c.transform, g);

            let hline = [settings.position[0], y, x2, y];
            cell_edge.draw(hline, &c.draw_state, c.transform, g);
        }

        let section_edge = Line::new(settings.section_edge_color, settings.section_edge_radius);
        for i in 1..3 {
            let x = settings.position[0] + i as f64 / 3.0 * settings.size;
            let y = settings.position[1] + i as f64 / 3.0 * settings.size;
            let x2 = settings.position[0] + settings.size;
            let y2 = settings.position[1] + settings.size;

            let vline = [x, settings.position[1], x, y2];
            section_edge.draw(vline, &c.draw_state, c.transform, g);

            let hline = [settings.position[0], y, x2, y];
            section_edge.draw(hline, &c.draw_state, c.transform, g);
        }

        Rectangle::new_border(settings.board_edge_color, settings.board_edge_radius).draw(
            board_rect,
            &c.draw_state,
            c.transform,
            g,
        );
    }
}
