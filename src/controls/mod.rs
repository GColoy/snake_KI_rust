use ggez::input::keyboard;
use crate::game_logic::snake::{Direction, Snake};

impl Snake {
    #[allow(dead_code)]
    pub fn process_input(&mut self, key_pressed: impl Fn(keyboard::KeyCode) -> bool) {
        if key_pressed(keyboard::KeyCode::W) || key_pressed(keyboard::KeyCode::Up) {
            self.change_direction(Direction::Up);
        } 
        if key_pressed(keyboard::KeyCode::S) || key_pressed(keyboard::KeyCode::Down) {
            self.change_direction(Direction::Down);
        } 
        if key_pressed(keyboard::KeyCode::A) || key_pressed(keyboard::KeyCode::Left) {
            self.change_direction(Direction::Left);
        } 
        if key_pressed(keyboard::KeyCode::D) || key_pressed(keyboard::KeyCode::Right) {
            self.change_direction(Direction::Right);
        }
    }
}