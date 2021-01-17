use std::collections::VecDeque;
use std::convert::TryInto;

use anyhow::{bail, Error, Result};

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

impl TryInto<Direction> for usize {
    type Error = Error;
    fn try_into(self) -> Result<Direction> {
        Ok(match self {
            0 => Direction::N,
            1 => Direction::NE,
            2 => Direction::E,
            3 => Direction::SE,
            4 => Direction::S,
            5 => Direction::SW,
            6 => Direction::W,
            7 => Direction::NW,
            v => bail!("unsupported value {}", v),
        })
    }
}

impl Into<(isize, isize)> for Direction {
    fn into(self) -> (isize, isize) {
        match self {
            Direction::N => (0, 1),
            Direction::NE => (1, 1),
            Direction::E => (1, 0),
            Direction::SE => (1, -1),
            Direction::S => (0, -1),
            Direction::SW => (-1, -1),
            Direction::W => (-1, 0),
            Direction::NW => (-1, 1),
        }
    }
}

impl Direction {
    pub fn left(&self) -> Self {
        ((*self as isize - 1).rem_euclid(8) as usize)
            .try_into()
            .unwrap()
    }

    pub fn right(&self) -> Self {
        ((*self as isize + 1).rem_euclid(8) as usize)
            .try_into()
            .unwrap()
    }
}

pub struct History<T> {
    history: VecDeque<T>,
    size: usize,
}

impl<T: PartialEq> Default for History<T> {
    fn default() -> Self {
        Self::with_size(6)
    }
}

impl<T: PartialEq> History<T> {
    fn with_size(size: usize) -> Self {
        Self {
            history: VecDeque::with_capacity(size),
            size,
        }
    }

    fn push(&mut self, value: T) {
        if self.size == 0 {
            // special case: no memory
            return;
        }
        if self.history.len() == self.size {
            // only keep the N most recent
            self.history.pop_front();
        }
        self.history.push_back(value);
    }

    pub fn contains(&self, value: &T) -> bool {
        self.history.contains(value)
    }
}

pub struct Spore {
    pub(crate) position: (usize, usize),
    pub(crate) direction: Direction,
    pub(crate) history: History<(usize, usize)>,
}

impl Default for Spore {
    fn default() -> Self {
        Self {
            position: (0, 0),
            direction: Direction::S,
            history: Default::default(),
        }
    }
}

impl Spore {
    pub fn with_memory(size: usize) -> Self {
        Self {
            position: (0, 0),
            direction: Direction::S,
            history: History::with_size(size),
        }
    }

    pub fn move_to(&mut self, position: (usize, usize)) {
        self.history.push(self.position);
        self.position = position;
    }

    pub fn turn(&mut self, direction: Direction) {
        self.direction = direction;
    }
}
