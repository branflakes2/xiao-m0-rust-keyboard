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
    pub fn layer(layer: usize) -> Self {
        return KeyStroke {
            modifiers: 0,
            hid_code: hid_codes::KEY_NONE,
            is_layer: true,
            layer: layer,
            toggle: false,
        };
    }

    pub fn t_layer(layer: usize) -> Self {
        return KeyStroke {
            modifiers: 0,
            hid_code: hid_codes::KEY_NONE,
            is_layer: true,
            layer: layer,
            toggle: true,
        };
    }

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
pub const OVER: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_ERR_OVF,
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

pub const TAB: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_TAB,
    is_layer: false,
    layer: 0,
    toggle: false,
};

pub const APOSTROPHE: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_APOSTROPHE,
    is_layer: false,
    layer: 0,
    toggle: false,
};

pub const COMMA: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_COMMA,
    is_layer: false,
    layer: 0,
    toggle: false,
};

pub const PERIOD: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_DOT,
    is_layer: false,
    layer: 0,
    toggle: false,
};

pub const BACKSPACE: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_BACKSPACE,
    is_layer: false,
    layer: 0,
    toggle: false,
};

pub const ESC: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_ESC,
    is_layer: false,
    layer: 0,
    toggle: false,
};

pub const FSLASH: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_SLASH,
    is_layer: false,
    layer: 0,
    toggle: false,
};

pub const LSHIFT: KeyStroke = KeyStroke {
    modifiers: hid_codes::KEY_MOD_LSHIFT,
    hid_code: hid_codes::KEY_NONE,
    is_layer: false,
    layer: 0,
    toggle: false,
};

pub const LCTRL: KeyStroke = KeyStroke {
    modifiers: hid_codes::KEY_MOD_LCTRL,
    hid_code: hid_codes::KEY_NONE,
    is_layer: false,
    layer: 0,
    toggle: false,
};

pub const LMETA: KeyStroke = KeyStroke {
    modifiers: hid_codes::KEY_MOD_LMETA,
    hid_code: hid_codes::KEY_NONE,
    is_layer: false,
    layer: 0,
    toggle: false,
};

pub const LALT: KeyStroke = KeyStroke {
    modifiers: hid_codes::KEY_MOD_LALT,
    hid_code: hid_codes::KEY_NONE,
    is_layer: false,
    layer: 0,
    toggle: false,
};

pub const LEFT: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_LEFT,
    is_layer: false,
    layer: 0,
    toggle: false,
};

pub const RIGHT: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_RIGHT,
    is_layer: false,
    layer: 0,
    toggle: false,
};

pub const UP: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_UP,
    is_layer: false,
    layer: 0,
    toggle: false,
};

pub const DOWN: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_DOWN,
    is_layer: false,
    layer: 0,
    toggle: false,
};

pub const SCOLON: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_SEMICOLON,
    is_layer: false,
    layer: 0,
    toggle: false,
};

pub const SPACE: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_SPACE,
    is_layer: false,
    layer: 0,
    toggle: false,
};

pub const END: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_END,
    is_layer: false,
    layer: 0,
    toggle: false,
};

pub const HOME: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_END,
    is_layer: false,
    layer: 0,
    toggle: false,
};

pub const ENTER: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_ENTER,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const PGUP: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_PAGEUP,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const PGDN: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_PAGEDOWN,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const RCTRL: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_RIGHTCTRL,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const RALT: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_RIGHTALT,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const LAYER1: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_NONE,
    is_layer: true,
    layer: 1,
    toggle: false,
};

pub const LAYER2: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_NONE,
    is_layer: true,
    layer: 2,
    toggle: false,
};

pub const T_LAYER3: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_NONE,
    is_layer: true,
    layer: 3,
    toggle: true,
};
pub const F1: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_F1,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const F2: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_F2,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const F3: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_F3,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const F4: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_F4,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const F5: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_F5,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const F6: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_F6,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const F7: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_F7,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const F8: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_F8,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const F9: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_F9,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const F10: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_F10,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const F11: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_F11,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const F12: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_F12,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const N0: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_KP0,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const N1: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_KP1,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const N2: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_KP2,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const N3: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_KP3,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const N4: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_KP4,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const N5: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_KP5,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const N6: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_KP6,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const N7: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_KP7,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const N8: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_KP8,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const N9: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_KP9,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const NSUB: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_KPMINUS,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const NPLUS: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_KPPLUS,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const NMUL: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_KPASTERISK,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const NDIV: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_KPSLASH,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const NDOT: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_KPDOT,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const K0: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_0,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const K1: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_1,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const K2: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_2,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const K3: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_3,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const K4: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_4,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const K5: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_5,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const K6: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_6,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const K7: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_7,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const K8: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_8,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const K9: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_9,
    is_layer: false,
    layer: 0,
    toggle: false,
};
