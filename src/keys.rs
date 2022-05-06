mod hid_codes;

struct KeyStroke {
    modifiers: u8,
    hid_code: u8,
}

const A: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_A,
};
const B: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_B,
};
const C: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_C,
};
const D: KeyStroke = KeyStroke {
    modifiers: 0,
    hid_code: hid_codes::KEY_D,
};
