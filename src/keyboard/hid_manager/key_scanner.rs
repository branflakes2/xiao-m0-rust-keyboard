pub mod layout;

use layout::{keys, DebounceLayer, Layer};

pub struct KeyTracker {
    debounce_tracker: DebounceLayer,
    pressed_keys: Layer,
    debounce_sense: usize,
    current_layer: usize,
    default_layer: usize,
    pressed_layers: [usize; layout::LAYOUT.len()],
}

impl KeyTracker {
    pub fn new(debounce_sense: usize) -> Self {
        return KeyTracker {
            debounce_tracker: [[[0; layout::SECTION_COLS]; layout::SECTION_ROWS];
                layout::N_SECTIONS],
            pressed_keys: [[[keys::NONE; layout::SECTION_COLS]; layout::SECTION_ROWS];
                layout::N_SECTIONS],
            debounce_sense,
            current_layer: 0,
            default_layer: 0,
            pressed_layers: [0xFF; layout::LAYOUT.len()],
        };
    }

    fn _press_key(&mut self, section: usize, col: usize, row: usize) -> keys::KeyStroke {
        if section < layout::N_SECTIONS && col < layout::SECTION_COLS && row < layout::SECTION_ROWS
        {
            let key = layout::get_key(self.current_layer, section, col, row);
            self.pressed_keys[section][row][col] = key;
            if key.is_layer {
                if key.toggle {
                    if self.default_layer == key.layer {
                        self.default_layer = 0;
                    } else {
                        self.default_layer = key.layer;
                    }
                } else {
                    self.current_layer = key.layer;
                    self.pressed_layers.rotate_right(1);
                    self.pressed_layers[0] = key.layer;
                }
            } else {
                return key;
            }
        }
        return keys::NONE;
    }

    fn _release_key(&mut self, section: usize, col: usize, row: usize) -> keys::KeyStroke {
        if section < layout::N_SECTIONS && col < layout::SECTION_COLS && row < layout::SECTION_ROWS
        {
            let key = self.pressed_keys[section][row][col];
            self.pressed_keys[section][row][col] = keys::NONE;
            if key.is_layer {
                if !key.toggle {
                    // remove the layer from the list of pressed layers
                    for i in 0..layout::LAYOUT.len() {
                        if self.pressed_layers[i] == key.layer {
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
            } else {
                return key;
            }
        }
        return keys::NONE;
    }

    pub fn process_column(
        &mut self,
        section: usize,
        column: layout::Column,
        column_number: usize,
    ) -> [[keys::KeyStroke; layout::SECTION_ROWS]; 2] {
        let mut hids = [[keys::NONE; layout::SECTION_ROWS]; 2];
        //if section >= layout::N_SECTIONS || row >= layout::SECTION_ROWS {
        //    return hids;
        //}
        for row in 0..layout::SECTION_ROWS {
            if (0x1 << row) & column > 0 {
                if self.pressed_keys[section][row][column_number].eq(keys::NONE) {
                    self.debounce_tracker[section][row][column_number] += 1;
                    if self.debounce_tracker[section][row][column_number] == self.debounce_sense {
                        hids[0][row] = self._press_key(section, column_number, row);
                    }
                }
            } else {
                if !self.pressed_keys[section][row][column_number].eq(keys::NONE) {
                    self.debounce_tracker[section][row][column_number] = 0;
                    hids[1][row] = self._release_key(section, column_number, row)
                }
            }
        }
        return hids;
    }
}
