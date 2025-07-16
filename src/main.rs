mod game_logic;
mod game_drawing;
mod game_options;
mod main_state;
mod controls;
mod ki;

use ggez::event;
use ggez::GameResult;
use main_state::MainState;

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("snake", "flix");
    let (ctx, event_loop) = 
        cb.window_setup(ggez::conf::WindowSetup::default().title("Snake Game"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(800.0, 600.0))
        .build()?;
    let mut state = MainState::new()?;
    state.apples.add_apple((5, 5)); // Example apple position
    state.apples.add_rnd_apple(state.options.grid_width, state.options.grid_height, |pos| !state.snake.part_of_snake(pos));
    ctx.gfx.window().set_resizable(true);
    event::run(ctx, event_loop, state)
}