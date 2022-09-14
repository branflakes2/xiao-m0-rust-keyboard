pub mod key_scanner;

use key_scanner::layout::keys;
use usbd_hid::descriptor::KeyboardReport;

pub struct HidManager {
    modifier: u8,
    clearable_modifier: u8,
    keys: [u8; 6],
    _old_keys: [u8; 6],
    _old_modifier: u8,
}

impl HidManager {
    pub fn new() -> Self {
        return HidManager {
            modifier: 0,
            clearable_modifier: 0,
            keys: [0, 0, 0, 0, 0, 0],
            _old_keys: [0, 0, 0, 0, 0, 0],
            _old_modifier: 0,
        };
    }

    pub fn press_key(&mut self, key: u8) {
        if key == 0x00 {
            return;
        }
        for i in 0..6 {
            if self.keys[i] == key {
                return;
            }
            if self.keys[i] == 0 {
                self.keys[i] = key;
                return;
            }
        }
    }

    fn release_key(&mut self, key: u8) {
        if key == 0 {
            return;
        }
        for i in 0..6 {
            if self.keys[i] == key {
                self.keys[i] = 0;
            }
        }
    }

    pub fn press_modifier(&mut self, m: u8, clearable: bool) -> bool {
        let old_m = self.modifier;
        self.clearable_modifier = 0;
        // if the modifier is marked as clearable
        if clearable {
            // and modifier is not already pressed
            if self.modifier & m == 0 {
                self.clearable_modifier = m;
            }
        } else {
            self.modifier |= m;
        }

        // report whether the modifier changed
        if self.modifier != old_m {
            return true;
        } else {
            return false;
        }
    }

    pub fn release_modifier(&mut self, m: u8, clearable: bool) {
        // only release clearable modifiers if it's not already pressed somewhere else
        if !clearable || self.modifier & m == 0 {
            self.modifier &= !m;
        }
        self.clearable_modifier = 0;
    }

    pub fn report(&mut self, which_reports: &mut u8) -> [KeyboardReport; 2] {
        *which_reports = 0;
        let new_modifier = self.modifier | self.clearable_modifier;
        let reports: [KeyboardReport; 2] = [
            KeyboardReport {
                modifier: new_modifier,
                reserved: 0,
                leds: 0,
                keycodes: self._old_keys,
            },
            KeyboardReport {
                modifier: new_modifier,
                reserved: 0,
                leds: 0,
                keycodes: self.keys,
            },
        ];
        if new_modifier != self._old_modifier {
            *which_reports |= 1;
            self._old_modifier = new_modifier;
        }
        if self.keys != self._old_keys {
            *which_reports |= 2;
            self._old_keys = self.keys;
        }
        return reports;
    }

    pub fn process_key(&mut self, key: keys::KeyStroke, down: bool) {
        if key.is_layer || key.eq(keys::NONE) {
            return;
        }
        if down {
            self.press_key(key.hid_code);
            self.press_modifier(key.modifiers, key.clearable);
        } else {
            self.release_key(key.hid_code);
            self.release_modifier(key.modifiers, key.clearable);
        }
    }
}
