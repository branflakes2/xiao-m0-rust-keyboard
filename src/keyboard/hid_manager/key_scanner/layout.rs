pub mod keys;

use keys::KeyStroke;

// Number of physical sections of the keyboard, polled via i2c
pub const N_SECTIONS: usize = 2;

// Used to allocate layers - TODO in a macro at some point to save space
pub const SECTION_ROWS: usize = 8;
pub const SECTION_COLS: usize = 8;

// I2C addresses of each section
pub const SECTION_I2C_ADDRESSES: [u8; N_SECTIONS] = [0x26, 0x27];

pub type Layer = [[[KeyStroke; SECTION_COLS]; SECTION_ROWS]; N_SECTIONS];
pub type Column = u8; // set this depending on the size that's read from
                      // io expander

#[rustfmt::skip]
mod unformatted {
    use crate::keyboard::layout::{SECTION_COLS, SECTION_ROWS, N_SECTIONS, Layer, keys};
    pub const LAYOUT_MAP: [[[[usize; 2]; SECTION_COLS]; SECTION_ROWS]; N_SECTIONS] = [
        // Left Section
        [
            [[0, 5], [0, 4], [0, 3], [0, 2], [0, 1], [0, 0], [7, 7], [7, 7]],
            [[1, 5], [1, 4], [1, 3], [1, 2], [1, 1], [1, 0], [7, 7], [7, 7]],
            [[2, 5], [2, 4], [2, 3], [2, 2], [2, 1], [2, 0], [7, 7], [7, 7]],
            [[3, 5], [3, 4], [3, 3], [3, 2], [7, 7], [7, 7], [7, 7], [7, 7]],
            [[4, 5], [4, 4], [7, 7], [7, 7], [7, 7], [7, 7], [7, 7], [7, 7]],
            [[5, 5], [5, 4], [7, 7], [7, 7], [7, 7], [7, 7], [7, 7], [7, 7]],
            [[7, 7], [7, 7], [7, 7], [7, 7], [7, 7], [7, 7], [7, 7], [7, 7]],
            [[7, 7], [7, 7], [7, 7], [7, 7], [7, 7], [7, 7], [7, 7], [7, 7]],
        ],
        // rows top to bottom 674532
        // columns left to right efcdab 452301
        [
            [[7, 7], [7, 7], [7, 7], [7, 7], [7, 7], [7, 7], [7, 7], [7, 7]],
            [[7, 7], [7, 7], [7, 7], [7, 7], [7, 7], [7, 7], [7, 7], [7, 7]],
            [[5, 4], [5, 5], [5, 2], [5, 3], [5, 0], [5, 1], [5, 6], [5, 7]],
            [[4, 4], [4, 5], [4, 2], [4, 3], [4, 0], [4, 1], [4, 6], [4, 7]],
            [[2, 4], [2, 5], [2, 2], [2, 3], [2, 0], [2, 1], [2, 6], [2, 7]],
            [[3, 4], [3, 5], [3, 2], [3, 3], [3, 0], [3, 1], [3, 6], [3, 7]],
            [[0, 4], [0, 5], [0, 2], [0, 3], [0, 0], [0, 1], [7, 7], [7, 7]],
            [[1, 4], [1, 5], [1, 2], [1, 3], [1, 0], [1, 1], [1, 6], [1, 7]],
        ]
    ];

