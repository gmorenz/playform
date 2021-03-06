## Introduction

playform aspires to be an open-world sandbox game written in Rust.
Right now, it renders a basic world and stops the player from colliding
with it, as well as allowing the player to place and destroy blocks.

Help is appreciated! You can hop over to the [issues page](https://github.com/bfops/playform/issues) to see what needs doing.

## What about [hematite](https://github.com/PistonDevelopers/hematite)?

We're aware of each other! While hematite intends to reproduce Minecfrat behavior and interact with actual Minecraft, playform's design goal is just to be fun and Minecraft-inspired - we'll have no reservations about diverging.

That said, these projects should probably keep a closer eye on each other than they do! If you notice redundancies, feel free to point them out.

## Making it work

It can be built with `cargo build`, which should grab dependencies
automatically. `playform` can then be run from the `target` directory.

  * Move: WASD
  * Jump: Space
  * Look around: Mouse
  * Remove block: Left-click
  * Place dirt block: Right-click
  * Toggle octree rendering: O
  * Save line-of-sight: M

One mob spawns that will play a tag-lke game with you: once you touch it, it
will follow you until it touches you, at which point it will stop again.

## Screenshots

![screenshot 1](/../screenshots/screenshots/screenshot1.png?raw=true)
![screenshot 2](/../screenshots/screenshots/screenshot2.png?raw=true)
![screenshot 3](/../screenshots/screenshots/screenshot3.png?raw=true)
![screenshot 4](/../screenshots/screenshots/screenshot4.png?raw=true)
