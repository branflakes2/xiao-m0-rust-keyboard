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
        [
            [[0, 5], [0, 4], [0, 3], [0, 2], [0, 1], [0, 0], [0, 6], [0, 7]],
            [[1, 0], [1, 1], [1, 2], [1, 3], [1, 4], [1, 5], [1, 6], [1, 7]],
            [[2, 0], [2, 1], [2, 2], [2, 3], [2, 4], [2, 5], [2, 6], [2, 7]],
            [[3, 0], [3, 1], [3, 2], [3, 3], [3, 4], [3, 5], [3, 6], [3, 7]],
            [[4, 0], [4, 1], [4, 2], [4, 3], [4, 4], [4, 5], [4, 6], [4, 7]],
            [[5, 0], [5, 1], [5, 2], [5, 3], [5, 4], [5, 5], [5, 6], [5, 7]],
            [[6, 0], [6, 1], [6, 2], [6, 3], [6, 4], [6, 5], [6, 6], [6, 7]],
            [[7, 0], [7, 1], [7, 2], [7, 3], [7, 4], [7, 5], [7, 6], [7, 7]],
        ]
    ];
    pub const LAYOUT: [Layer; 2] = [
        // Layer 0
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
                [keys::I, keys::J, keys::K, keys::L, keys::M, keys::N, keys::O, keys::P],
                [keys::Q, keys::R, keys::S, keys::T, keys::U, keys::V, keys::W, keys::X],
                [keys::Y, keys::Z, keys::A, keys::B, keys::C, keys::D, keys::E, keys::F],
                [keys::G, keys::H, keys::I, keys::J, keys::K, keys::L, keys::M, keys::N],
                [keys::O, keys::P, keys::Q, keys::R, keys::S, keys::T, keys::U, keys::V],
                [keys::W, keys::X, keys::Y, keys::Z, keys::A, keys::B, keys::C, keys::D],
                [keys::E, keys::F, keys::G, keys::H, keys::I, keys::J, keys::K, keys::L],
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
                [keys::I, keys::J, keys::K, keys::L, keys::M, keys::N, keys::O, keys::P],
                [keys::Q, keys::R, keys::S, keys::T, keys::U, keys::V, keys::W, keys::X],
                [keys::Y, keys::Z, keys::A, keys::B, keys::C, keys::D, keys::E, keys::F],
                [keys::G, keys::H, keys::I, keys::J, keys::K, keys::L, keys::M, keys::N],
                [keys::O, keys::P, keys::Q, keys::R, keys::S, keys::T, keys::U, keys::V],
                [keys::W, keys::X, keys::Y, keys::Z, keys::A, keys::B, keys::C, keys::D],
                [keys::E, keys::F, keys::G, keys::H, keys::I, keys::J, keys::K, keys::L],
            ],
        ],
    ];
}

pub use unformatted::*;

pub fn get_key(layer: usize, section: usize, col: usize, row: usize) -> KeyStroke {
    let mapped_stroke: [usize; 2] = LAYOUT_MAP[section][row][col];
    return LAYOUT[layer][section][mapped_stroke[0]][mapped_stroke[1]];
}
