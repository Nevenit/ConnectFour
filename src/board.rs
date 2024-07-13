use std::cmp::{min};
use coffee::graphics::{Color, Mesh, Point, Shape, Window};

#[derive(Clone)]
pub(crate) struct Board {
    pub(crate) grid: [i32; 2],
    pub(crate) size: [f32; 2],
    pub(crate) pos: [f32; 2],
    pub(crate) token_size: f32,
    pub(crate) grid_cell_size: f32,
    pub(crate) full_board: u64,
    win_overlay: u64,
    debug_overlay: u64,
    pub(crate) pegs_all: u64,
    pegs_p1: u64,
    pegs_p2: u64,
    win_patterns: [u64; 4],
    win_pattern_bounds: [u64; 3]
}

impl Board {
    pub(crate) fn new(size: [i32; 2]) -> Board {
        Board {
            grid: size,
            size: [0.0, 0.0],
            pos: [0.0, 0.0],
            token_size: 0.0,
            grid_cell_size: 0.0,
            full_board: u64::pow(2, (size[0] * size[1]) as u32) - 1,
            win_overlay: 0,
            debug_overlay: 0,
            pegs_all: 0,
            pegs_p1: 0,
            pegs_p2: 0,
            // Vertical, horizontal, top-left to bottom-right, bottom-left to top-right

            win_patterns: [0b0000000000000000000000000000000000000001000000100000010000001,
                            0b0000000000000000000000000000000000000000000000000000000000001111,
                            0b0000000000000000000000000000000000000001000000010000000100000001,
                            0b0000000000000000000000000000000000000000001000001000001000001000],
            // Vertical, top-left to bottom-right, bottom-left to top-right
            win_pattern_bounds: [0b0000000000000000000000000000000000000000000111111111111111111111,
                                0b0000000000000000000000000011100000110000001100000011000001110000,
                                 0b0000000000000000000000111000011000001000000000000100000110000111
            ]
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
                    if self.win_overlay & pos_mask != 0 {
                        // Render win
                        color = Color::from_rgb(0,255,0);
                    } else if self.pegs_p1 & pos_mask != 0 {
                        // Render player 1
                        color = Color::from_rgb(255,255,0);
                    } else if self.pegs_p2 & pos_mask != 0{
                        // Render player 2
                        color = Color::from_rgb(255,0,0);
                    } else {
                        // Render gray in case of error
                        color = Color::from_rgb(100,100,100);
                    }
                }

                if self.debug_overlay & pos_mask != 0 {
                    color = Color::BLACK;
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

    pub(crate) fn debug_print_board(&self, point_pos: [i32; 2]) {
        for y in (0..self.grid[1]).rev() {
            for x in (0..self.grid[0]).rev() {
                if point_pos == [x, y] {
                    print!("V ");
                }
                else {
                    let index = x + y * self.grid[0];
                    if self.pegs_all & (1 << index) != 0 {
                        if self.pegs_p1 & (1 << index) != 0 {
                            print!("X ");
                        } else if self.pegs_p2 & (1 << index) != 0 {
                            print!("O ");
                        }

                    } else {
                        print!("_ ");
                    }
                }

            }
            println!();
        }
    }

    pub(crate) fn check_win(&mut self, token_position: [i32; 2], player: i32) -> bool {
        let player_board: u64;
        let token_mask = 1 << token_position[0] + (token_position[1] * self.grid[0]);

        if player == 1 {
            player_board = self.pegs_p1.clone();
        } else if player == 2 {
            player_board = self.pegs_p2.clone();
        } else {
            println!("Error? Wrong player? What the heck is this:{}", player);
            player_board = self.pegs_p2.clone();
        }
        //let method = 0;


        //if method == 0 {
            let mut win_mask: u64;
            let mut move_count: i32;
            let mut won = false;

            let start_offset: [i32; 2] = [min(3, token_position[0]), min(3, token_position[1])];
            let min_start_offset: i32 = min(start_offset[0], start_offset[1]);
            let end_offset: [i32; 2] = [min(3, self.grid[0] - 1 - token_position[0]), min(3, self.grid[1] - 1 - token_position[1])];
            let min_end_offset: i32 = min(end_offset[0], end_offset[1]);

            // Check Vertical
            // Were only checking new tokens which must be the highest in its row, and if that row is smaller than four its impossible for it to be a win.
            if token_mask & self.win_pattern_bounds[0] == 0 {
                win_mask = self.win_patterns[0] << token_position[0] + ((token_position[1] - start_offset[1]) * self.grid[0]);
                if player_board & win_mask == win_mask {
                    println!("Vertical win detected! Player: {}", player);
                    self.win_overlay = self.win_overlay | win_mask;
                    won = true;
                }
            }

            // Check Horizontal
            // Figure out how many positions we have to check
            move_count = 1 + start_offset[0] + end_offset[0] - 4;
            for i in 0..=move_count{
                // Offset the win mask so it matches the position we want to check
                win_mask = self.win_patterns[1] << token_position[0] - start_offset[0] + (token_position[1] * self.grid[0]) + i;
                if player_board & win_mask == win_mask {
                    println!("Horizontal detected! Player: {}", player);
                    self.win_overlay = self.win_overlay | win_mask;
                    won = true;
                }
            }

            // Check top-left to bottom-right diagonal
            if token_mask & self.win_pattern_bounds[1] == 0 {
                move_count = 1 + min_end_offset + min_start_offset - 4;
                for i in 0..=move_count {
                    win_mask = self.win_patterns[2] << token_position[0] - min_start_offset + i + ((token_position[1] - min_start_offset + i) * self.grid[0]);
                    if player_board & win_mask == win_mask {
                        println!("Left diagonal win detected! Player: {}", player);
                        self.win_overlay = self.win_overlay | win_mask;
                        won = true;
                    }
                }

            }

            // Check bottom-left to top-right diagonal
            if token_mask & self.win_pattern_bounds[2] == 0 {
                // We have to calculate new start and end offsets because of the win pattern having to be chacked in a different direction
                let offset_start = min(3, min(token_position[1], self.grid[0] - 1 - token_position[0]));
                let offset_end = min(3, min(token_position[0], self.grid[1] - 1 - token_position[1]));
                move_count = 1 + offset_start + offset_end - 4;
                for i in 0..=move_count {
                    win_mask = self.win_patterns[3] << (token_position[0] + offset_start - i - 3) + ((token_position[1] - offset_start + i) * self.grid[0]);
                    if player_board & win_mask == win_mask {
                        println!("Right diagonal win detected! Player: {}", player);
                        self.win_overlay = self.win_overlay | win_mask;
                        won = true;
                    }
                }
            }
        //}
        // else if method == 1 {
        //     let mut win_mask;
        //     for y in 0..self.grid[1] - 3 {
        //         for x in 0..self.grid[0] {
        //             win_mask = self.win_patterns[0] << x + (y * self.grid[0]);
        //             if player_board & win_mask == win_mask {
        //                 println!("Vertical win detected! Player: {}", player);
        //                 self.win_overlay = win_mask;
        //                 return true;
        //             }
        //         }
        //     }
        //     for y in 0..self.grid[1] {
        //         for x in 0..self.grid[0] - 3 {
        //             win_mask = self.win_patterns[1] << x + (y * self.grid[0]);
        //             if player_board & win_mask == win_mask {
        //                 println!("Vertical win detected! Player: {}", player);
        //                 self.win_overlay = win_mask;
        //                 return true;
        //             }
        //         }
        //     }
        //     for y in 0..self.grid[1] {
        //         for x in 0..self.grid[0] - 3 {
        //             win_mask = self.win_patterns[2] << x + (y * self.grid[0]);
        //             if player_board & win_mask == win_mask {
        //                 println!("Vertical win detected! Player: {}", player);
        //                 self.win_overlay = win_mask;
        //                 return true;
        //             }
        //         }
        //     }
        //     for y in 0..self.grid[1] {
        //         for x in 0..self.grid[0] - 3 {
        //             win_mask = self.win_patterns[3] << x + (y * self.grid[0]);
        //             if player_board & win_mask == win_mask {
        //                 println!("Vertical win detected! Player: {}", player);
        //                 self.win_overlay = win_mask;
        //                 return true;
        //             }
        //         }
        //     }
        // }


        return won;
    }

    pub(crate) fn place_token(&mut self, column: i32, player: i32) -> Option<[i32; 2]> {
        // Check if the column number is within the valid range
        if column < 0 || column >= self.grid[0] {
            // Column number is out of bounds
            return None;
        }

        for row in 0..self.grid[1] {
            let binary_pos = column + (row * self.grid[0]);
            let pos_mask = 1 << binary_pos;

            // Check if there is already a peg in that location
            if self.pegs_all & pos_mask != 0 {
                continue; // Skip to the next row in the same column
            }

            // Place peg in pegs_all
            self.pegs_all |= pos_mask;

            // Assign the peg to the corresponding player
            if player == 1 {
                self.pegs_p1 |= pos_mask;
            } else if player == 2 {
                self.pegs_p2 |= pos_mask;
            }

            // Return the position where the token was placed
            return Some([column, row]);
        }

        // If no position was found in this column because it's full, return None
        return None;
    }

    pub(crate) fn reset(&mut self) {
        self.win_overlay = 0;
        self.debug_overlay = 0;
        self.pegs_all = 0;
        self.pegs_p1 = 0;
        self.pegs_p2 = 0;
    }
}