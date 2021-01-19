# Fungus Simulation

Agent-based simulation of fungus spores (or something like that).

Spores move around the world, leaving behind pheromones that other spores can follow.

An image of the resulting pheromone  trails is generated.

![fungus](https://raw.githubusercontent.com/oatzy/fungus/master/examples/fungus-10_000spore-075diff-100it.gif "animated spore simulation")

## CLI

```bash
Usage: fungus [-W <width>] [-H <height>] [-T <iterations>] [-s <spores>] [--deposit <deposit>] [--diffuse <diffuse>] [--spread] [--memory <memory>] [--every <every>] [-o <output>]

Fungus simulation

Options:
  -W, --width       image width
  -H, --height      image height
  -T, --iterations  number of iterations to run
  -s, --spores      spore count
  --deposit         how much pheromone to deposit at each step
  --diffuse         diffusion rate
  --spread          whether pheromone should spread out as it diffuses
  --memory          how many steps a spore remembers
  --every           generate images every nth iteration
  -o, --output      directory to save image to
  --help            display usage information
```
