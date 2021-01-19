use std::path::PathBuf;

use anyhow::Result;
use argh::FromArgs;

use fungus::{Config, Fungus};

#[derive(FromArgs)]
/// Fungus simulation
struct Args {
    /// image width
    #[argh(option, short = 'W', default = "100")]
    width: usize,

    /// image height
    #[argh(option, short = 'H', default = "100")]
    height: usize,

    /// number of iterations to run
    #[argh(option, short = 'T', default = "1000")]
    iterations: usize,

    /// spore count
    #[argh(option, short = 's', default = "2000")]
    spores: usize,

    /// how much pheromone to deposit at each step
    #[argh(option, default = "100.0")]
    deposit: f64,

    #[argh(option, default = "0.75")]
    /// diffusion rate
    diffuse: f64,

    #[argh(switch)]
    /// whether pheromone should spread out as it diffuses
    spread: bool,

    #[argh(option)]
    /// how many steps a spore remembers
    memory: Option<usize>,

    #[argh(option)]
    /// generate images every nth iteration
    every: Option<usize>,

    #[argh(option, short = 'o')]
    /// directory to save image to
    output: Option<PathBuf>,
}

fn main() -> Result<()> {
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
        fungus.iterate();

        if args.every.map(|e| i % e == 0).unwrap_or(false) {
            println!("{}", i);
            fungus.save_image(output.join(format!("fungus-{}.png", i)))?;
        }
    }

    // save the final image of the pheromones
    fungus.save_image(output.join(format!("fungus-{}.png", args.iterations)))?;

    Ok(())
}
