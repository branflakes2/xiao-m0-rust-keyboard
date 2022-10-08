#![allow(dead_code)]

use self::hid_codes::KEY_MOD_LSHIFT;

mod hid_codes;

pub enum KeyPress<'a> {
    Macro(&'a [MacroGroup<'a>]),
    Single(KeyStroke),
}

pub struct MacroGroup<'a> {
    pub delay_ms: u16,
    pub keystrokes: &'a [KeyStroke],
}

pub struct MacroPress {
    delay_ms: u16,
    stroke: KeyStroke,
}

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
    pub clearable: bool, // allows the mod to be cleared if a keystroke with no modifier is pressed
    // while this key is pressed - used for symbols which need modifiers like !@#

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
            clearable: true,
            is_layer: true,
            layer,
            toggle: false,
        };
    }

    pub fn t_layer(layer: usize) -> Self {
        return KeyStroke {
            modifiers: 0,
            hid_code: hid_codes::KEY_NONE,
            clearable: true,
            is_layer: true,
            layer,
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
    clearable: false,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const OVER: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_ERR_OVF,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const A: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_A,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const B: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_B,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const C: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_C,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const D: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_D,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const E: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_E,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const F: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_F,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const G: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_G,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const H: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_H,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const I: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_I,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const J: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_J,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const K: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_K,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const L: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_L,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const M: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_M,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const N: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_N,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const O: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_O,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const P: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_P,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const Q: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_Q,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const R: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_R,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const S: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_S,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const T: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_T,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const U: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_U,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const V: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_V,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const W: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_W,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const X: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_X,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const Y: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_Y,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const Z: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_Z,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};

pub const TAB: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_TAB,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};

pub const APOSTROPHE: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_APOSTROPHE,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};

pub const COMMA: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_COMMA,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};

pub const PERIOD: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_DOT,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};

pub const BACKSPACE: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_BACKSPACE,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};

pub const DELETE: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_DELETE,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};

pub const ESC: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_ESC,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};

pub const FSLASH: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_SLASH,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};

pub const LSHIFT: KeyStroke = KeyStroke {
    modifiers: hid_codes::KEY_MOD_LSHIFT,
    hid_code: hid_codes::KEY_NONE,
    clearable: false,
    is_layer: false,
    layer: 0,
    toggle: false,
};

pub const LCTRL: KeyStroke = KeyStroke {
    modifiers: hid_codes::KEY_MOD_LCTRL,
    hid_code: hid_codes::KEY_NONE,
    clearable: false,
    is_layer: false,
    layer: 0,
    toggle: false,
};

pub const LMETA: KeyStroke = KeyStroke {
    modifiers: hid_codes::KEY_MOD_LMETA,
    hid_code: hid_codes::KEY_NONE,
    clearable: false,
    is_layer: false,
    layer: 0,
    toggle: false,
};

pub const LALT: KeyStroke = KeyStroke {
    modifiers: hid_codes::KEY_MOD_LALT,
    hid_code: hid_codes::KEY_NONE,
    clearable: false,
    is_layer: false,
    layer: 0,
    toggle: false,
};

pub const LEFT: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_LEFT,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};

pub const RIGHT: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_RIGHT,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};

pub const UP: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_UP,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};

pub const DOWN: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_DOWN,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};

pub const SCOLON: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_SEMICOLON,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};

pub const SPACE: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_SPACE,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};

pub const END: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_END,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};

pub const HOME: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_HOME,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};

pub const ENTER: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_ENTER,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const PGUP: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_PAGEUP,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const PGDN: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_PAGEDOWN,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const RCTRL: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_RIGHTCTRL,
    clearable: false,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const RALT: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_RIGHTALT,
    clearable: false,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const LAYER1: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_NONE,
    clearable: true,
    is_layer: true,
    layer: 1,
    toggle: false,
};

pub const LAYER2: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_NONE,
    clearable: true,
    is_layer: true,
    layer: 2,
    toggle: false,
};

