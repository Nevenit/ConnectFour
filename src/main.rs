use coffee::graphics::{Color, Frame, Window, WindowSettings, Mesh, Shape, Point};
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

struct MyGame {
    game_size: [i32; 2],


}

impl Game for MyGame {
    type Input = ();
    type LoadingScreen = ();


    fn load(_window: &Window) -> Task<MyGame> {
        Task::succeed(|| MyGame { game_size: [7,6] })
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        frame.clear(Color::BLUE);
        let mut mesh = Mesh::new();
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
}