# Pigeon

A simple, cross-platform 2D rendering library.

## Why

Pigeon's development is primarily driven by the need for a graphical backend for AVN. However I also wanted to keep it seperate from AVN so it could be used for other projects, such as a backend for [egui](https://github.com/emilk/egui).

I wanted it to be simple, yet flexable enough that it could be implemented as a component in other projects and expanded on with custom pipelines .etc with parrot.

## Disclaimer

I am very new to graphics programming and this is more or less an effort to increase my understanding. Use at your own risk

## Dependencies

Pigeon uses parrot as a simplifying layer between it and wgpu and uses winit for windowing.
