#![allow(dead_code)]
mod hid_codes;

pub struct KeyStroke {
    pub modifiers: u8,
    pub hid_code: u8,
}

pub const A: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_A,
};
pub const B: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_B,
};
pub const C: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_C,
};
pub const D: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_D,
};
pub const E: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_E,
};
pub const F: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_F,
};
pub const G: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_G,
};
pub const H: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_H,
};
pub const I: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_I,
};
pub const J: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_J,
};
pub const K: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_K,
};
pub const L: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_L,
};
pub const M: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_M,
};
pub const N: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_N,
};
pub const O: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_O,
};
pub const P: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_P,
};
pub const Q: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_Q,
};
pub const R: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_R,
};
pub const S: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_S,
};
pub const T: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_T,
};
pub const U: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_U,
};
pub const V: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_V,
};
pub const W: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_W,
};
pub const X: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_X,
};
pub const Y: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_Y,
};
pub const Z: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_Z,
};
