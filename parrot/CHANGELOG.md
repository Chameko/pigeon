# Changelog

This keeps note of the API changes so I don't have to stare at commit messages at 3AM

## Version 0.0.2

### Changes

- Included wgpu spirv feature and hence, make the function safe

- Re-exported painter in the library

## Verion 0.0.3

### Changes

- Improved documentation (wow)

- New type of Set to prevent typing out verbose arrays of arrays

- Added function in painter to create index buffers

- Vertex and index buffers are now labled

- Pipeline has been edited to allow for no buffers

- Plumbers now have to know what user defined vertex type they're using

## Version 0.1.0

### Changes

- Plumbers now have a function that returns their name. This is for labeling.

- Lables EVERYWHERE.

- Device in painter is now public

- You can now update vertex and index buffers

- Removed draw and paintable traints, they were confusing and would be better implemented in an external crate

- The prepare function for plumber now works for multiple uniform buffers

- Update buffer functions now properly update a buffers size

## Version 0.2.0

Finally, I created some examples and tests and discovered: Everything doesn't work. D: So I went on a quest to fix it all.

This update also brings multisampling and depth textures into parrot and (hopefully) makes the update buffer functions actually work. As I have just implemented depth buffers and multisampling be aware there's probably going to be bugs.
Also: **EXAMPLES WOOOOOOOOOOOOOOOOOOOOO**, now you can see parrot in action

### Changes

- Updating buffer functions now either update or create a new buffer big enough to fit the data

- Removed the requirement to specify the vertex type your using

- Fixed a lot of bugs

## New

- Added depth textures

- Added multisampling

- Added examples and improved documentation

- Added custom pipelines

- Added frame buffers

- Added basic logging