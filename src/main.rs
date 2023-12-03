mod board;
mod input;

use coffee::graphics::{Color, Frame, Window, WindowSettings, Mesh, Shape, Rectangle, Point};
use coffee::load::Task;
use coffee::{Game, Timer};
use crate::input::{BasicInput, CustomInput};
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
    custom_input: CustomInput,
}

impl Game for MyGame {
    type Input = BasicInput;
    type LoadingScreen = ();


    fn load(_window: &Window) -> Task<MyGame> {
        Task::succeed(|| MyGame { board: board::Board::new(), custom_input: CustomInput::new() })
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        //self.game_size_and_pos = calculate_board_size_and_position(self.grid_size, [_window.width(), _window.height()]);
        frame.clear(Color::WHITE);
        let mut mesh = Mesh::new();
        let selected_cell = self.board.get_selected_cell(self.custom_input.input.mouse_position);

        mesh.fill(
            Shape::Rectangle(Rectangle {
                x: self.board.pos[0],
                y: self.board.pos[1],
                width: self.board.size[0],
                height: self.board.size[1],
            }),
            Color::BLUE,
        );
        if selected_cell.is_some() {
            mesh.fill(
                Shape::Rectangle(Rectangle {
                    x: (self.board.pos[0] + self.board.size[0]) - (selected_cell.unwrap()[0] + 1) as f32 * self.board.grid_cell_size,
                    y: (self.board.pos[1] + self.board.size[1]) - (selected_cell.unwrap()[1] + 1) as f32 * self.board.grid_cell_size,
                    width: self.board.grid_cell_size,
                    height: self.board.grid_cell_size,
                }),
                Color::GREEN,
            );
        }
        self.board.render_grid(&mut mesh);
        mesh.draw(&mut frame.as_target());
    }

    fn interact(&mut self, input: &mut BasicInput, _window: &mut Window) {
        self.custom_input.input.mouse_position = input.mouse_position;
        self.custom_input.input.mouse_buttons_pressed = input.mouse_buttons_pressed.clone();
        self.custom_input.input.mouse_wheel = input.mouse_wheel;
        self.custom_input.input.keys_pressed = input.keys_pressed.clone();
        self.custom_input.input.text_buffer = input.text_buffer.clone();
    }

    fn update(&mut self, _window: &Window) -> () {
        self.board.update_board_size(_window);
        //println!("{:?}", self.game_size_and_pos)
        self.board.place_token(0,1,1);
        self.board.place_token(0,0,2);
        self.board.place_token(1,0,2);

        if self.custom_input.mouse_click(mouse::Button::Left) {
            let pressed_cell = self.board.get_selected_cell(self.custom_input.input.mouse_position);
            if pressed_cell.is_some() {
                self.board.place_token(pressed_cell.unwrap()[0], pressed_cell.unwrap()[1], 1);
            }
        }
        if self.custom_input.mouse_click(mouse::Button::Right) {
            let pressed_cell = self.board.get_selected_cell(self.custom_input.input.mouse_position);
            if pressed_cell.is_some() {
                self.board.place_token(pressed_cell.unwrap()[0], pressed_cell.unwrap()[1], 2);
            }
        }

    }
}