    pub const LAYOUT: [Layer; 2] = [
        // Layer 0
        [
            // Left Section
            [
                [keys::TAB, keys::APOSTROPHE, keys::COMMA, keys::PERIOD, keys::P, keys::Y, keys::NONE, keys::NONE],
                [keys::ESC, keys::A, keys::O, keys::E, keys::U, keys::I, keys::NONE, keys::NONE],
                [keys::LSHIFT, keys::SCOLON, keys::Q, keys::J, keys::K, keys::X, keys::NONE, keys::NONE],
                [keys::NONE, keys::NONE, keys::LEFT, keys::RIGHT, keys::LAYER1, keys::SPACE, keys::NONE, keys::NONE],
                [keys::NONE, keys::NONE, keys::NONE, keys::NONE, keys::LALT, keys::LCTRL, keys::NONE, keys::NONE],
                [keys::NONE, keys::NONE, keys::NONE, keys::NONE, keys::HOME, keys::END, keys::NONE, keys::NONE],
                [keys::NONE, keys::NONE, keys::NONE, keys::NONE, keys::NONE, keys::NONE, keys::NONE, keys::NONE],
                [keys::NONE, keys::NONE, keys::NONE, keys::NONE, keys::NONE, keys::NONE, keys::NONE, keys::NONE],
            ],
            // Right Section
            [
                [keys::F, keys::G, keys::C, keys::R, keys::L, keys::BACKSPACE, keys::NONE, keys::NONE],
                [keys::D, keys::H, keys::T, keys::N, keys::S, keys::FSLASH, keys::NONE, keys::NONE],
                [keys::B, keys::M, keys::W, keys::V, keys::Z, keys::ENTER, keys::NONE, keys::NONE],
                [keys::SPACE, keys::LAYER2, keys::DOWN, keys::UP, keys::NONE, keys::NONE, keys::NONE, keys::NONE],
                [keys::PGUP, keys::RALT, keys::NONE, keys::NONE, keys::NONE, keys::NONE, keys::NONE, keys::NONE],
                [keys::PGDN, keys::RCTRL, keys::NONE, keys::NONE, keys::NONE, keys::NONE, keys::NONE, keys::NONE],
                [keys::NONE, keys::NONE, keys::NONE, keys::NONE, keys::NONE, keys::NONE, keys::NONE, keys::NONE],
                [keys::NONE, keys::NONE, keys::NONE, keys::NONE, keys::NONE, keys::NONE, keys::NONE, keys::NONE],
            ],
        ],
        // Layer 1
        [
            // Left Section
            [
                [keys::A, keys::B, keys::C, keys::D, keys::E, keys::F, keys::G, keys::H],
                [keys::I, keys::J, keys::K, keys::L, keys::M, keys::N, keys::O, keys::P],
                [keys::Q, keys::R, keys::S, keys::T, keys::U, keys::V, keys::W, keys::X],
                [keys::Y, keys::Z, keys::A, keys::B, keys::C, keys::D, keys::E, keys::F],
                [keys::G, keys::H, keys::I, keys::J, keys::K, keys::L, keys::M, keys::N],
                [keys::O, keys::P, keys::Q, keys::R, keys::S, keys::T, keys::U, keys::V],
                [keys::W, keys::X, keys::Y, keys::Z, keys::A, keys::B, keys::C, keys::D],
                [keys::E, keys::F, keys::G, keys::H, keys::I, keys::J, keys::K, keys::L],
            ],
            // Right Section
            [
                [keys::A, keys::B, keys::C, keys::D, keys::E, keys::F, keys::G, keys::H],
                [keys::A, keys::B, keys::C, keys::D, keys::E, keys::F, keys::G, keys::H],
                [keys::A, keys::B, keys::C, keys::D, keys::E, keys::F, keys::G, keys::H],
                [keys::A, keys::B, keys::C, keys::D, keys::E, keys::F, keys::G, keys::H],
                [keys::A, keys::B, keys::C, keys::D, keys::E, keys::F, keys::G, keys::H],
                [keys::A, keys::B, keys::C, keys::D, keys::E, keys::F, keys::G, keys::H],
                [keys::A, keys::B, keys::C, keys::D, keys::E, keys::F, keys::G, keys::H],
                [keys::A, keys::B, keys::C, keys::D, keys::E, keys::F, keys::G, keys::H],
            ],
        ],
    ];
}

pub use unformatted::*;

pub fn get_key(layer: usize, section: usize, col: usize, row: usize) -> KeyStroke {
    let mapped_stroke: [usize; 2] = LAYOUT_MAP[section][row][col];
    return LAYOUT[layer][section][mapped_stroke[0]][mapped_stroke[1]];
}
