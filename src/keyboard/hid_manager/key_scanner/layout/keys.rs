#![allow(dead_code)]

use self::hid_codes::KEY_MOD_LSHIFT;

mod hid_codes;

pub type Macro<'a> = &'a [MacroGroup<'a>];

#[derive(Copy, Clone)]
pub enum KeyPress<'a> {
    Macro(Macro<'a>),
    Single(KeyStroke),
    Layer(Layer),
}

impl<'a> EqKeyPress for KeyPress<'a> {
    fn eq(&self, other: &KeyPress) -> bool {
        match other {
            KeyPress::Macro(other) => other.eq(self),
            KeyPress::Single(other) => other.eq(self),
            KeyPress::Layer(other) => other.eq(self),
        }
    }
}

#[derive(Copy, Clone)]
pub struct MacroGroup<'a> {
    pub delay_ms: u16,
    pub keystrokes: &'a [MacroPress],
}

impl<'a> EqKeyPress for Macro<'a> {
    fn eq(&self, other: &KeyPress) -> bool {
        match other {
            KeyPress::Macro(other) => {
                if self.len() != other.len() {
                    return false;
                }
                for i in 0..other.len() {
                    if !self[i]._eq(&other[i]) {
                        return false;
                    }
                }
                return true;
            }
            _ => false,
        }
    }
}

impl<'a> MacroGroup<'a> {
    fn _eq(self, other: &MacroGroup) -> bool {
        if self.keystrokes.len() != other.keystrokes.len() || self.delay_ms != other.delay_ms {
            return false;
        }
        for i in 0..self.keystrokes.len() {
            if !self.keystrokes[i]._eq(&other.keystrokes[i]) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Copy, Clone)]
pub struct MacroPress {
    pub delay_ms: u16,
    pub stroke: KeyStroke,
}

impl MacroPress {
    fn _eq(&self, other: &MacroPress) -> bool {
        self.stroke._eq(&other.stroke) && self.delay_ms == other.delay_ms
    }
}

#[derive(Copy, Clone)]
pub struct Layer {
    pub layer: usize,
    pub toggle: bool,
}

impl EqKeyPress for Layer {
    fn eq(&self, other: &KeyPress) -> bool {
        match other {
            KeyPress::Layer(other) => self._eq(other),
            _ => false,
        }
    }
}

impl Layer {
    fn _eq(&self, other: &Layer) -> bool {
        self.layer == other.layer && self.toggle == other.toggle
    }
}

pub trait EqKeyPress {
    fn eq(&self, other: &KeyPress) -> bool;
}

///Defines what happens when a key is pressed. Keys which are used to switch layers must have
///is_layer set to true. Keys with is_layer == true will ignore modifiers and hid_code.
///
///modifiers: modifiers sent when this key is pressed
///hid_code: hid_code sent when this key is pressed
///clearable: allows the mod to be cleared if a keystroke with no mod is pressed concurrently
#[derive(Copy, Clone)]
pub struct KeyStroke {
    pub modifiers: u8,
    pub hid_code: u8,
    pub clearable: bool,
}

impl EqKeyPress for KeyStroke {
    fn eq(&self, other: &KeyPress) -> bool {
        match other {
            KeyPress::Single(s) => self._eq(s),
            _ => false,
        }
    }
}

impl KeyStroke {
    fn _eq(&self, other: &KeyStroke) -> bool {
        self.modifiers == other.modifiers
            && self.hid_code == other.hid_code
            && self.clearable == other.clearable
    }

