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
    /// Screen resolution height
    // The 'h' is already used by the help parameter.
    #[structopt(short = "y", long = "height", default_value = "600")]
    height: u16,
    /// Screen resolution width
    #[structopt(short = "x", long = "width", default_value = "800")]
    width: u16,
    /// Hardware texture and lighting (T & L) [default: false]
    // I want to explicitly set values no matter the defaults which can be done
    // by using an `Option` instead, e.g. "rt3conf -t false".
    #[structopt(short = "t", long = "hardware-texture-and-lighting")]
    hardware_tnl: Option<bool>,
}

fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();

    println!("Writing engine.cfg to current directory...");
    let mut engine = Engine::new();

    // Set screen resolution
    engine.set_height(opt.height);
    engine.set_width(opt.width);
    if let Some(t) = opt.hardware_tnl {
        engine.set_disable_hardware_tnl(!t);
    }

    // Serialize the engine and write it to disk
    let serialized: Vec<u8> = engine.serialize();
    let mut file = File::create("engine.cfg")?;
    file.write_all(&serialized)?;

    Ok(())
}