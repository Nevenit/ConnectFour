mod board;
mod token;

use coffee::graphics::{Color, Frame, Window, WindowSettings, Mesh, Shape, Rectangle};
use coffee::load::Task;
use coffee::{Game, Timer};

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
}

impl Game for MyGame {
    type Input = ();
    type LoadingScreen = ();


    fn load(_window: &Window) -> Task<MyGame> {
        Task::succeed(|| MyGame { board: board::Board::new() })
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        //self.game_size_and_pos = calculate_board_size_and_position(self.grid_size, [_window.width(), _window.height()]);
        frame.clear(Color::BLUE);
        let mut mesh = Mesh::new();

        mesh.fill(
            Shape::Rectangle(Rectangle {
                x: self.board.pos[0],
                y: self.board.pos[1],
                width: self.board.size[0],
                height: self.board.size[1],
            }),
            Color::WHITE,
        );

        /*let mut x = 0;
        while x < 10 {
            let mut y = 0;
            while y < 10 {
                mesh.fill(
                    Shape::Circle {
                        center: Point::new(50.0 + (50.0 * x as f32), 50.0 + (50.0 * y as f32)),
                        radius: 15.0,
                    },
                    Color::RED,
                );
                y += 1;
            }
            x += 1;
        }*/
        mesh.draw(&mut frame.as_target());
    }

    fn update(&mut self, _window: &Window) -> () {
        self.board.update_board_size(_window)
        //println!("{:?}", self.game_size_and_pos)
    }
}