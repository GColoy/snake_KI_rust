use std::collections::VecDeque;

use ggez::event;
use ggez::graphics;
use ggez::{Context, GameResult};
use crate::game_logic::snake::Snake;
use crate::game_logic::appels::Appels;
use crate::game_options::GameOptions;
use crate::game_drawing::draw_canvas;
use crate::ki::next_move;

#[derive(Clone)]
pub struct MainState {
    pub snake: Snake,
    pub apples: Appels,
    pub options: GameOptions,
    pub last_tick: u128,
}

impl MainState {
    pub fn new() -> GameResult<MainState> {
        let s = MainState {
            snake: Snake::new(
                Some(VecDeque::from([(0, 4), (0, 3), (0, 2), (0, 1), (0, 0)])), // Initial snake position
            ),
            apples: Appels::new(),
            options: GameOptions {
                grid_width: 20,
                grid_height: 15,
            },
            last_tick: 0,
        };
        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // self.snake.process_input(|key| _ctx.keyboard.is_key_pressed(key));
        self.last_tick = self.last_tick + _ctx.time.delta().as_millis();
        if self.last_tick > 300 {
            self.snake.change_direction(next_move(self.clone()));
            if self.snake.move_snake(|pos| self.apples.is_apple_at(pos)) {
                self.apples.add_rnd_apple(self.options.grid_width, self.options.grid_height, |pos| !self.snake.part_of_snake(pos));
                self.apples.remove_apple(self.snake.get_head());
            }
            if self.snake.has_collided(self.options.grid_width, self.options.grid_height) {
                return Err(ggez::GameError::CustomError("Game Over".to_string()));
            }
            self.last_tick = 0;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(
            ctx,
            graphics::Color::from_rgb(10, 10, 10),
        );

        draw_canvas(self.options.grid_width, self.options.grid_height, ctx, &mut canvas, self)?;

        canvas.finish(ctx)?;
        Ok(())
    }
}