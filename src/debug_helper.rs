use ggez::graphics::{self};
use std::sync::Mutex;
use once_cell::sync::Lazy;

static DEBUG_LINE: Lazy<Mutex<Vec<Vec<ggez::mint::Point2<f32>>>>> = Lazy::new(|| Mutex::new(Vec::new()));

pub fn add_debug_line(points: Vec<(isize, isize)>) {
    let line_points = points.iter().map(|&(x, y)| ggez::mint::Point2 { x: x as f32, y: y as f32 }).collect::<Vec<_>>();
    DEBUG_LINE.lock().unwrap().push(line_points);
    println!("Debug line added: {:?}", DEBUG_LINE.lock().unwrap());
}

pub fn draw_debug_lines(canvas: &mut ggez::graphics::Canvas, ctx: &mut ggez::Context, box_size: f32, top_distance: f32, left_distance: f32) -> ggez::GameResult<()> {
    for line in DEBUG_LINE.lock().unwrap().iter() {
        let line = line.iter().map(|p| ggez::mint::Point2 { x: p.x * box_size + left_distance + box_size/2.0, y: p.y * box_size + top_distance + box_size/2.0 }).collect::<Vec<_>>();
        let line_mesh = graphics::Mesh::new_line(ctx, &line, 1.0, graphics::Color::from_rgb(200, 200, 200))?;
        canvas.draw(&line_mesh, ggez::mint::Point2 { x: 0.0, y: 0.0 });
    }
    DEBUG_LINE.lock().unwrap().clear(); // Clear after drawing to avoid duplication
    Ok(())
}