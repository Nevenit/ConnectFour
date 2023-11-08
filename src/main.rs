use coffee::graphics::{Color, Frame, Window, WindowSettings, Mesh, Shape, Point, Rectangle};
use coffee::load::Task;
use coffee::{Game, Result, Timer};

fn main() {
    MyGame::run(WindowSettings {
        title: String::from("BotnetFour"),
        size: (1280, 720),
        resizable: true,
        fullscreen: false,
        maximized: false,
    });
}

fn calculate_board_size_and_position(board_size: [i32; 2], screen_size: [f32; 2]) -> [f32; 4] {
    let board_scale = (board_size[0] / board_size[1]) as f32;
    let screen_scale = screen_size[0] / screen_size[1];
    if board_scale > screen_scale {
        [screen_size[0], screen_size[0] / board_scale, 0.0, (screen_size[1] - (screen_size[0] / board_scale)) / 2.0]
    } else {
        [screen_size[1] * board_scale, screen_size[1], (screen_size[0] - (screen_size[1] * board_scale)) / 2.0, 0.0]
    }

}

struct MyGame {
    grid_size: [i32; 2],
    game_size_and_pos: [f32; 4],
}

impl Game for MyGame {
    type Input = ();
    type LoadingScreen = ();


    fn load(_window: &Window) -> Task<MyGame> {
        Task::succeed(|| MyGame { grid_size: [7,6], game_size_and_pos: [0.0, 0.0, 0.0, 0.0] })
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        //self.game_size_and_pos = calculate_board_size_and_position(self.grid_size, [_window.width(), _window.height()]);
        frame.clear(Color::BLUE);
        let mut mesh = Mesh::new();

        mesh.fill(
            Shape::Rectangle {
                0: Rectangle {
                    x: self.game_size_and_pos[2],
                    y: self.game_size_and_pos[3],
                    width: self.game_size_and_pos[0],
                    height: self.game_size_and_pos[1],
                },
            },
            Color::WHITE,
        );

        let mut x = 0;
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
        }
        mesh.draw(&mut frame.as_target());
    }

    // fn update(&mut self, _window: &Window) -> () {
    //     self.game_size_and_pos = calculate_board_size_and_position(self.grid_size, [_window.width(), _window.height()]);
    // }
}