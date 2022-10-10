pub mod layout;

use layout::{
    keys::{self, EqKeyPress},
    Layer,
};

pub struct KeyTracker<'a> {
    pressed_keys: Layer<'a>,
    current_layer: usize,
    default_layer: usize,
    pressed_layers: [usize; layout::LAYOUT.len()],
}

impl<'a> KeyTracker<'a> {
    pub fn new() -> Self {
        return KeyTracker {
            pressed_keys: [[[keys::NONE; layout::SECTION_COLS]; layout::SECTION_ROWS];
                layout::N_SECTIONS],
            current_layer: 0,
            default_layer: 0,
            pressed_layers: [0xFF; layout::LAYOUT.len()],
        };
    }

    fn _press_key(&mut self, section: usize, col: usize, row: usize) -> keys::KeyPress {
        if section < layout::N_SECTIONS && col < layout::SECTION_COLS && row < layout::SECTION_ROWS
        {
            let key = layout::get_key(self.current_layer, section, col, row);
            self.pressed_keys[section][row][col] = key;

            // process layer keys
            match key {
                keys::KeyPress::Layer(k) => {
                    if k.toggle {
                        if self.default_layer == k.layer {
                            self.default_layer = 0;
                        } else {
                            self.default_layer = k.layer;
                        }
                    } else {
                        self.current_layer = k.layer;
                        self.pressed_layers.rotate_right(1);
                        self.pressed_layers[0] = k.layer;
                    }
                    return keys::NONE;
                }
                _ => return key,
            }
        }
        return keys::NONE;
    }

    fn _release_key(&mut self, section: usize, col: usize, row: usize) -> keys::KeyPress {
        if section < layout::N_SECTIONS && col < layout::SECTION_COLS && row < layout::SECTION_ROWS
        {
            let key = self.pressed_keys[section][row][col];
            self.pressed_keys[section][row][col] = keys::NONE;

            // process layer keys
            match key {
                keys::KeyPress::Layer(k) => {
                    if !k.toggle {
                        for i in 0..layout::LAYOUT.len() {
                            if self.pressed_layers[i] == k.layer {
                                for j in i..layout::LAYOUT.len() - 1 {
                                    self.pressed_layers[j] = self.pressed_layers[j + 1];
                                }
                                self.pressed_layers[self.pressed_layers.len() - 1] = 0xFF;
                                break;
                            }
                        }
                        if self.pressed_layers[0] == 0xFF {
                            self.current_layer = self.default_layer;
                        } else {
                            self.current_layer = self.pressed_layers[0];
                        }
                    }
                }
                _ => return key,
            }
        }
        return keys::NONE;
    }

    pub fn get_key(&self, section: usize, row: usize, column: usize) -> keys::KeyPress {
        return self.pressed_keys[section][row][column];
    }

    pub fn process_column(
        &mut self,
        section: usize,
        column: layout::Column,
        column_number: usize,
    ) -> [[keys::KeyPress; layout::SECTION_ROWS]; 2] {
        let mut hids = [[keys::NONE; layout::SECTION_ROWS]; 2];
        for row in 0..layout::SECTION_ROWS {
            // if key is down
            if (0x1 << row) & column > 0 {
                // and was up before
                if self.get_key(section, row, column_number).eq(&keys::NONE) {
                    hids[0][row] = self._press_key(section, column_number, row);
                }
            // if key is up
            } else {
                // and was down before
                if !self.pressed_keys[section][row][column_number].eq(&keys::NONE) {
                    hids[1][row] = self._release_key(section, column_number, row)
                }
            }
        }
        return hids;
    }
}
