use std::collections::HashMap;
use glfw::{Key, MouseButton};

fn to_axis(pos: bool, neg: bool) -> f32 {
    pos as i32 as f32 - neg as i32 as f32
}

pub struct Input {
    keys: HashMap<Key, bool>,
    buttons: HashMap<MouseButton, bool>,
    cursor_pos: (f32, f32),
}

impl Input {
    pub fn new() -> Input {
        Input {
            keys: Default::default(),
            buttons: Default::default(),
            cursor_pos: (0.0, 0.0),
        }
    }

    pub fn key_pressed(&self, key: Key) -> bool {
        **self.keys.get(&key).get_or_insert(&false)
    }

    pub fn button_pressed(&self, button: MouseButton) -> bool {
        **self.buttons.get(&button).get_or_insert(&false)
    }

    pub fn cursor_pos(&self) -> (f32, f32) {
        self.cursor_pos
    }

    pub fn movement(&self) -> (f32, f32, f32) {
        (
            to_axis(self.key_pressed(Key::D), self.key_pressed(Key::A)),
            to_axis(self.key_pressed(Key::Space), self.key_pressed(Key::LeftShift)),
            to_axis(self.key_pressed(Key::W), self.key_pressed(Key::S)),
        )
    }

    pub fn set_key_pressed(&mut self, key: Key, pressed: bool) {
        self.keys.insert(key, pressed);
    }

    pub fn set_button_pressed(&mut self, button: MouseButton, pressed: bool) {
        self.buttons.insert(button, pressed);
    }

    pub fn set_cursor_pos(&mut self, x: f32, y: f32) {
        self.cursor_pos = (x, y)
    }
}