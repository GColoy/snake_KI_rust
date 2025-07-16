use std::collections::VecDeque;

use crate::MainState;
use crate::game_logic::snake::Direction;

pub fn next_move(state: MainState) -> Direction {
    let head = state.snake.get_head();
    let apple = get_closest_apple(&state);

    match apple {
        Some(apple_pos) => {
            if head.0 < apple_pos.0 && state.snake.old_direction != Direction::Left {
                return Direction::Right
            } 
            if head.0 > apple_pos.0 && state.snake.old_direction != Direction::Left {
                return Direction::Left
            } 
            if head.1 < apple_pos.1 && state.snake.old_direction != Direction::Up {
                return Direction::Down
            } 
            if head.1 > apple_pos.1 && state.snake.old_direction != Direction::Up {
                return Direction::Up
            }
            return state.snake.old_direction; // If already moving towards the apple, keep the direction
        }
        None => return Direction::Right,
    }
}

fn get_closest_apple(state: &MainState) -> Option<(isize, isize)> {
    state.apples.positions.iter().cloned().min_by_key(|apple| {
        manhattan_distance(state.snake.get_head(), *apple)
    })
}

fn manhattan_distance(start: (isize, isize), position: (isize, isize)) -> isize {
    return (start.0 - position.0).abs() + (start.1 - position.1).abs();
}

pub fn get_straights(path: &VecDeque<(isize, isize)>) -> Vec<(isize, isize)> {
    let mut straights = Vec::new();
    straights.push(path[0]);
    for i in 1..path.len() - 1 {
        if path[i].0 == path[i - 1].0 || path[i].1 == path[i - 1].1 {
            continue;
        } else {
            straights.push(path[i - 1]);
        }
    }
    straights.push(*path.back().unwrap());
    straights
}