    pub fn is_none(self) -> bool {
        self.modifiers == 0 && self.hid_code == 0
    }
}

pub const NONE: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_NONE,
    clearable: false,
});
pub const OVER: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_ERR_OVF,
    clearable: true,
});
pub const A: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_A,
    clearable: true,
});
pub const B: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_B,
    clearable: true,
});
pub const C: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_C,
    clearable: true,
});
pub const D: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_D,
    clearable: true,
});
pub const E: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_E,
    clearable: true,
});
pub const F: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_F,
    clearable: true,
});
pub const G: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_G,
    clearable: true,
});
pub const H: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_H,
    clearable: true,
});
pub const I: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_I,
    clearable: true,
});
pub const J: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_J,
    clearable: true,
});
pub const K: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_K,
    clearable: true,
});
pub const L: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_L,
    clearable: true,
});
pub const M: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_M,
    clearable: true,
});
pub const N: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_N,
    clearable: true,
});
pub const O: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_O,
    clearable: true,
});
pub const P: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_P,
    clearable: true,
});
pub const Q: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_Q,
    clearable: true,
});
pub const R: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_R,
    clearable: true,
});
pub const S: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_S,
    clearable: true,
});
pub const T: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_T,
    clearable: true,
});
pub const U: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_U,
    clearable: true,
});
pub const V: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_V,
    clearable: true,
});
pub const W: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_W,
    clearable: true,
});
pub const X: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_X,
    clearable: true,
});
pub const Y: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_Y,
    clearable: true,
});
pub const Z: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_Z,
    clearable: true,
});

pub const TAB: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_TAB,
    clearable: true,
});

pub const APOSTROPHE: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_APOSTROPHE,
    clearable: true,
});

pub const COMMA: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_COMMA,
    clearable: true,
});

pub const PERIOD: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_DOT,
    clearable: true,
});

pub const BACKSPACE: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_BACKSPACE,
    clearable: true,
});

pub const DELETE: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_DELETE,
    clearable: true,
});

pub const ESC: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_ESC,
    clearable: true,
});

pub const FSLASH: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_SLASH,
    clearable: true,
});

pub const LSHIFT: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: hid_codes::KEY_MOD_LSHIFT,
    hid_code: hid_codes::KEY_NONE,
    clearable: false,
});

pub const LCTRL: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: hid_codes::KEY_MOD_LCTRL,
    hid_code: hid_codes::KEY_NONE,
    clearable: false,
});

pub const LMETA: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: hid_codes::KEY_MOD_LMETA,
    hid_code: hid_codes::KEY_NONE,
    clearable: false,
});

pub const LALT: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: hid_codes::KEY_MOD_LALT,
    hid_code: hid_codes::KEY_NONE,
    clearable: false,
});

pub const LEFT: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_LEFT,
    clearable: true,
});

pub const RIGHT: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_RIGHT,
    clearable: true,
});

pub const UP: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_UP,
    clearable: true,
});

pub const DOWN: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_DOWN,
    clearable: true,
});

pub const SCOLON: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_SEMICOLON,
    clearable: true,
});

pub const SPACE: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_SPACE,
    clearable: true,
});

pub const END: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_END,
    clearable: true,
});

pub const HOME: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_HOME,
    clearable: true,
});

pub const ENTER: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_ENTER,
    clearable: true,
});
pub const PGUP: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_PAGEUP,
    clearable: true,
});
pub const PGDN: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_PAGEDOWN,
    clearable: true,
});
pub const RCTRL: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_RIGHTCTRL,
    clearable: false,
});
pub const RALT: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_RIGHTALT,
    clearable: false,
});

pub const LAYER1: KeyPress = KeyPress::Layer(Layer {
    layer: 1,
    toggle: false,
});

pub const LAYER2: KeyPress = KeyPress::Layer(Layer {
    layer: 2,
    toggle: false,
});

