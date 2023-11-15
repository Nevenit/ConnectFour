use coffee::graphics::Window;

pub(crate) struct Board {
    pub(crate) grid: [f32; 2],
    pub(crate) size: [f32; 2],
    pub(crate) pos: [f32; 2],
    pub(crate)  token_size: f32,
    pegs_all: i64,
    pegs_p1: i64,
    pegs_p2: i64
}

impl Board {
    pub(crate) fn new() -> Board {
        Board {
            grid: [8.0, 7.0],
            size: [0.0, 0.0],
            pos: [0.0, 0.0],
            token_size: 0.0,
            pegs_all: 0,
            pegs_p1: 0,
            pegs_p2: 0,
        }
    }

    pub(crate) fn calculate_board_size_and_position(&mut self, screen_size: [f32; 2]){
        let board_scale = self.grid[0] / self.grid[1];
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
        self.token_size = (self.size[0] * 0.85) / self.grid[0];
     }

    pub(crate) fn update_board_size(&mut self, _window: &Window) {
        self.calculate_board_size_and_position([_window.width(), _window.height()]);
        self.calculate_token_size();
    }
}