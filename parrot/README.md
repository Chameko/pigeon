# Parrot

![Latest version](https://img.shields.io/crates/v/pigeon-parrot?style=flat-square)
![Docs](https://img.shields.io/docsrs/pigeon-parrot?style=flat-square)
![License](https://img.shields.io/crates/l/pigeon-parrot?style=flat-square)

A repeated middleware library for wgpu.

## Design

Parrot is more or less a re-implementation of [easygpu](https://github.com/khonsulabs/easygpu) for pigeon, except missing basic features like depth textures and multi sampling. The reason for these is I want to understand them a bit better before implementing them into parrot and because they aren't essential to pigeon.
