use std::path::PathBuf;

use anyhow::Result;
use argh::FromArgs;
use log::debug;

use fungus::{Config, Fungus};

#[derive(FromArgs)]
/// Fungus simulation
struct Args {
    /// image width (default=100)
    #[argh(option, short = 'W', default = "100")]
    width: usize,

    /// image height (default=100)
    #[argh(option, short = 'H', default = "100")]
    height: usize,

    /// number of iterations to run (default=1000)
    #[argh(option, short = 'T', default = "1000")]
    iterations: usize,

    /// spore count (default=2000)
    #[argh(option, short = 's', default = "2000")]
    spores: usize,

    /// how much pheromone to deposit at each step (default=100)
    #[argh(option, default = "100.0")]
    deposit: f64,

    /// diffusion rate (default=0.75)
    #[argh(option, default = "0.75")]
    diffuse: f64,

    /// whether pheromone should spread out as it diffuses
    #[argh(switch)]
    spread: bool,

    /// how many steps a spore remembers (default=6)
    #[argh(option)]
    memory: Option<usize>,

    /// generate images every nth iteration
    #[argh(option)]
    every: Option<usize>,

    /// directory to save image to (default="./output/")
    #[argh(option, short = 'o')]
    output: Option<PathBuf>,
}

fn main() -> Result<()> {
    env_logger::init();

    let args: Args = argh::from_env();

    let output = args.output.unwrap_or("output".into());

    assert!(args.diffuse <= 1.0);

    let config = Config {
        deposit: args.deposit,
        diffuse: args.diffuse,
        spread: args.spread,
    };

    let mut fungus = Fungus::new(args.width, args.height).with_config(config);
    fungus.add_random_spores(args.spores, args.memory);

    // run the simulation
    for i in 0..args.iterations {
        debug!("step {}", i);
        fungus.iterate();

        if args.every.map(|e| i % e == 0).unwrap_or(false) {
            fungus.save_image(output.join(format!("fungus-{}.png", i)))?;
        }
    }

    // save the final image of the pheromones
    fungus.save_image(output.join(format!("fungus-{}.png", args.iterations)))?;

    Ok(())
}
