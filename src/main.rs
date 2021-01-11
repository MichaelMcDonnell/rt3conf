use std::fs::File;
use std::io::prelude::*;

use structopt::StructOpt;

use rt3conf::Engine;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "rt3conf",
    about = "Create configuration files for the game Railroad Tycoon 3."
)]
struct Opt {
    /// Accelerated mouse [default: true]
    #[structopt(short = "a", long = "accelerated-mouse")]
    accelerated_mouse: Option<bool>,
    /// Font shadows [default: true]
    #[structopt(short = "s", long = "font-shadows")]
    font_shadows: Option<bool>,
    /// Full screen [default: true]
    #[structopt(short = "f", long = "full-screen")]
    full_screen: Option<bool>,
    /// Hardware texture and lighting (T & L) [default: false]
    // I want to explicitly set values no matter the defaults which can be done
    // by using an `Option` instead, e.g. "rt3conf -t false".
    #[structopt(short = "t", long = "hardware-texture-and-lighting")]
    hardware_tnl: Option<bool>,
    /// Screen resolution height
    // The 'h' is already used by the help parameter.
    #[structopt(short = "y", long = "height", default_value = "600")]
    height: u32,
    /// Screen resolution width
    #[structopt(short = "x", long = "width", default_value = "800")]
    width: u32,
}

fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();

    println!("Writing engine.cfg to current directory...");
    let mut engine = Engine::new();

    if let Some(a) = opt.accelerated_mouse {
        engine.set_accelerated_mouse(a);
    }
    if let Some(s) = opt.font_shadows {
        engine.set_font_shadows(s);
    }
    if let Some(f) = opt.full_screen {
        engine.set_full_screen(f);
    }
    if let Some(t) = opt.hardware_tnl {
        engine.set_disable_hardware_tnl(!t);
    }
    // Set screen resolution
    engine.set_height(opt.height);
    engine.set_width(opt.width);

    // Serialize the engine and write it to disk
    let serialized: Vec<u8> = engine.serialize();
    let mut file = File::create("engine.cfg")?;
    file.write_all(&serialized)?;

    Ok(())
}
