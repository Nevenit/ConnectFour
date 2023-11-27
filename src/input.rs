use std::collections::HashSet;
use coffee::graphics::Point;
use coffee::input;
use coffee::input::{keyboard, mouse};
use coffee::input::Input;

pub(crate) struct CustomInput {
    pub(crate) mouse_position: Point,
    pub(crate) mouse_buttons_pressed: HashSet<mouse::Button>,
    pub(crate) mouse_wheel: Point,
    pub(crate) keys_pressed: HashSet<keyboard::KeyCode>,
    pub(crate) text_buffer: String,
}

impl Input for CustomInput {
    fn new() -> CustomInput {
        CustomInput {
            mouse_position: Point::new(0.0,0.0),
            mouse_buttons_pressed: HashSet::new(),
            mouse_wheel: Point::new(0.0,0.0),
            keys_pressed: HashSet::new(),
            text_buffer: String::new(),
        }
    }

   fn update(&mut self, event: input::Event) {
        match event {
            input::Event::Mouse(mouse_event) => match mouse_event {
                mouse::Event::CursorMoved { x, y}  => {
                    self.mouse_position = Point::new(x, y);
                }
                mouse::Event::Input { state, button } => match state {
                    input::ButtonState::Pressed => {
                        self.mouse_buttons_pressed.insert(button);
                    }
                    input::ButtonState::Released => {
                        self.mouse_buttons_pressed.remove(&button);
                    }
                },
                mouse::Event::WheelScrolled { delta_x, delta_y} => {
                    self.mouse_wheel = Point::new(delta_x, delta_y);
                }
                _ => {}
            },
            input:: Event::Keyboard(keyboard_event) => match keyboard_event {
                keyboard::Event::TextEntered { character } => {
                    self.text_buffer.push(character);
                }
                keyboard::Event::Input {key_code, state} => match state {
                    input::ButtonState::Pressed => {
                        self.keys_pressed.insert(key_code);
                    }
                    input::ButtonState::Released => {
                        self.keys_pressed.remove(&key_code);
                    }
                },
            },
            _ => {}
        }
    }
    fn clear (&mut self) {
        self.text_buffer.clear();
    }
}