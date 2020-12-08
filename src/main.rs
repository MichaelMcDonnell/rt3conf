extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_big_array;

use std::fs::File;
use std::io::prelude::*;

/// The engine.cfg file contains 980 bytes.
const ENGINE_CFG_LEN: usize = 980;

// Serde can by default only handle arrays with up to 32 elements. This adds
// arrays for the length we need.
big_array! { BigArray; ENGINE_CFG_LEN }

/// Only the first 197 bytes of the fixed engine.cfg file has anything
/// useful in it. The rest is zero.
const PARTIAL_FIXED_ENGINE_CFG_LEN: usize = 197;

/// Only the first 197 bytes of the fixed engine.cfg file has anything
/// useful in it. The rest is zero.
const PARTIAL_FIXED_ENGINE_CFG: [u8; PARTIAL_FIXED_ENGINE_CFG_LEN] = [
    0x1B, 0x04, 0x00, 0x00, 0x20, 0x03, 0x00, 0x00,
    0x58, 0x02, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00,
    0x01, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
    0x01, 0x01, 0x00, 0x01, 0x01, 0x01, 0x01, 0x01,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00,
    0x00, 0x01, 0x01, 0x00, 0x00, 0x80, 0x3F, 0x00,
    0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x01, 0x92, 0x0A, 0x86,
    0x3F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x01, 0x01, 0x01, 0x01, 0x01,
    0x01, 0x01, 0x01, 0x01, 0x01, 0x00, 0x01, 0x01,
    0x01, 0x01, 0x01, 0x00, 0x08, 0x3D, 0x18, 0x45,
    0x84, 0x66, 0x00, 0x00, 0x33, 0x33, 0x33, 0x3F,
    0x00, 0x00, 0x80, 0x3F, 0x00, 0x00, 0x80, 0x3F,
    0x04, 0x00, 0x00, 0x00, 0x63, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
    0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x96, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x02, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x01, 0x00, 0x00, 0x00, 0x01
];

/// Contains the data for the engine.cfg file.
#[derive(Serialize, Deserialize)]
struct Engine {
    // Put everything into one field for now. Later split up into more named
    // fields as the format is reverse engineered.
    #[serde(with = "BigArray")]
    field0: [u8; ENGINE_CFG_LEN],
}

impl Engine {
    fn new() -> Self {
        Self {
            field0: Self::fixed_data(),
        }
    }

    fn fixed_data() -> [u8; ENGINE_CFG_LEN] {
        // Initialize with zeros to the right length
        let mut fixed_engine_cfg: [u8; ENGINE_CFG_LEN] = [0; ENGINE_CFG_LEN];
        // Copy useful data into the array
        fixed_engine_cfg[..PARTIAL_FIXED_ENGINE_CFG.len()].clone_from_slice(&PARTIAL_FIXED_ENGINE_CFG);

        fixed_engine_cfg
    }
}

fn main() -> std::io::Result<()> {
    println!("Writing engine.cfg to current directory...");
    let engine = Engine::new();
    let serialized: Vec<u8> = bincode::serialize(&engine).unwrap();
    let mut file = File::create("engine.cfg")?;
    file.write_all(&serialized)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_data() {
        let fixed_data = Engine::fixed_data();
        assert_eq!(fixed_data.len(), ENGINE_CFG_LEN);
        // The first part should be useful data
        assert_eq!(fixed_data[0..PARTIAL_FIXED_ENGINE_CFG_LEN], PARTIAL_FIXED_ENGINE_CFG);
        // The rest should be zeroes
        assert!(fixed_data.iter().skip(PARTIAL_FIXED_ENGINE_CFG_LEN).all(|item| *item == 0));
    }

    #[test]
    fn serialize_fixed() {
        let engine = Engine::new();
        let serialized: Vec<u8> = bincode::serialize(&engine).unwrap();
        // The first part should be useful data
        assert_eq!(serialized.len(), ENGINE_CFG_LEN);
        assert_eq!(serialized[0..PARTIAL_FIXED_ENGINE_CFG_LEN], PARTIAL_FIXED_ENGINE_CFG);
        // The rest should be zeroes
        assert!(serialized.iter().skip(PARTIAL_FIXED_ENGINE_CFG_LEN).all(|item| *item == 0));
    }
}