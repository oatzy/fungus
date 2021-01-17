use std::convert::TryInto;
use std::path::Path;

use anyhow::Result;

use rand::distributions::Uniform;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

use super::{
    agent::{Direction, Spore},
    world::World,
};

pub struct Config {
    pub deposit: f64,
    pub diffuse: f64,
    pub spread: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            deposit: 100 as _,
            diffuse: 0.25,
            spread: false,
        }
    }
}

pub struct Fungus {
    world: World,
    spores: Vec<Spore>,
    config: Config,
}

impl Fungus {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            world: World::new(width, height),
            spores: Default::default(),
            config: Default::default(),
        }
    }

    pub fn with_config(mut self, config: Config) -> Self {
        self.config = config;
        self
    }

    pub fn add_spore(&mut self, spore: Spore) {
        self.spores.push(spore);
    }

    pub fn add_random_spores(&mut self, count: usize, memory: Option<usize>) {
        let mut rng = thread_rng();

        for pos in random_positions(self.world.width, self.world.height).take(count) {
            let mut spore: Spore = memory.map(|s| Spore::with_memory(s)).unwrap_or_default();
            spore.position = pos;
            spore.direction = rng.gen_range::<usize, _, _>(0, 8).try_into().unwrap();

            self.add_spore(spore);
        }
    }

    pub fn iterate(&mut self) {
        for spore in self.spores.iter_mut() {
            self.world
                .deposit_pheromone(&spore.position, self.config.deposit);
            let next = choose_move(&vec![
                look(spore.direction, &spore, &self.world),
                look(spore.direction.left(), &spore, &self.world),
                look(spore.direction.right(), &spore, &self.world),
            ]);
            spore.move_to(next.position);
            spore.turn(next.direction);
        }
        if self.config.spread {
            self.world.diffuse_and_spread(self.config.diffuse);
        } else {
            self.world.diffuse_pheromone(self.config.diffuse);
        }
    }

    pub fn save_image<P: AsRef<Path>>(&self, path: P) -> Result<()> {
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

fn look(direction: Direction, spore: &Spore, world: &World) -> Move {
    let delta: (isize, isize) = direction.into();
    let position = (
        (spore.position.0 as isize + delta.0).rem_euclid(world.width as _) as _,
        (spore.position.1 as isize + delta.1).rem_euclid(world.height as _) as _,
    );

    let pheromone = if spore.history.contains(&position) {
        0.0
    } else {
        world.get_pheromone(&position).unwrap()
    };

    Move {
        position,
        direction,
        pheromone,
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
