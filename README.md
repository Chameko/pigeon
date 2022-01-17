# Pigeon

A simple rendering library designed to be implemented into a game engine or extended for general use

## Parrot

A middleware library between wgpu and pidgeon, designed to make wgpu easier to work with. It houses abstractions around for wgpu as well as some convinient functions for pigeon. Parrot is derived from [easygpu](https://github.com/khonsulabs/easygpu/) and is mostly just a re-implementation for pigeon. I would not recommend using it in your library as its not designed with other libraries in mind. A better option would be [easygpu](https://github.com/khonsulabs/easygpu/)
