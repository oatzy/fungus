use anyhow::Result;

use rand::distributions::Uniform;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

use super::{
    agent::{Actor, Direction, Spore},
    world::World,
};

pub struct Config {
    deposit: f64,
    diffuse: f64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            deposit: 100 as _,
            diffuse: 0.25,
        }
    }
}

pub struct Fungus {
    world: World,
    actors: Vec<Actor>,
    config: Config,
}

impl Fungus {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            world: World::new(width, height),
            actors: Default::default(),
            config: Default::default(),
        }
    }

    pub fn with_config(mut self, config: Config) -> Self {
        self.config = config;
        self
    }

    pub fn add_spore(&mut self, spore: Spore) {
        let agent = Actor {
            spore,
            history: Default::default(),
        };
        self.actors.push(agent);
    }

    pub fn add_random_spores(&mut self, count: usize) {
        for pos in random_positions(self.world.width, self.world.height).take(count) {
            let mut spore: Spore = Default::default();
            spore.position = pos;

            self.add_spore(spore);
        }
    }

    pub fn iterate(&mut self) {
        for actor in self.actors.iter_mut() {
            self.world
                .deposit_pheromone(&actor.spore.position, self.config.deposit);
            let next = choose_move(&vec![
                look_ahead(&actor.spore, &self.world),
                look_left(&actor.spore, &self.world),
                look_right(&actor.spore, &self.world),
            ]);
            actor.move_to(next.position);
            actor.turn(next.direction);
        }
        self.world.defuse_pheromone(self.config.diffuse);
    }

    pub fn save_image(&self, path: &str) -> Result<()> {
        let imgbuf: image::RgbImage = self.world.clone().into();
        imgbuf.save(path)?;
        Ok(())
    }
}

pub fn random_positions(width: usize, height: usize) -> impl Iterator<Item = (usize, usize)> {
    // generate random agents
    let rng = thread_rng();
    let xrange = Uniform::new_inclusive(0, width - 1);
    let yrange = Uniform::new_inclusive(0, height - 1);

    rng.sample_iter(xrange).zip(rng.sample_iter(yrange))
}

#[derive(Clone)]
struct Move {
    position: (usize, usize),
    direction: Direction,
    pheromone: f64,
}

fn look_ahead(spore: &Spore, world: &World) -> Move {
    let delta: (isize, isize) = match spore.direction {
        Direction::N => (0, 1),
        Direction::NE => (1, 1),
        Direction::E => (1, 0),
        Direction::SE => (1, -1),
        Direction::S => (0, -1),
        Direction::SW => (-1, -1),
        Direction::W => (-1, 0),
        Direction::NW => (-1, 1),
    };
    let position = (
        (spore.position.0 as isize + delta.0).rem_euclid(world.width as _) as _,
        (spore.position.1 as isize + delta.1).rem_euclid(world.height as _) as _,
    );

    Move {
        position,
        direction: spore.direction,
        pheromone: world.get_pheromone(&position).unwrap(),
    }
}

fn look_left(spore: &Spore, world: &World) -> Move {
    let (delta, direction): ((isize, isize), _) = match spore.direction {
        Direction::NW => ((0, 1), Direction::W),
        Direction::N => ((1, 1), Direction::NW),
        Direction::NE => ((1, 0), Direction::N),
        Direction::E => ((1, -1), Direction::NE),
        Direction::SE => ((0, -1), Direction::E),
        Direction::S => ((-1, -1), Direction::SE),
        Direction::SW => ((-1, 0), Direction::S),
        Direction::W => ((-1, 1), Direction::SW),
    };
    let position = (
        (spore.position.0 as isize + delta.0).rem_euclid(world.width as _) as _,
        (spore.position.1 as isize + delta.1).rem_euclid(world.height as _) as _,
    );

    Move {
        position,
        direction,
        pheromone: world.get_pheromone(&position).unwrap(),
    }
}

fn look_right(spore: &Spore, world: &World) -> Move {
    let (delta, direction): ((isize, isize), _) = match spore.direction {
        Direction::NE => ((0, 1), Direction::E),
        Direction::E => ((1, 1), Direction::SE),
        Direction::SE => ((1, 0), Direction::S),
        Direction::S => ((1, -1), Direction::SW),
        Direction::SW => ((0, -1), Direction::W),
        Direction::W => ((-1, -1), Direction::NW),
        Direction::NW => ((-1, 0), Direction::N),
        Direction::N => ((-1, 1), Direction::NE),
    };
    let position = (
        (spore.position.0 as isize + delta.0).rem_euclid(world.width as _) as _,
        (spore.position.1 as isize + delta.1).rem_euclid(world.height as _) as _,
    );

    Move {
        position,
        direction,
        pheromone: world.get_pheromone(&position).unwrap(),
    }
}

fn choose_move(moves: &Vec<Move>) -> Move {
    let nonzero: Vec<_> = moves
        .iter()
        .cloned()
        .filter(|m| m.pheromone != 0.0)
        .collect();

    let mut rng = thread_rng();

    if nonzero.len() != 0 {
        nonzero
            .choose_weighted(&mut rng, |m| m.pheromone)
            .unwrap()
            .to_owned()
    } else {
        moves.choose(&mut rng).unwrap().to_owned()
    }
}
