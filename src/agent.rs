#[derive(Clone, Copy)]
pub enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

pub struct Spore {
    pub(crate) position: (usize, usize),
    pub(crate) direction: Direction,
}

impl Default for Spore {
    fn default() -> Self {
        Self {
            position: (0, 0),
            direction: Direction::S,
        }
    }
}

pub struct Actor {
    pub(crate) spore: Spore,
    pub(crate) history: Vec<(usize, usize)>,
}

impl Actor {
    pub fn move_to(&mut self, position: (usize, usize)) {
        self.history.push(self.spore.position);
        self.spore.position = position;
    }

    pub fn turn(&mut self, direction: Direction) {
        self.spore.direction = direction;
    }
}
