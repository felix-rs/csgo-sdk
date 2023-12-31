//! Patterns used in the crate are saved here.

pub const INPUT_INTERFACE: &[u8; 14] = b"\xB9\x00\x00\x00\x00\x8B\x40\x38\xFF\xD0\x84\xC0\x0F\x85";
pub const VIEW_RENDER_BEAMS: &[u8; 18] =
    b"\xB9\x00\x00\x00\x00\xA1\x00\x00\x00\x00\xFF\x10\xA1\x00\x00\x00\x00\xB9";
pub const GLOW_MANAGER: &[u8; 10] = b"\x0F\x11\x05\x00\x00\x00\x00\x83\xC8\x01";
pub const IS_OTHER_ENEMY: &[u8; 9] = b"\x8B\xCE\xE8\x00\x00\x00\x00\x02\xC0";
