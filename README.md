# Pigeon

A simple, flexable, cross-platform 2D rendering library... thing.

## Why

Pigeon's development is primarily driven by the need for a graphical backend for AVN. However I also wanted to keep it seperate from AVN so it could be used for other projects.

## Design

I wanted it to be simple, small, portable and flexable. Pigeon isn't designed to manage your application and create windows, it just draws shapes to a screen real good (or as good as I can make it).

You can see some examples in the example folder

## Goals

- Simple: You create the shapes, and hand them to their corresponding pipeline's function
- Flexable: Allow the addition of custom pipelines and graphics and have pigeon auto-magically render them
- Portable: **TODO** test building with web assembly and other platforms.

## Non goals

- Graphics engine: Pigeon is designed be a component that you can modify to your needs
- Ultra-blazing 4 parallel universe optimisation: I'm still learning graphics programming and I don't have the knowledge. I've done what I feel is a reasonable level of optimisation and don't plan to extend it much more

Pigeon implements a form of auto-batching where all the graphics that are rendered with the same pipeline are grouped together into one large vertex and index array. They are then sorted by texture to minimise the swapping of texture bind groups during rendering.

Pigeon designed to come with very little build in, but instead with the ability to expand with your own pipelines using parrot and your own graphics using pigeon.

# Future plans

For the next major update I'm looking into allowing pigeon to deploy to Web assembly.

# Disclaimer

I am very new to graphics programming and this is more or less an effort to increase my understanding. Use at your own risk.