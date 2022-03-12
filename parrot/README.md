# Parrot

![Latest version](https://img.shields.io/crates/v/pigeon-parrot?style=flat-square)
![Docs](https://img.shields.io/docsrs/pigeon-parrot?style=flat-square)
![License](https://img.shields.io/crates/l/pigeon-parrot?style=flat-square)

A repeated simplifying library for wgpu.

## Design

Parrot is more or less a re-implementation of [easygpu](https://github.com/khonsulabs/easygpu) for pigeon, except missing basic features like depth textures and multi sampling. The reason for this is I want to understand them a bit better before implementing them into parrot and because they aren't essential to pigeon.

I've also just messed around with the API, making changes as I go and adding/removing functionality as I create pigeon.

This is a long way of saying the API is not stable and is subject to change.
