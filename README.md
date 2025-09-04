# gbrust (WIP)

A small Game Boy emulator written in Rust.

Just a hobby project for fun and learning.

## Project Structure
- **gbcore**  
  Contains the actual Game Boy logic.  
  Implemented with `#![no_std]` so it could (maybe) be ported to embedded devices in the future.  

- **gbgl**  
  A very naive GLFW-based frontend.  
  Only exists to run and debug the core during development.  

## Building
```sh
git clone https://github.com/yourusername/gbrust.git
cd gbrust
cargo run -p gbgl -- path/to/rom.gb