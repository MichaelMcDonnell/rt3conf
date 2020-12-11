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

const SIZE_OF_BOOL: usize = 1; // std::mem::size_of<bool>() not working yet
const SIZE_OF_U16: usize = 2; // std::mem::size_of<u16>() not working yet

const DISABLE_ACCELERATED_MOUSE_LEN: usize = SIZE_OF_BOOL;
#[cfg(test)]
const FULL_SCREEN_LEN: usize = SIZE_OF_BOOL;
const FONT_SHADOWS_LEN: usize = SIZE_OF_BOOL;
const DISABLE_HARDWARE_TNL_LEN: usize = SIZE_OF_BOOL;
const WIDTH_LEN: usize = SIZE_OF_U16;
const HEIGHT_LEN: usize = SIZE_OF_U16;

// The field offsets were found through reverse engineering.
const OFFSET_FIELD0: usize = 0;
const OFFSET_WIDTH: usize = 4;
const OFFSET_FIELD1: usize = OFFSET_WIDTH + WIDTH_LEN;
const OFFSET_HEIGHT: usize = 8;
const OFFSET_FIELD2: usize = OFFSET_HEIGHT + HEIGHT_LEN;
const OFFSET_FULL_SCREEN: usize = 16;
const OFFSET_FONT_SHADOWS: usize = OFFSET_FULL_SCREEN + 1; // 17
const OFFSET_FIELD3: usize = OFFSET_FONT_SHADOWS + FONT_SHADOWS_LEN;
const OFFSET_DISABLE_ACCELERATED_MOUSE: usize = 184;
const OFFSET_FIELD4: usize = OFFSET_DISABLE_ACCELERATED_MOUSE + DISABLE_ACCELERATED_MOUSE_LEN;
const OFFSET_DISABLE_HARDWARE_TNL: usize = 196;
const OFFSET_FIELD5: usize = OFFSET_DISABLE_HARDWARE_TNL + DISABLE_HARDWARE_TNL_LEN;

const FIELD0_LEN: usize = OFFSET_WIDTH - OFFSET_FIELD0;
const FIELD1_LEN: usize = OFFSET_HEIGHT - OFFSET_FIELD1;
const FIELD2_LEN: usize = OFFSET_FULL_SCREEN - OFFSET_FIELD2;
const FIELD3_LEN: usize = OFFSET_DISABLE_ACCELERATED_MOUSE - OFFSET_FIELD3;
const FIELD4_LEN: usize = OFFSET_DISABLE_HARDWARE_TNL - OFFSET_FIELD4;
const FIELD5_LEN: usize = ENGINE_CFG_LEN - OFFSET_FIELD5;

// Serde can by default only handle arrays with up to 32 elements. This adds
// handling of arrays for the lengths we need.
big_array! { BigArray; FIELD3_LEN, FIELD5_LEN }

/// Contains the data for the engine.cfg file.
///
/// The fields will be split up into more named fields as they are reverse
/// engineered. A field with a number is an unknown field. The numbering is
/// subject to change.
#[derive(Serialize, Deserialize)]
#[repr(C)]
pub struct Engine {
    field0: [u8; FIELD0_LEN],
    width: u16,
    field1: [u8; FIELD1_LEN],
    height: u16,
    field2: [u8; FIELD2_LEN],
    full_screen: bool,
    font_shadows: bool,
    #[serde(with = "BigArray")]
    field3: [u8; FIELD3_LEN],
    disable_accelerated_mouse: bool,
    field4: [u8; FIELD4_LEN],
    disable_hardware_tnl: bool,
    #[serde(with = "BigArray")]
    field5: [u8; FIELD5_LEN],
}

impl Engine {
    pub fn new() -> Self {
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

    pub fn serialize(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }

    pub fn set_accelerated_mouse(&mut self, accelerated_mouse: bool) {
        // The menu shows "Accelerated Mouse" but the disabled state is stored,
        // i.e. 1 is stored when disabled and 0 when enabled.
        self.disable_accelerated_mouse = !accelerated_mouse;
    }

    pub fn set_font_shadows(&mut self, font_shadows: bool) {
        self.font_shadows = font_shadows;
    }

    pub fn set_full_screen(&mut self, full_screen: bool) {
        self.full_screen = full_screen;
    }

    pub fn set_disable_hardware_tnl(&mut self, disable_hardware_tnl: bool) {
        self.disable_hardware_tnl = disable_hardware_tnl;
    }

    pub fn set_height(&mut self, height: u16) {
        self.height = height;
    }

    pub fn set_width(&mut self, width: u16) {
        self.width = width;
    }
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
        assert!(!engine.disable_accelerated_mouse);
        assert!(engine.font_shadows);
        assert!(engine.full_screen);
        assert!(engine.disable_hardware_tnl);
        assert_eq!(engine.height, 600);
        assert_eq!(engine.width, 800);
    }

