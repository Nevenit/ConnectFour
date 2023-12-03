use std::collections::HashSet;
use std::hash::Hash;
use coffee::graphics::Point;
use coffee::input;
use coffee::input::{keyboard, mouse};
use coffee::input::Input;

pub(crate) struct BasicInput {
    pub(crate) mouse_position: Point,
    pub(crate) mouse_buttons_pressed: HashSet<mouse::Button>,
    pub(crate) mouse_wheel: Point,
    pub(crate) keys_pressed: HashSet<keyboard::KeyCode>,
    pub(crate) text_buffer: String,
}

pub(crate) struct CustomInput {
    pub(crate) input: BasicInput,
    mouse_buttons_pressed_down: HashSet<mouse::Button>,
    keys_pressed_down: HashSet<keyboard::KeyCode>
}

impl CustomInput {
    pub(crate) fn new() -> CustomInput {
        CustomInput {
            input: BasicInput {
                mouse_position: Point::new(0.0,0.0),
                mouse_buttons_pressed: HashSet::new(),
                mouse_wheel: Point::new(0.0,0.0),
                keys_pressed: HashSet::new(),
                text_buffer: String::new(),
            },
            mouse_buttons_pressed_down: HashSet::new(),
            keys_pressed_down: HashSet::new(),
        }
    }

    pub(crate) fn mouse_click(&mut self, button: mouse::Button) -> bool {
        if self.input.mouse_buttons_pressed.contains(&button) {
            if self.mouse_buttons_pressed_down.contains(&button) {
                return false;
            }
            self.mouse_buttons_pressed_down.insert(button);
            return true;
        }
        self.mouse_buttons_pressed_down.remove(&button);
        return false
    }
}

impl Input for BasicInput {
    fn new() -> BasicInput {
        BasicInput {
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