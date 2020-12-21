/// The engine.cfg file contains 980 bytes.
const ENGINE_CFG_LEN: usize = 980;

/// Only the first 201 bytes of the engine.cfg file seems to have anything
/// useful in it. The rest is zero.
const USED_ENGINE_CFG_LEN: usize = 201;

/// Only the first 201 bytes of the engine.cfg file seems to have anything
/// useful in it. The rest is zero.
const PARTIAL_FIXED_ENGINE_CFG: [u8; USED_ENGINE_CFG_LEN] = [
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
    0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
    0x00,
];

#[cfg(test)]
const HEADER: u32 = 0x00_00_04_1B;

// Serde can by default only handle arrays with up to 32 elements. This adds
// handling of arrays for the lengths we need.
big_array! { BigArray; 39, 779 }

/// Contains the data for the engine.cfg file.
///
/// The fields will be split up into more named fields as they are reverse
/// engineered. A field with a number is an unknown field. The numbering is
/// subject to change.
#[derive(Serialize, Deserialize)]
#[repr(C)]
pub struct Engine {
    header: u32,
    width: u32,
    height: u32,
    field0: [u8; 4],
    full_screen: bool,
    font_shadows: bool,
    field1: [u8; 16],
    anisotropic_filter: bool,
    field2: [u8; 5],
    anti_alias: bool,
    field3: [u8; 3],
    mipmapping_bias: u8,
    mipmapping: u8,
    field4: [u8; 4],
    distance_fogging: bool,
    field5: [u8; 2],
    // FIXME: Something is off, u16 did not align correctly. Maybe it goes from
    //        52 to 54?
    gamma: [u8; 2],
    field6: [u8; 26],
    invert_camera: bool,
    field7: [u8; 11],
    draw_clouds: bool,
    field8: [u8; 5],
    ocean_waves: bool,
    field9: u8,
    water_reflections: bool,
    field10: [u8; 14],
    sound_volume: f32,
    music_volume: f32,
    voice_volume: f32,
    sound_provider: u8,
    field11: [u8; 3],
    speaker_settings: u8,
    field12: [u8; 3],
    disable_safe_refresh_rate: bool,
    #[serde(with = "BigArray")]
    field13: [u8; 39],
    texture_detail: u8,
    field14: [u8; 3],
    disable_color_mouse_cursor: bool,
    field15: [u8; 3],
    disable_accelerated_mouse: bool,
    field16: [u8; 3],
    grey_for_inactive_trains: bool,
    field17: [u8; 7],
    disable_hardware_tnl: bool,
    field18: [u8; 3],
    color_adjustment: u8,
    #[serde(with = "BigArray")]
    field19: [u8; 779],
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

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
    }

    pub fn set_width(&mut self, width: u32) {
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
        assert_eq!(fixed_data[0..USED_ENGINE_CFG_LEN], PARTIAL_FIXED_ENGINE_CFG);
        // The rest should be zeroes
        assert!(fixed_data.iter().skip(USED_ENGINE_CFG_LEN).all(|item| *item == 0));
    }

    #[test]
    fn serialize_fixed() {
        let engine = Engine::new();
        let serialized: Vec<u8> = bincode::serialize(&engine).unwrap();
        // The first part should be useful data
        assert_eq!(serialized.len(), ENGINE_CFG_LEN);
        assert_eq!(serialized[0..USED_ENGINE_CFG_LEN], PARTIAL_FIXED_ENGINE_CFG);
        // The rest should be zeroes
        assert!(serialized.iter().skip(USED_ENGINE_CFG_LEN).all(|item| *item == 0));
    }

    #[test]
    fn deserialize_fixed() {
        let fixed_data = Engine::fixed_data();
        let engine: Engine = bincode::deserialize(&fixed_data[..]).unwrap();
        assert_eq!(engine.header, HEADER);

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
    fn size_of_engine_struct() {
        assert_eq!(std::mem::size_of::<Engine>(), ENGINE_CFG_LEN);
    }

    #[test]
    fn field_offsets() {
        // The field offsets were found through reverse engineering. I opened
        // the settings, changed a value and noticed the difference in the file.
        // This was repeated until I had gone through all the settings. Some of
        // the settings are in the game.cfg file instead of the engine.cfg file.
        assert_eq!(offset_of!(Engine, header), 0);
        assert_eq!(offset_of!(Engine, width), 4);
        assert_eq!(offset_of!(Engine, height), 8);
        assert_eq!(offset_of!(Engine, field0), 12);
        assert_eq!(offset_of!(Engine, full_screen), 16);
        assert_eq!(offset_of!(Engine, font_shadows), 17);
        assert_eq!(offset_of!(Engine, field1), 18);
        assert_eq!(offset_of!(Engine, anisotropic_filter), 34);
        assert_eq!(offset_of!(Engine, field2), 35);
        assert_eq!(offset_of!(Engine, anti_alias), 40);
        assert_eq!(offset_of!(Engine, field3), 41);
        assert_eq!(offset_of!(Engine, mipmapping_bias), 44);
        assert_eq!(offset_of!(Engine, mipmapping), 45);
        assert_eq!(offset_of!(Engine, field4), 46);
        assert_eq!(offset_of!(Engine, distance_fogging), 50);
        assert_eq!(offset_of!(Engine, field5), 51);
        assert_eq!(offset_of!(Engine, gamma), 53);
        assert_eq!(offset_of!(Engine, field6), 55);
        assert_eq!(offset_of!(Engine, invert_camera), 81);
        assert_eq!(offset_of!(Engine, field7), 82);
        assert_eq!(offset_of!(Engine, draw_clouds), 93);
        assert_eq!(offset_of!(Engine, field8), 94);
        assert_eq!(offset_of!(Engine, ocean_waves), 99);
        assert_eq!(offset_of!(Engine, field9), 100);
        assert_eq!(offset_of!(Engine, water_reflections), 101);
        assert_eq!(offset_of!(Engine, field10), 102);
        assert_eq!(offset_of!(Engine, sound_volume), 116);
        assert_eq!(offset_of!(Engine, music_volume), 120);
        assert_eq!(offset_of!(Engine, voice_volume), 124);
        assert_eq!(offset_of!(Engine, sound_provider), 128);
        assert_eq!(offset_of!(Engine, speaker_settings), 132);
        assert_eq!(offset_of!(Engine, field12), 133);
        assert_eq!(offset_of!(Engine, disable_safe_refresh_rate), 136);
        assert_eq!(offset_of!(Engine, field13), 137);
        assert_eq!(offset_of!(Engine, texture_detail), 176);
        assert_eq!(offset_of!(Engine, field14), 177);
        assert_eq!(offset_of!(Engine, disable_color_mouse_cursor), 180);
        assert_eq!(offset_of!(Engine, field15), 181);
        assert_eq!(offset_of!(Engine, disable_accelerated_mouse), 184);
        assert_eq!(offset_of!(Engine, field16), 185);
        assert_eq!(offset_of!(Engine, grey_for_inactive_trains), 188);
        assert_eq!(offset_of!(Engine, field17), 189);
        assert_eq!(offset_of!(Engine, disable_hardware_tnl), 196);
        assert_eq!(offset_of!(Engine, field18), 197);
        assert_eq!(offset_of!(Engine, color_adjustment), 200);
        assert_eq!(offset_of!(Engine, field19), 201);
    }
}