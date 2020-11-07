use std::iter;

extern crate image;
extern crate rand;

use rand::distributions::Uniform;
use rand::{thread_rng, Rng};

#[derive(Clone)]
struct World {
    width: usize,
    height: usize,
    buffer: Vec<f64>,
}

impl World {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width: width,
            height: height,
            buffer: vec![0_f64; width * height],
        }
    }

    fn deposit_pheromone(&mut self, location: &(usize, usize), amount: f64) {
        *self
            .buffer
            .get_mut(location.0 + location.1 * self.width)
            .unwrap() += amount;
    }

    fn diffuse_pheromones(&self) -> World {
        // TODO: diffuse further?
        let diffusion_rate = 0.25;
        let mut diffused = Self::new(self.width, self.height);

        for (pos, amount) in self.buffer.iter().enumerate() {
            *diffused.buffer.get_mut(pos).unwrap() = diffusion_rate * amount;
            let share = amount * (1_f64 - diffusion_rate) / 8_f64; // 8 neighbours
            for n in neighbours(pos, self.width, self.height).iter() {
                *diffused.buffer.get_mut(*n).unwrap() += share;
            }
        }
        diffused
    }
}

impl Into<image::RgbImage> for World {
    fn into(self) -> image::RgbImage {
        let max = self
            .buffer
            .iter()
            .max_by(|x, y| x.partial_cmp(y).unwrap())
            .unwrap();

        let mut imgbuf = image::RgbImage::new(self.width as u32, self.height as u32);
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let value = (255_f64
                * self
                    .buffer
                    .get(y as usize * self.width + x as usize)
                    .unwrap()
                / max)
                .round() as u8;
            *pixel = image::Rgb([0, value, 0]);
        }
        imgbuf
    }
}

#[derive(Clone, Copy)]
enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Direction {
    fn adjacent(&self) -> Vec<Direction> {
        match self {
            Direction::N => vec![Direction::NW, Direction::NE],
            Direction::NE => vec![Direction::N, Direction::E],
            Direction::E => vec![Direction::NE, Direction::SE],
            Direction::SE => vec![Direction::E, Direction::S],
            Direction::S => vec![Direction::SE, Direction::SW],
            Direction::SW => vec![Direction::S, Direction::W],
            Direction::W => vec![Direction::SW, Direction::NW],
            Direction::NW => vec![Direction::W, Direction::N],
        }
    }
}

impl Into<(isize, isize)> for Direction {
    fn into(self) -> (isize, isize) {
        match self {
            Direction::N => (0, -1),
            Direction::NE => (1, -1),
            Direction::E => (1, 0),
            Direction::SE => (1, 1),
            Direction::S => (0, 1),
            Direction::SW => (-1, 1),
            Direction::W => (-1, 0),
            Direction::NW => (-1, -1),
        }
    }
}

struct Agent {
    position: (usize, usize),
    heading: Direction,
}

impl Agent {
    fn next_move(&self, world: &World) -> (usize, usize) {
        let buffer = &world.buffer;
        let neighbours = self.neighbours(world.width, world.height);
        let cur = self.position.0 + world.width * self.position.1;
        let pos = neighbours
            .iter()
            .map(|x| (x, buffer.get(*x).unwrap()))
            // TODO: pick random if multiple are the same
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .or(Some((&cur, &0_f64))) // TODO: fix this
            .unwrap()
            .0;
        (pos % world.width, pos / world.width)
    }

    fn neighbours(&self, width: usize, height: usize) -> Vec<usize> {
        let cur = (self.position.0 as isize, self.position.1 as isize);
        self.heading
            .adjacent()
            .iter()
            .chain(iter::once(&self.heading))
            .map(|&x| x.into())
            .map(|(x, y)| (cur.0 + x, cur.1 + y))
            .filter(|(x, y)| *x > 0 && *y > 0)
            .filter(|(x, y)| *x < width as isize && *y < height as isize)
            .map(|(x, y)| (x + width as isize * y) as usize)
            .collect()
    }
}

struct Fungus {
    world: World,
    agents: Vec<Agent>,
}

impl Fungus {
    fn iterate(&mut self) {
        let deposit = 100_f64;

        // deposit pheromones and move
        for agent in self.agents.iter_mut() {
            let next = agent.next_move(&self.world);
            self.world.deposit_pheromone(&(agent.position), deposit);
            agent.heading = match (
                next.0 as isize - agent.position.0 as isize,
                next.1 as isize - agent.position.1 as isize,
            ) {
                (0, 1) => Direction::S,
                (0, -1) => Direction::N,
                (-1, 0) => Direction::W,
                (1, 0) => Direction::E,
                (-1, -1) => Direction::NW,
                (-1, 1) => Direction::SW,
                (1, 1) => Direction::SE,
                (1, -1) => Direction::NE,
                (0, 0) => Direction::SE, // TODO: fix me
                (x, y) => unreachable!("({},{})", x, y),
            };
            agent.position = next;
        }

        self.world = self.world.diffuse_pheromones();
    }
}

fn neighbours(pos: usize, width: usize, height: usize) -> Vec<usize> {
    let p = ((pos / width) as isize, (pos % width) as isize);
    let adjacent: Vec<(isize, isize)> = vec![
        (p.0 - 1, p.1 - 1),
        (p.0 - 1, p.1),
        (p.0, p.1 - 1),
        (p.0 + 1, p.1),
        (p.0, p.1 + 1),
        (p.0 + 1, p.1 + 1),
        (p.0 + 1, p.1 - 1),
        (p.0 - 1, p.1 + 1),
    ];
    adjacent
        .iter()
        .filter(|(x, y)| *x > 0 && *y > 0)
        .filter(|(x, y)| *x < width as isize && *y < height as isize)
        .map(|(x, y)| (*x + width as isize * *y) as usize)
        .collect()
}

fn main() {
    let imgx = 100;
    let imgy = 100;

    // generate random agents
    let mut rng = thread_rng();
    let xrange = Uniform::new_inclusive(0, imgx - 1);
    let yrange = Uniform::new_inclusive(0, imgy - 1);

    let agent_count = 2000;

    let agents: Vec<Agent> = rng
        .sample_iter(xrange)
        .zip(rng.sample_iter(yrange))
        .take(agent_count)
        .map(|x| Agent {
            position: x,
            heading: Direction::S,
        })
        .collect();

    // create world
    let mut world = World::new(imgx, imgy);

    // pre-load some pheromone
    // for pos in rng
    //     .sample_iter(xrange)
    //     .zip(rng.sample_iter(yrange))
    //     .take(10000)
    // {
    //     world.deposit_pheromone(&pos, rng.gen_range(10_f64, 100_f64))
    // }

    let mut fungus = Fungus {
        world: world,
        agents: agents,
    };

    // run the simulation
    let iterations = 1000;

    for i in 0..iterations {
        fungus.iterate();

        if i % 100 == 0 {
            let imgbuf: image::RgbImage = fungus.world.clone().into();
            imgbuf.save(format!("output/fungus-{}.png", i)).unwrap();
        }
    }

    // save an image of the pheromones
    let imgbuf: image::RgbImage = fungus.world.into();
    imgbuf.save("fungus.png").unwrap();
}
