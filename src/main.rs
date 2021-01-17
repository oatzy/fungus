use anyhow::Result;
use argh::FromArgs;

use fungus::{Config, Fungus};

#[derive(FromArgs)]
/// Fungus simulation
struct Args {
    /// image width
    #[argh(option, default = "100")]
    width: usize,

    /// image height
    #[argh(option, default = "100")]
    height: usize,

    /// number of iterations to run
    #[argh(option, default = "1000")]
    iterations: usize,

    /// spore count
    #[argh(option, default = "2000")]
    spores: usize,

    /// how much to deposit at each step
    #[argh(option, default = "100.0")]
    deposit: f64,

    #[argh(option, default = "0.25")]
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
}

fn main() -> Result<()> {
    let args: Args = argh::from_env();

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
            fungus.save_image(format!("output/fungus-{}.png", i))?;
        }
    }

    // save an image of the pheromones
    fungus.save_image("fungus.png")?;

    Ok(())
}
