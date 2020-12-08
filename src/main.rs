extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_big_array;

use std::fs::File;
use std::io::prelude::*;

/// The engine.cfg file contains 980 bytes.
const ENGINE_CFG_LEN: usize = 980;

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

const SIZE_OF_U16: usize = 2; // std::mem::size_of<u16>() not working yet
const RESOLUTION_X_LEN: usize = SIZE_OF_U16;
const RESOLUTION_Y_LEN: usize = SIZE_OF_U16;

// The field offsets were found through reverse engineering.
const OFFSET_FIELD0: usize = 0;
const OFFSET_RESOLUTION_X: usize = 4;
const OFFSET_FIELD1: usize = OFFSET_RESOLUTION_X + RESOLUTION_X_LEN;
const OFFSET_RESOLUTION_Y: usize = 8;
const OFFSET_FIELD2: usize = OFFSET_RESOLUTION_Y + RESOLUTION_Y_LEN;

const FIELD0_LEN: usize = OFFSET_RESOLUTION_X - OFFSET_FIELD0;
const FIELD1_LEN: usize = OFFSET_RESOLUTION_Y - OFFSET_FIELD1;
const FIELD2_LEN: usize = ENGINE_CFG_LEN - OFFSET_FIELD2;

// Serde can by default only handle arrays with up to 32 elements. This adds
// arrays for the length we need.
big_array! { BigArray; FIELD2_LEN }

/// Contains the data for the engine.cfg file.
///
/// The fields will be split up into more named fields as they are reverse
/// engineered. A field with a number is an unknown field. The numbering is
/// subject to change.
#[derive(Serialize, Deserialize)]
struct Engine {
    field0: [u8; FIELD0_LEN],
    resolution_x: u16,
    field1: [u8; FIELD1_LEN],
    resolution_y: u16,
    #[serde(with = "BigArray")]
    field2: [u8; FIELD2_LEN],
}

impl Engine {
    fn new() -> Self {
        let fixed_data = Engine::fixed_data();
        let engine: Engine = bincode::deserialize(&fixed_data[..]).unwrap();

        engine
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
    use std::assert_eq;

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

    #[test]
    fn deserialize_fixed() {
        let fixed_data = Engine::fixed_data();
        let engine: Engine = bincode::deserialize(&fixed_data[..]).unwrap();
        // You can check these values in the game's settings
        assert_eq!(engine.resolution_x, 800);
        assert_eq!(engine.resolution_y, 600);
    }

    #[test]
    fn field_ordering() {
        assert!(OFFSET_FIELD0 < OFFSET_RESOLUTION_X);
        assert!(OFFSET_RESOLUTION_X < OFFSET_FIELD1);
        assert!(OFFSET_FIELD1 < OFFSET_RESOLUTION_Y);
        assert!(OFFSET_RESOLUTION_Y < OFFSET_FIELD2);
    }
}