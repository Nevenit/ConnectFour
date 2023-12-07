use coffee::graphics::{Color, Mesh, Point, Shape, Window};

pub(crate) struct Board {
    pub(crate) grid: [i32; 2],
    pub(crate) size: [f32; 2],
    pub(crate) pos: [f32; 2],
    pub(crate) token_size: f32,
    pub(crate) grid_cell_size: f32,
    pegs_all: i64,
    pegs_p1: i64,
    pegs_p2: i64,
    win_patterns: [i64; 4]
}

impl Board {
    pub(crate) fn new() -> Board {
        Board {
            grid: [8, 7],
            size: [0.0, 0.0],
            pos: [0.0, 0.0],
            token_size: 0.0,
            grid_cell_size: 0.0,
            pegs_all: 0,
            pegs_p1: 0,
            pegs_p2: 0,
            // Vertical, horizontal, bottom-left to top-right, top-left to bottom-right
            win_patterns: [0b0000000000000000000000000000000000000001000000010000000100000001,
                            0b0000000000000000000000000000000000000000000000000000000000001111,
                            0b0000000000000000000000000000000000001000000001000000001000000001,
                            0b0000000000000000000000000000000000000001000000100000010000001000]
        }
    }

    pub(crate) fn calculate_board_size_and_position(&mut self, screen_size: [f32; 2]){
        let board_scale = self.grid[0] as f32 / self.grid[1] as f32;
        let screen_scale = screen_size[0] / screen_size[1];
        if board_scale > screen_scale {
            self.size = [screen_size[0], screen_size[0] / board_scale];
            self.pos = [0.0, (screen_size[1] - (screen_size[0] / board_scale)) / 2.0];
        } else {
            self.size = [screen_size[1] * board_scale, screen_size[1]];
            self.pos = [(screen_size[0] - (screen_size[1] * board_scale)) / 2.0, 0.0];
        }
    }

     pub(crate) fn calculate_token_size(&mut self) {
        self.token_size = (self.size[0] * 0.85) / self.grid[0] as f32;
     }

    pub(crate) fn calculate_grid_cell_size(&mut self){
        self.grid_cell_size = self.size[0] / self.grid[0] as f32;
    }

    pub(crate) fn update_board_size(&mut self, _window: &Window) {
        self.calculate_board_size_and_position([_window.width(), _window.height()]);
        self.calculate_token_size();
        self.calculate_grid_cell_size();
    }

    pub(crate) fn get_selected_cell(&mut self, mouse_pos: Point) -> Option<[i32; 2]> {
        let mouse_on_board: Point = Point::new(mouse_pos.x - self.pos[0],
                                               mouse_pos.y - self.pos[1]);
        let cell_x = (mouse_on_board.x / self.grid_cell_size).floor() as i32;
        let cell_y = (mouse_on_board.y / self.grid_cell_size).floor() as i32;
        if  cell_x < 0 || cell_x >= self.grid[0] || cell_y < 0 || cell_y >= self.grid[1]  {
            return None
        }

        Some([self.grid[0] - 1 - (mouse_on_board.x / self.grid_cell_size).floor() as i32,
            self.grid[1] - 1 - (mouse_on_board.y / self.grid_cell_size).floor() as i32])
    }

    // Change to render grid, probably chuck it in the board class
    pub(crate) fn render_grid(&mut self, mesh: &mut Mesh, mouse_position: &Point) {
        let spacing: [f32; 2] = [self.size[0] / self.grid[0] as f32, self.size[1] / self.grid[1] as f32];
        let selected_cell = self.get_selected_cell(*mouse_position);

        for y in 0..self.grid[1]{
            for x in 0..self.grid[0]{
                let bit_pos = x + (y * self.grid[0]);
                let pos_mask = 1 << bit_pos;

                let mut color = Color::WHITE;

                if selected_cell.is_some() && x == selected_cell.unwrap()[0] {
                    color = Color::from_rgb(150,150,150);
                }

                if self.pegs_all & pos_mask != 0{
                    if self.pegs_p1 & pos_mask != 0 {
                        color = Color::from_rgb(255,255,0);
                    } else if self.pegs_p2 & pos_mask != 0{
                        color = Color::from_rgb(255,0,0);
                    } else {
                        color = Color::from_rgb(100,100,100);
                        //println!("Something went very weird, there is a peg here but it doesnt belong to either player.");
                    }
                }

                mesh.fill(
                    Shape::Circle {
                        center: Point::new((self.pos[0] + self.size[0]) - ((x as f32 * spacing[0]) + (spacing[0] / 2.0)) , (self.pos[1] + self.size[1]) - ((y as f32 * spacing[1]) + (spacing[1] / 2.0))),
                        radius: self.token_size / 2.0,
                    },
                    color,
                );
            }
        }
    }

    pub(crate) fn check_win(&mut self, token_position: [i32; 2], player: i32) {
        let mut player_board: i64;

        if player == 1 {
            player_board = self.pegs_p1.clone();
        } else if player == 2 {
            player_board = self.pegs_p2.clone();
        }

        // Check Vertical


    }

    pub(crate) fn place_token(&mut self, column: i32, player: i32) -> Option<[i32; 2]> {
        for row in 0 .. self.grid[1] {
            let binary_pos = column + (row * self.grid[0]);
            let pos_mask = 1 << binary_pos;
            if self.pegs_all & pos_mask != 0 {
                // There is already a peg in that location
                continue;
            }

            // Place peg in pegs_all
            self.pegs_all |= pos_mask;

            if player == 1 {
                self.pegs_p1 |= pos_mask;
            }
            else if player == 2 {
                self.pegs_p2 |= pos_mask;
            }

            return Some([column, row]);
        }
        // Column is full
        return None
    }
}