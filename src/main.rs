#![feature(test)]

mod board;
mod input;
mod tests;

use std::fmt::Debug;
use std::{thread, time};
use coffee::graphics::{Color, Frame, Window, WindowSettings, Mesh, Shape, Rectangle, Point};
use coffee::load::Task;
use coffee::{Game, Timer};
use crate::input::{BasicInput, CustomInput};
use coffee::input::{Input, keyboard, mouse};
use crate::board::Board;

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
        Task::succeed(|| MyGame { board: board::Board::new([7,6]), custom_input: CustomInput::new()})
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        //self.game_size_and_pos = calculate_board_size_and_position(self.grid_size, [_window.width(), _window.height()]);
        frame.clear(Color::WHITE);
        let mut mesh = Mesh::new();
        //let selected_cell = self.board.get_selected_cell(self.custom_input.input.mouse_position);

        mesh.fill(
            Shape::Rectangle(Rectangle {
                x: self.board.pos[0],
                y: self.board.pos[1],
                width: self.board.size[0],
                height: self.board.size[1],
            }),
            Color::BLUE,
        );
        /*if selected_cell.is_some() {
            mesh.fill(
                Shape::Rectangle(Rectangle {
                    x: (self.board.pos[0] + self.board.size[0]) - (selected_cell.unwrap()[0] + 1) as f32 * self.board.grid_cell_size,
                    y: (self.board.pos[1] + self.board.size[1]) - (selected_cell.unwrap()[1] + 1) as f32 * self.board.grid_cell_size,
                    width: self.board.grid_cell_size,
                    height: self.board.grid_cell_size,
                }),
                Color::GREEN,
            );
        }*/
        self.board.render_grid(&mut mesh, &self.custom_input.input.mouse_position);
        //self.board.render_win_overlay();
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

        let mut counter: i64 = 0;
        board_combinations_recursive(Board::new(self.board.grid), &mut counter);
        println!("Done! Cout: {}", counter);
        let mut line = String::new();
        let b1 = std::io::stdin().read_line(&mut line).unwrap();

        if self.custom_input.mouse_click(mouse::Button::Left) {
            let pressed_cell = self.board.get_selected_cell(self.custom_input.input.mouse_position);
            if pressed_cell.is_some() {
                let token_position = self.board.place_token(pressed_cell.unwrap()[0], 1);
                if token_position.is_some() {
                    self.board.check_win(token_position.unwrap(), 1);
                }
            }
        }
        if self.custom_input.mouse_click(mouse::Button::Right) {
            let pressed_cell = self.board.get_selected_cell(self.custom_input.input.mouse_position);
            if pressed_cell.is_some() {
                let token_position = self.board.place_token(pressed_cell.unwrap()[0], 2);
                if token_position.is_some() {
                    self.board.check_win(token_position.unwrap(), 2);
                }
            }
        }
        if self.custom_input.input.keys_pressed.contains(&keyboard::KeyCode::Space) {
            self.board.reset();
        }
        // if self.custom_input.input.keys_pressed.len() > 0 {
        //     println!();
        //     print!("Keys pressed: ");
        //     for key in &self.custom_input.input.keys_pressed {
        //         print!("{:?} ", key);
        //     }
        // }

        //if self.custom_input.input.keys_pressed.contains()
    }
}

fn board_combinations_recursive(board: Board, counter: &mut i64) {
    let player: i32 = (*counter as i32 % 2) + 1;
    //println!("C:{} P:{}", counter, player);

    for column in 0..board.grid[0] {
        let mut board_copy = board.clone();
        //print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        let token_position = board_copy.place_token(column, player);

        //board_copy.debug_print_board([column,-1]);
        //println!("Count: {}", counter);

        if token_position.is_some() {
            //println!("all: {0}, full: {1}", board_copy.pegs_all, board_copy.full_board);
            if board_copy.pegs_all == board_copy.full_board {
                //println!("Board full");
                continue;
            }

            *counter += 1;

            board_combinations_recursive(board_copy, counter);
        }
        else {
            continue;
        }
    }
}