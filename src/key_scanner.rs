// Number of physical sections of the keyboard, polled via i2c
const N_SECTIONS: u8 = 2;

// Size in width x height
const SECTION_SIZES: [[u8; 2]; N_SECTIONS] = [[8, 8], [8, 8]];

// I2C addresses of each section
const SECTION_I2C_ADDRESSES: [u8; N_SECTIONS] = [0x27, 0x28];

pub struct KeyScanner {}
