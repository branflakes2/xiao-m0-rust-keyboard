#![allow(dead_code)]
mod hid_codes;

///Defines what happens when a key is pressed. Keys which are used to switch layers must have
///is_layer set to true. Keys with is_layer == true will ignore modifiers and hid_code.
///
///modifiers: modifiers sent when this key is pressed
///hid_code: hid_code sent whhen this key is pressed
///is_layer: is this keystroke used to select a layer
///layer: which layer to select
///toggle: should the layer be toggled as the default, or only be active while the key is held
#[derive(Copy, Clone)]
pub struct KeyStroke {
    pub modifiers: u8,
    pub hid_code: u8,

    // these are used as layer select
    pub is_layer: bool,
    pub layer: usize,
    pub toggle: bool,
}

impl KeyStroke {
    pub fn eq(self, other: KeyStroke) -> bool {
        return self.modifiers == other.modifiers
            && self.hid_code == other.hid_code
            && self.is_layer == other.is_layer
            && self.layer == other.layer
            && self.toggle == other.toggle;
    }
}

pub const NONE: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_NONE,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const A: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_A,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const B: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_B,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const C: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_C,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const D: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_D,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const E: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_E,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const F: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_F,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const G: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_G,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const H: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_H,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const I: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_I,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const J: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_J,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const K: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_K,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const L: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_L,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const M: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_M,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const N: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_N,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const O: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_O,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const P: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_P,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const Q: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_Q,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const R: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_R,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const S: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_S,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const T: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_T,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const U: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_U,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const V: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_V,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const W: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_W,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const X: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_X,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const Y: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_Y,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const Z: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_Z,
    is_layer: false,
    layer: 0,
    toggle: false,
};
