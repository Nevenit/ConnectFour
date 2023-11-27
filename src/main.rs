mod board;
mod input;

use coffee::graphics::{Color, Frame, Window, WindowSettings, Mesh, Shape, Rectangle};
use coffee::load::Task;
use coffee::{Game, Timer};
use crate::input::CustomInput;
use coffee::input::{Input, keyboard, mouse};

fn main() {
    let _ = MyGame::run(WindowSettings {
        title: String::from("Connect Four"),
        size: (1280, 720),
        resizable: true,
        fullscreen: false,
        maximized: false,
    });
}

struct MyGame {
    board: board::Board,
    input: CustomInput,
}

impl Game for MyGame {
    type Input = CustomInput;
    type LoadingScreen = ();


    fn load(_window: &Window) -> Task<MyGame> {
        Task::succeed(|| MyGame { board: board::Board::new(), input: Input::new() })
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        //self.game_size_and_pos = calculate_board_size_and_position(self.grid_size, [_window.width(), _window.height()]);
        frame.clear(Color::WHITE);
        let mut mesh = Mesh::new();

        mesh.fill(
            Shape::Rectangle(Rectangle {
                x: self.board.pos[0],
                y: self.board.pos[1],
                width: self.board.size[0],
                height: self.board.size[1],
            }),
            Color::BLUE,
        );
        mesh.fill(
            Shape::Rectangle(Rectangle {
                x: self.input.mouse_position.x,
                y: self.input.mouse_position.y,
                width: 50.0,
                height: 50.0,
            }),
            Color::WHITE,
        );
        self.board.render_grid(&mut mesh);
        mesh.draw(&mut frame.as_target());
    }

    fn interact(&mut self, input: &mut CustomInput, _window: &mut Window) {
        self.input.mouse_position = input.mouse_position;
        self.input.mouse_buttons_pressed = input.mouse_buttons_pressed.clone();
        self.input.mouse_wheel = input.mouse_wheel;
        self.input.keys_pressed = input.keys_pressed.clone();
        self.input.text_buffer = input.text_buffer.clone();
    }

    fn update(&mut self, _window: &Window) -> () {
        self.board.update_board_size(_window);
        //println!("{:?}", self.game_size_and_pos)
        self.board.place_token(0,1,1);
        self.board.place_token(0,0,2);
        self.board.place_token(1,0,2);

    }
}