    #[test]
    fn non_default_values() {
        let mut engine: Engine = Engine::new();
        
        engine.set_accelerated_mouse(false);
        engine.set_font_shadows(false);
        engine.set_full_screen(false);
        engine.set_disable_hardware_tnl(false);
        engine.set_height(1080);
        engine.set_width(1920);

        // Round-trip serialization to check values are stored and read correctly
        let serialized: Vec<u8> = engine.serialize();
        let deserialized: Engine = bincode::deserialize(&serialized).unwrap();

        assert!(deserialized.disable_accelerated_mouse);
        assert!(!deserialized.font_shadows);
        assert!(!deserialized.full_screen);
        assert!(!deserialized.disable_hardware_tnl);
        assert_eq!(deserialized.height, 1080);
        assert_eq!(deserialized.width, 1920);
    }

    #[test]
    fn field_ordering() {
        assert!(OFFSET_FIELD0 < OFFSET_WIDTH);
        assert!(OFFSET_WIDTH < OFFSET_FIELD1);
        assert!(OFFSET_FIELD1 < OFFSET_HEIGHT);
        assert!(OFFSET_HEIGHT < OFFSET_FIELD2);
        assert!(OFFSET_FIELD2 < OFFSET_FULL_SCREEN);
        assert!(OFFSET_FULL_SCREEN < OFFSET_FONT_SHADOWS);
        assert!(OFFSET_FONT_SHADOWS < OFFSET_FIELD3);
        assert!(OFFSET_FIELD3 < OFFSET_DISABLE_ACCELERATED_MOUSE);
        assert!(OFFSET_DISABLE_ACCELERATED_MOUSE < OFFSET_FIELD4);
        assert!(OFFSET_FIELD4 < OFFSET_DISABLE_HARDWARE_TNL);
        assert!(OFFSET_DISABLE_HARDWARE_TNL < OFFSET_FIELD5);
    }

    #[test]
    fn field_lengths() {
        assert_eq!(OFFSET_FIELD0 + FIELD0_LEN, OFFSET_WIDTH);
        assert_eq!(OFFSET_WIDTH + WIDTH_LEN, OFFSET_FIELD1);
        assert_eq!(OFFSET_FIELD1 + FIELD1_LEN, OFFSET_HEIGHT);
        assert_eq!(OFFSET_HEIGHT + HEIGHT_LEN, OFFSET_FIELD2);
        assert_eq!(OFFSET_FIELD2 + FIELD2_LEN, OFFSET_FULL_SCREEN);
        assert_eq!(OFFSET_FULL_SCREEN + FULL_SCREEN_LEN, OFFSET_FONT_SHADOWS);
        assert_eq!(OFFSET_FONT_SHADOWS + FONT_SHADOWS_LEN, OFFSET_FIELD3);
        assert_eq!(OFFSET_FIELD3 + FIELD3_LEN, OFFSET_DISABLE_ACCELERATED_MOUSE);
        assert_eq!(OFFSET_DISABLE_ACCELERATED_MOUSE + DISABLE_ACCELERATED_MOUSE_LEN, OFFSET_FIELD4);
        assert_eq!(OFFSET_FIELD4 + FIELD4_LEN, OFFSET_DISABLE_HARDWARE_TNL);
        assert_eq!(OFFSET_DISABLE_HARDWARE_TNL + DISABLE_HARDWARE_TNL_LEN, OFFSET_FIELD5);
        assert_eq!(OFFSET_FIELD5 + FIELD5_LEN, ENGINE_CFG_LEN);
    }

    #[test]
    fn fields_total_length() {
        let field_lengths: Vec<usize> = vec![
            FIELD0_LEN,
            WIDTH_LEN,
            FIELD1_LEN,
            HEIGHT_LEN,
            FIELD2_LEN,
            FULL_SCREEN_LEN,
            FONT_SHADOWS_LEN,
            FIELD3_LEN,
            DISABLE_ACCELERATED_MOUSE_LEN,
            FIELD4_LEN,
            DISABLE_HARDWARE_TNL_LEN,
            FIELD5_LEN,
        ];
        let total_len: usize = field_lengths.iter().sum();
        assert_eq!(total_len, ENGINE_CFG_LEN)
    }
}