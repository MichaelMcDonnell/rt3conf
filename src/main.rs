extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_big_array;

use std::fs::File;
use std::io::prelude::*;

mod engine;
use engine::Engine;

fn main() -> std::io::Result<()> {
    println!("Writing engine.cfg to current directory...");
    let engine = Engine::new();
    let serialized: Vec<u8> = engine.serialize();
    let mut file = File::create("engine.cfg")?;
    file.write_all(&serialized)?;

    Ok(())
}