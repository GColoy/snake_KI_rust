use std::collections::VecDeque;

#[derive(Clone)]
pub struct Snake {
    pub body: VecDeque<(isize, isize)>,
    pub direction: Direction,
    pub old_direction: Direction,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Snake {
    pub fn new(body: Option<VecDeque<(isize, isize)>>) -> Self {
        Snake {
            body: body.unwrap_or_else(|| VecDeque::from([(0, 0)])),
            direction: Direction::Right,
            old_direction: Direction::Right,
        }
    }

    pub fn move_snake(&mut self, can_grow: impl Fn((isize, isize)) -> bool) -> bool {
        // Calculate the new head position based on the current direction
        let (head_x, head_y) = self.body[0];
        let new_head = (head_x + self.direction.vec().0, head_y + self.direction.vec().1);

        if !can_grow(new_head) {
            self.body.pop_back();
        }
        self.body.push_front(new_head);
        self.old_direction = self.direction;
        // Check if the snake has eaten an apple
        can_grow(new_head)
    }

    pub fn has_collided(&self, width: isize, height: isize) -> bool {
        let (head_x, head_y) = self.body[0];
        // Check if the snake's head is out of bounds
        if head_x < 0 || head_x >= width || head_y < 0 || head_y >= height {
            return true;
        }
        // Check if the snake's head collides with its body
        for part in self.body.iter().skip(1) {
            if *part == (head_x, head_y) {
                return true;
            }
        }
        false
    }

    pub fn change_direction(&mut self, new_direction: Direction) {
        if self.old_direction == new_direction {
            return;
        }
        self.direction = match (self.old_direction, new_direction) {
            (Direction::Up, Direction::Down) => self.direction,
            (Direction::Down, Direction::Up) => self.direction,
            (Direction::Left, Direction::Right) => self.direction,
            (Direction::Right, Direction::Left) => self.direction,
            _ => new_direction,
        };
    }

    pub fn part_of_snake(&self, position: (isize, isize)) -> bool {
        self.body.contains(&position)
    }

    pub fn get_head(&self) -> (isize, isize) {
        *self.body.front().unwrap_or(&(0, 0))
    }
    
}

impl Direction {
    pub fn vec(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}