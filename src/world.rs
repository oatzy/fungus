use image;

#[derive(Clone, Default, PartialEq)]
struct Cell {
    pheromone: f64,
}

#[derive(Clone)]
pub struct World {
    pub(crate) width: usize,
    pub(crate) height: usize,
    buffer: Vec<Cell>,
}

impl World {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: vec![Default::default(); width * height],
        }
    }

    fn index(&self, pos: &(usize, usize)) -> usize {
        (pos.0 % self.width) + (pos.1 % self.height) * self.width
    }

    pub fn get_pheromone(&self, pos: &(usize, usize)) -> Option<f64> {
        self.buffer.get(self.index(pos)).map(|c| c.pheromone)
    }

    pub fn deposit_pheromone(&mut self, pos: &(usize, usize), amount: f64) {
        let inx = self.index(pos);
        if let Some(cell) = self.buffer.get_mut(inx) {
            cell.pheromone += amount;
        }
    }

    pub fn diffuse_pheromone(&mut self, factor: f64) {
        for cell in self.buffer.iter_mut() {
            cell.pheromone *= factor;
        }
    }

    pub fn diffuse_and_spread(&mut self, factor: f64) {
        // TODO: diffuse further?
        let mut diffused: Vec<Cell> = vec![Default::default(); self.width * self.height];

        for (pos, cell) in self.buffer.iter().enumerate() {
            let share = cell.pheromone * (1.0 - factor) / 8.0; // 8 neighbours

            diffused.get_mut(pos).unwrap().pheromone = cell.pheromone * factor;

            let (x, y) = (pos % self.width, pos / self.width);
            for (dx, dy) in neighbours() {
                let p: (usize, usize) = (
                    (x as isize + dx).rem_euclid(self.width as _) as _,
                    (y as isize + dy).rem_euclid(self.height as _) as _,
                );
                let n = p.0 + p.1 * self.width;
                diffused.get_mut(n).unwrap().pheromone += share;
            }
        }

        self.buffer = diffused;
    }

    fn max(&self) -> f64 {
        self.buffer
            .iter()
            .map(|c| c.pheromone)
            .max_by(|x, y| x.partial_cmp(y).unwrap())
            .unwrap_or(0 as _)
    }
}

impl Into<image::RgbImage> for World {
    fn into(self) -> image::RgbImage {
        let max = self.max();

        let mut imgbuf = image::RgbImage::new(self.width as _, self.height as _);
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let value =
                (255_f64 * self.get_pheromone(&(x as _, y as _)).unwrap() / max).round() as _;
            *pixel = image::Rgb([0, value, 0]);
        }
        imgbuf
    }
}

fn neighbours() -> impl Iterator<Item = (isize, isize)> {
    (-1..=1)
        .map(|dx| (-1..=1).map(move |dy| (dx, dy)))
        .flatten()
        .filter(|d| d != &(0, 0))
}
