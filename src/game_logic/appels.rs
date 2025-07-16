use rand::prelude::*;

#[derive(Clone)]
pub struct Appels {
    pub positions: Vec<(isize, isize)>,
}
  
impl Appels {
    pub fn new() -> Self {
        Appels {
            positions: Vec::new(),
        }
    }

    pub fn add_apple(&mut self, position: (isize, isize)) {
        self.positions.push(position);
    }

    pub fn add_rnd_apple(&mut self, width: isize, height: isize, is_valid: impl Fn((isize, isize)) -> bool) {
        let mut rng = rand::rng();
        loop {
            let x = rng.random_range(0..i32::try_from(width).unwrap()) as isize;
            let y = rng.random_range(0..i32::try_from(height).unwrap()) as isize;
            let pos = (x, y);

            if is_valid(pos) {
                self.add_apple(pos);
                break;
            }
        }
    }

    pub fn remove_apple(&mut self, position: (isize, isize)) {
        if let Some(index) = self.positions.iter().position(|&pos| pos == position) {
            self.positions.swap_remove(index);
        }
    }

    pub fn is_apple_at(&self, position: (isize, isize)) -> bool {
        self.positions.contains(&position)
    }
}
