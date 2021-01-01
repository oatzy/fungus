use anyhow::Result;

use fungus::Fungus;

fn main() -> Result<()> {
    let width = 100;
    let height = 100;

    let spore_count = 2000;

    let mut fungus = Fungus::new(width, height);
    fungus.add_random_spores(spore_count);

    // run the simulation
    let iterations = 1000;

    for i in 0..iterations {
        fungus.iterate();

        if i % 100 == 0 {
            println!("{}", i);
            // let imgbuf: image::RgbImage = fungus.world.clone().into();
            // imgbuf.save(format!("output/fungus-{}.png", i)).unwrap();
        }
    }

    // save an image of the pheromones
    fungus.save_image("fungus.png")?;

    Ok(())
}
