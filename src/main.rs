extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_big_array;

use std::fs::File;
use std::io::prelude::*;

use structopt::StructOpt;

mod engine;
use engine::Engine;

#[derive(Debug, StructOpt)]
#[structopt(name = "rt3conf", about = "Create configuration files for the game Railroad Tycoon 3.")]
struct Opt {
    /// Set screen resolution height
    // The 'h' is already used by the help parameter.
    #[structopt(short = "y", long = "height", default_value = "600")]
    height: u16,
    /// Set screen resolution width
    #[structopt(short = "x", long = "width", default_value = "800")]
    width: u16,
}

fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();

    println!("Writing engine.cfg to current directory...");
    let mut engine = Engine::new();

    // Set screen resolution
    engine.set_height(opt.height);
    engine.set_width(opt.width);

    // Serialize the engine and write it to disk
    let serialized: Vec<u8> = engine.serialize();
    let mut file = File::create("engine.cfg")?;
    file.write_all(&serialized)?;

    Ok(())
}