// Layer for games
pub const T_LAYER3: KeyPress = KeyPress::Layer(Layer {
    layer: 3,
    toggle: true,
});
pub const F1: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_F1,
    clearable: true,
});
pub const F2: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_F2,
    clearable: true,
});
pub const F3: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_F3,
    clearable: true,
});
pub const F4: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_F4,
    clearable: true,
});
pub const F5: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_F5,
    clearable: true,
});
pub const F6: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_F6,
    clearable: true,
});
pub const F7: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_F7,
    clearable: true,
});
pub const F8: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_F8,
    clearable: true,
});
pub const F9: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_F9,
    clearable: true,
});
pub const F10: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_F10,
    clearable: true,
});
pub const F11: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_F11,
    clearable: true,
});
pub const F12: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_F12,
    clearable: true,
});
pub const N0: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_KP0,
    clearable: true,
});
pub const N1: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_KP1,
    clearable: true,
});
pub const N2: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_KP2,
    clearable: true,
});
pub const N3: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_KP3,
    clearable: true,
});
pub const N4: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_KP4,
    clearable: true,
});
pub const N5: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_KP5,
    clearable: true,
});
pub const N6: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_KP6,
    clearable: true,
});
pub const N7: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_KP7,
    clearable: true,
});
pub const N8: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_KP8,
    clearable: true,
});
pub const N9: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_KP9,
    clearable: true,
});
pub const NSUB: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_KPMINUS,
    clearable: true,
});
pub const NPLUS: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_KPPLUS,
    clearable: true,
});
pub const NMUL: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_KPASTERISK,
    clearable: true,
});
pub const NDIV: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_KPSLASH,
    clearable: true,
});
pub const NDOT: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_KPDOT,
    clearable: true,
});
pub const K0: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_0,
    clearable: true,
});
pub const K1: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_1,
    clearable: true,
});
pub const K2: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_2,
    clearable: true,
});
pub const K3: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_3,
    clearable: true,
});
pub const K4: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_4,
    clearable: true,
});
pub const K5: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_5,
    clearable: true,
});
pub const K6: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_6,
    clearable: true,
});
pub const K7: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_7,
    clearable: true,
});
pub const K8: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_8,
    clearable: true,
});
pub const K9: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_9,
    clearable: true,
});
pub const BACKTICK: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_GRAVE,
    clearable: true,
});
pub const TILDE: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: hid_codes::KEY_MOD_LSHIFT,
    hid_code: hid_codes::KEY_GRAVE,
    clearable: true,
});
pub const EXCLAMATION: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: hid_codes::KEY_MOD_LSHIFT,
    hid_code: hid_codes::KEY_1,
    clearable: true,
});
pub const QUESTION: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: hid_codes::KEY_MOD_LSHIFT,
    hid_code: hid_codes::KEY_SLASH,
    clearable: true,
});
pub const LPAREN: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: hid_codes::KEY_MOD_LSHIFT,
    hid_code: hid_codes::KEY_9,
    clearable: true,
});
pub const RPAREN: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: hid_codes::KEY_MOD_LSHIFT,
    hid_code: hid_codes::KEY_0,
    clearable: true,
});
pub const ASTERISK: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: hid_codes::KEY_MOD_LSHIFT,
    hid_code: hid_codes::KEY_8,
    clearable: true,
});
pub const ATSIGN: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: hid_codes::KEY_MOD_LSHIFT,
    hid_code: hid_codes::KEY_2,
    clearable: true,
});
pub const POUND: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: hid_codes::KEY_MOD_LSHIFT,
    hid_code: hid_codes::KEY_3,
    clearable: true,
});
pub const AND: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: hid_codes::KEY_MOD_LSHIFT,
    hid_code: hid_codes::KEY_7,
    clearable: true,
});
pub const DOLLAR: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: hid_codes::KEY_MOD_LSHIFT,
    hid_code: hid_codes::KEY_4,
    clearable: true,
});
pub const PERCENT: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: hid_codes::KEY_MOD_LSHIFT,
    hid_code: hid_codes::KEY_5,
    clearable: true,
});
pub const EQUAL: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_EQUAL,
    clearable: true,
});
pub const LBRACKET: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_LEFTBRACE,
    clearable: true,
});
pub const RBRACKET: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_RIGHTBRACE,
    clearable: true,
});
pub const LCBRACE: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: hid_codes::KEY_MOD_LSHIFT,
    hid_code: hid_codes::KEY_LEFTBRACE,
    clearable: true,
});
pub const RCBRACE: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: hid_codes::KEY_MOD_LSHIFT,
    hid_code: hid_codes::KEY_RIGHTBRACE,
    clearable: true,
});
pub const BSLASH: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_BACKSLASH,
    clearable: true,
});
pub const UPCARROT: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: KEY_MOD_LSHIFT,
    hid_code: hid_codes::KEY_6,
    clearable: true,
});
pub const PLUS: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: hid_codes::KEY_MOD_LSHIFT,
    hid_code: hid_codes::KEY_EQUAL,
    clearable: true,
});
pub const HYPHEN: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_MINUS,
    clearable: true,
});
pub const UNDERSCORE: KeyPress = KeyPress::Single(KeyStroke {
    modifiers: hid_codes::KEY_MOD_LSHIFT,
    hid_code: hid_codes::KEY_MINUS,
    clearable: true,
});
