use coffee::graphics::Color;

struct Token {
    pos: [f32; 2],
    player: i32,
    color: Color
}

impl Token {
    fn new(pos: [f32; 2], player: i32, color: Color) -> Token {
        Token {
            pos,
            player,
            color
        }
    }
}