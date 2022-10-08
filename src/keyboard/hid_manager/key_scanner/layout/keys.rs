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

pub const NONE: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_NONE,
    clearable: false,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const OVER: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_ERR_OVF,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const A: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_A,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const B: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_B,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const C: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_C,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const D: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_D,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const E: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_E,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const F: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_F,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const G: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_G,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const H: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_H,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const I: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_I,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const J: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_J,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const K: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_K,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const L: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_L,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const M: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_M,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const N: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_N,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const O: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_O,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const P: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_P,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const Q: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_Q,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const R: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_R,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const S: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_S,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const T: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_T,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const U: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_U,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const V: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_V,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const W: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_W,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const X: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_X,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const Y: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_Y,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const Z: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_Z,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});

pub const TAB: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_TAB,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});

pub const APOSTROPHE: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_APOSTROPHE,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});

pub const COMMA: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_COMMA,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});

pub const PERIOD: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_DOT,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});

pub const BACKSPACE: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_BACKSPACE,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});

pub const DELETE: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_DELETE,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});

pub const ESC: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_ESC,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});

pub const FSLASH: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_SLASH,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});

pub const LSHIFT: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: hid_codes::KEY_MOD_LSHIFT,
    hid_code: hid_codes::KEY_NONE,
    clearable: false,
    is_layer: false,
    layer: 0,
    toggle: false,
});

pub const LCTRL: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: hid_codes::KEY_MOD_LCTRL,
    hid_code: hid_codes::KEY_NONE,
    clearable: false,
    is_layer: false,
    layer: 0,
    toggle: false,
});

pub const LMETA: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: hid_codes::KEY_MOD_LMETA,
    hid_code: hid_codes::KEY_NONE,
    clearable: false,
    is_layer: false,
    layer: 0,
    toggle: false,
});

pub const LALT: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: hid_codes::KEY_MOD_LALT,
    hid_code: hid_codes::KEY_NONE,
    clearable: false,
    is_layer: false,
    layer: 0,
    toggle: false,
});

pub const LEFT: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_LEFT,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});

pub const RIGHT: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_RIGHT,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});

pub const UP: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_UP,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});

pub const DOWN: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_DOWN,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});

pub const SCOLON: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_SEMICOLON,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});

pub const SPACE: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_SPACE,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});

pub const END: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_END,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});

pub const HOME: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_HOME,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});

pub const ENTER: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_ENTER,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const PGUP: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_PAGEUP,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const PGDN: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_PAGEDOWN,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const RCTRL: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_RIGHTCTRL,
    clearable: false,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const RALT: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_RIGHTALT,
    clearable: false,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const LAYER1: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_NONE,
    clearable: true,
    is_layer: true,
    layer: 1,
    toggle: false,
});

pub const LAYER2: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_NONE,
    clearable: true,
    is_layer: true,
    layer: 2,
    toggle: false,
});

// Layer for games
pub const T_LAYER3: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_NONE,
    clearable: true,
    is_layer: true,
    layer: 3,
    toggle: true,
});
pub const F1: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_F1,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const F2: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_F2,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const F3: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_F3,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const F4: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_F4,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const F5: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_F5,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const F6: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_F6,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const F7: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_F7,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const F8: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_F8,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const F9: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_F9,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const F10: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_F10,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const F11: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_F11,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const F12: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_F12,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const N0: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_KP0,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const N1: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_KP1,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const N2: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_KP2,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const N3: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_KP3,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const N4: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_KP4,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const N5: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_KP5,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const N6: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_KP6,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const N7: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_KP7,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const N8: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_KP8,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const N9: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_KP9,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const NSUB: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_KPMINUS,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const NPLUS: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_KPPLUS,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const NMUL: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_KPASTERISK,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const NDIV: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_KPSLASH,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const NDOT: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_KPDOT,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const K0: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_0,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const K1: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_1,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const K2: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_2,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const K3: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_3,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const K4: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_4,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const K5: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_5,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const K6: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_6,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const K7: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_7,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const K8: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_8,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const K9: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_9,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const BACKTICK: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_GRAVE,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const TILDE: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: hid_codes::KEY_MOD_LSHIFT,
    hid_code: hid_codes::KEY_GRAVE,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const EXCLAMATION: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: hid_codes::KEY_MOD_LSHIFT,
    hid_code: hid_codes::KEY_1,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const QUESTION: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: hid_codes::KEY_MOD_LSHIFT,
    hid_code: hid_codes::KEY_SLASH,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const LPAREN: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: hid_codes::KEY_MOD_LSHIFT,
    hid_code: hid_codes::KEY_9,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const RPAREN: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: hid_codes::KEY_MOD_LSHIFT,
    hid_code: hid_codes::KEY_0,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const ASTERISK: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: hid_codes::KEY_MOD_LSHIFT,
    hid_code: hid_codes::KEY_8,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const ATSIGN: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: hid_codes::KEY_MOD_LSHIFT,
    hid_code: hid_codes::KEY_2,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const POUND: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: hid_codes::KEY_MOD_LSHIFT,
    hid_code: hid_codes::KEY_3,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const AND: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: hid_codes::KEY_MOD_LSHIFT,
    hid_code: hid_codes::KEY_7,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const DOLLAR: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: hid_codes::KEY_MOD_LSHIFT,
    hid_code: hid_codes::KEY_4,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const PERCENT: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: hid_codes::KEY_MOD_LSHIFT,
    hid_code: hid_codes::KEY_5,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const EQUAL: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_EQUAL,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const LBRACKET: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_LEFTBRACE,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const RBRACKET: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_RIGHTBRACE,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const LCBRACE: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: hid_codes::KEY_MOD_LSHIFT,
    hid_code: hid_codes::KEY_LEFTBRACE,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const RCBRACE: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: hid_codes::KEY_MOD_LSHIFT,
    hid_code: hid_codes::KEY_RIGHTBRACE,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const BSLASH: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_BACKSLASH,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const UPCARROT: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: KEY_MOD_LSHIFT,
    hid_code: hid_codes::KEY_6,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const PLUS: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: hid_codes::KEY_MOD_LSHIFT,
    hid_code: hid_codes::KEY_EQUAL,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const HYPHEN: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_MINUS,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
pub const UNDERSCORE: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: hid_codes::KEY_MOD_LSHIFT,
    hid_code: hid_codes::KEY_MINUS,
    clearable: true,
    is_layer: false,
    layer: 0,
    toggle: false,
});
