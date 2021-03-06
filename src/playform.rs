//! The entry point.
#![crate_type = "bin"]
#![deny(warnings)]
#![deny(missing_doc)]
#![feature(globs)]
#![feature(macro_rules)]
#![feature(unsafe_destructor)]
#![feature(phase)]

extern crate gl;
extern crate glw;
extern crate input;
extern crate libc;
extern crate nalgebra;
extern crate ncollide3df32;
extern crate piston;
extern crate png;
extern crate sdl2;
extern crate sdl2_game_window;
extern crate shader_version;

mod common;
mod block;
mod fontloader;
// so the time! macro is defined in main
mod stopwatch;
mod loader;
mod main;
mod mob;
mod octree;
mod physics;
mod ttf;

#[allow(dead_code)]
fn main() {
  return main::main();
}
