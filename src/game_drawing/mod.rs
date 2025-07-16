use ggez::glam::vec2;
use ggez::graphics::{self, Canvas, Color};

use ggez::{Context, GameResult};
use crate::game_logic::snake::Snake;
use crate::game_logic::appels::Appels;
use crate::main_state::MainState;

pub fn grid(width: isize, height: isize, ctx: &mut Context, canvas: &mut Canvas) -> GameResult<(f32, f32, f32)> {

    let graphics_width = ctx.gfx.drawable_size().0;
    let graphics_height = ctx.gfx.drawable_size().1;
    let box_size = (graphics_width / width as f32).min(graphics_height / height as f32).min(40.0);

    let h = height as f32 * box_size;
    let w = width as f32 * box_size;

    // Draw offset form the side so it's centered
    let top_distance = (graphics_height - h) / 2.0;
    let left_distance = (graphics_width - w) / 2.0;
    let rim_points = vec![
        vec2(left_distance, top_distance),
        vec2(left_distance + w, top_distance),
        vec2(left_distance + w, top_distance + h),
        vec2(left_distance, top_distance + h),
        vec2(left_distance, top_distance),
    ];
    let rim = graphics::Mesh::new_line(ctx, &rim_points, 2.0, Color::from_rgb(100, 100, 100))?;
    canvas.draw(&rim, vec2(0.0, 0.0));
    // Draw horizontal lines
    let horizontal_points = vec![vec2(left_distance, 0.0), vec2(left_distance + w, 0.0)];
    let horizontal_line = graphics::Mesh::new_line(ctx, &horizontal_points, 1.0, Color::from_rgb(100, 100, 100))?;
    for i in 1..height {
        let y = top_distance + (i as f32 * box_size);
        canvas.draw(&horizontal_line, vec2(0.0, y));
    }
    // Draw vertical lines
    let vertical_points = vec![vec2(0.0, top_distance), vec2(0.0, top_distance + h)];
    let vertical_line = graphics::Mesh::new_line(ctx, &vertical_points, 1.0, Color::from_rgb(100, 100, 100))?;
    for i in 1..width {
        let x = left_distance + (i as f32 * box_size);
        canvas.draw(&vertical_line, vec2(x, 0.0));
    }
    Ok((box_size, top_distance, left_distance))
}

//implement draw for Snake
impl Snake {
    fn draw(&self, ctx: &mut Context, canvas: &mut Canvas, box_size: f32, top_distance: f32, left_distance: f32) -> GameResult<()> {
        let rect = graphics::Rect::new(
            0.0,
            0.0,
            box_size,
            box_size,
        );
        let snake_segment = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, Color::from_rgb(0, 200, 0))?;
        for (x, y) in &self.body {
            canvas.draw(&snake_segment, vec2(
                *x as f32 * box_size + left_distance,
                *y as f32 * box_size + top_distance,
            ));
        }
        // draw the head in lighter green
        let snake_head = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, Color::from_rgb(0, 255, 0))?;
        let head_x = self.body[0].0;
        let head_y = self.body[0].1;
        canvas.draw(&snake_head, vec2(
            head_x as f32 * box_size + left_distance,
            head_y as f32 * box_size + top_distance,
        ));
        Ok(())
    }
}

impl Appels {
    fn draw(&self, ctx: &mut Context, canvas: &mut Canvas, box_size: f32, top_distance: f32, left_distance: f32) -> GameResult<()> {
        let rect = graphics::Rect::new(
            0.0,
            0.0,
            box_size,
            box_size,
        );
        let apple_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, Color::from_rgb(255, 0, 0))?;
        for (x, y) in &self.positions {
            canvas.draw(&apple_mesh, vec2(
                *x as f32 * box_size + left_distance,
                *y as f32 * box_size + top_distance,
            ));
        }
        Ok(())
    }
}

pub fn draw_canvas(width: isize, height: isize, ctx: &mut Context, canvas: &mut Canvas, game_state: &MainState) -> GameResult<(f32, f32, f32)> {
    let (box_size, top_distance, left_distance) = grid(width, height, ctx, canvas)?;
    game_state.snake.draw(ctx, canvas, box_size, top_distance, left_distance)?;
    game_state.apples.draw(ctx, canvas, box_size, top_distance, left_distance)?;

    Ok((box_size, top_distance, left_distance))
}