// Layer for games
pub const T_LAYER3: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_NONE,
    clearable: true,
    is_layer: true,
    layer: 3,
    toggle: true,
};
pub const F1: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_F1,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const F2: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_F2,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const F3: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_F3,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const F4: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_F4,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const F5: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_F5,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const F6: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_F6,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const F7: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_F7,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const F8: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_F8,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const F9: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_F9,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const F10: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_F10,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const F11: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_F11,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const F12: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_F12,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const N0: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_KP0,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const N1: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_KP1,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const N2: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_KP2,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const N3: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_KP3,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const N4: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_KP4,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const N5: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_KP5,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const N6: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_KP6,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const N7: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_KP7,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const N8: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_KP8,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const N9: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_KP9,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const NSUB: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_KPMINUS,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const NPLUS: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_KPPLUS,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const NMUL: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_KPASTERISK,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const NDIV: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_KPSLASH,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const NDOT: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_KPDOT,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const K0: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_0,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const K1: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_1,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const K2: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_2,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const K3: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_3,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const K4: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_4,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const K5: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_5,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const K6: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_6,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const K7: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_7,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const K8: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_8,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const K9: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_9,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const BACKTICK: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_GRAVE,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const TILDE: KeyStroke = KeyStroke {
    modifiers: hid_codes::KEY_MOD_LSHIFT,
    hid_code: hid_codes::KEY_GRAVE,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const EXCLAMATION: KeyStroke = KeyStroke {
    modifiers: hid_codes::KEY_MOD_LSHIFT,
    hid_code: hid_codes::KEY_1,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const QUESTION: KeyStroke = KeyStroke {
    modifiers: hid_codes::KEY_MOD_LSHIFT,
    hid_code: hid_codes::KEY_SLASH,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const LPAREN: KeyStroke = KeyStroke {
    modifiers: hid_codes::KEY_MOD_LSHIFT,
    hid_code: hid_codes::KEY_9,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const RPAREN: KeyStroke = KeyStroke {
    modifiers: hid_codes::KEY_MOD_LSHIFT,
    hid_code: hid_codes::KEY_0,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const ASTERISK: KeyStroke = KeyStroke {
    modifiers: hid_codes::KEY_MOD_LSHIFT,
    hid_code: hid_codes::KEY_8,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const ATSIGN: KeyStroke = KeyStroke {
    modifiers: hid_codes::KEY_MOD_LSHIFT,
    hid_code: hid_codes::KEY_2,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const POUND: KeyStroke = KeyStroke {
    modifiers: hid_codes::KEY_MOD_LSHIFT,
    hid_code: hid_codes::KEY_3,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const AND: KeyStroke = KeyStroke {
    modifiers: hid_codes::KEY_MOD_LSHIFT,
    hid_code: hid_codes::KEY_7,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const DOLLAR: KeyStroke = KeyStroke {
    modifiers: hid_codes::KEY_MOD_LSHIFT,
    hid_code: hid_codes::KEY_4,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const PERCENT: KeyStroke = KeyStroke {
    modifiers: hid_codes::KEY_MOD_LSHIFT,
    hid_code: hid_codes::KEY_5,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const EQUAL: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_EQUAL,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const LBRACKET: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_LEFTBRACE,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const RBRACKET: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_RIGHTBRACE,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const LCBRACE: KeyStroke = KeyStroke {
    modifiers: hid_codes::KEY_MOD_LSHIFT,
    hid_code: hid_codes::KEY_LEFTBRACE,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const RCBRACE: KeyStroke = KeyStroke {
    modifiers: hid_codes::KEY_MOD_LSHIFT,
    hid_code: hid_codes::KEY_RIGHTBRACE,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const BSLASH: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_BACKSLASH,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const UPCARROT: KeyStroke = KeyStroke {
    modifiers: KEY_MOD_LSHIFT,
    hid_code: hid_codes::KEY_6,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const PLUS: KeyStroke = KeyStroke {
    modifiers: hid_codes::KEY_MOD_LSHIFT,
    hid_code: hid_codes::KEY_EQUAL,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const HYPHEN: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_MINUS,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
pub const UNDERSCORE: KeyStroke = KeyStroke {
    modifiers: hid_codes::KEY_MOD_LSHIFT,
    hid_code: hid_codes::KEY_MINUS,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
};
