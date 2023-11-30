use coffee::graphics::{Color, Mesh, Point, Shape, Window};

pub(crate) struct Board {
    pub(crate) grid: [i32; 2],
    pub(crate) size: [f32; 2],
    pub(crate) pos: [f32; 2],
    pub(crate) token_size: f32,
    pub(crate) grid_cell_size: f32,
    pegs_all: i64,
    pegs_p1: i64,
    pegs_p2: i64
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

    pub(crate) fn place_token(&mut self, x: i32, y: i32, player: i32) {
        let binary_pos: i32 = x + (y * self.grid[0]);
        let pos_mask: i64 = 1 << binary_pos;

        // Check if there is a peg in that position on the board
        if self.pegs_all & pos_mask != 0{
            //println!("Cant place token in pos: {}, {} as there is already a peg there", x, y);
            return;
        }

        // Place peg in pegs_all
        self.pegs_all |= pos_mask;


        if player == 1 {
            self.pegs_p1 |= pos_mask;
        }
        else if player == 2 {
            self.pegs_p2 |= pos_mask;
        }

    }
}