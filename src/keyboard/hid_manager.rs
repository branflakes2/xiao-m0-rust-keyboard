pub mod key_scanner;

use key_scanner::layout::keys;
use usbd_hid::descriptor::KeyboardReport;

pub struct HidManager {
    modifier: u8,
    clearable_modifier: u8,
    keys: [u8; 6],
}

impl HidManager {
    pub fn new() -> Self {
        return HidManager {
            modifier: 0,
            clearable_modifier: 0,
            keys: [0, 0, 0, 0, 0, 0],
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

    pub fn press_modifier(&mut self, m: u8, clearable: bool) {
        let old_m = self.modifier;
        self.clearable_modifier = 0;
        if clearable {
            self.clearable_modifier = m;
        } else {
            self.modifier |= m;
        }
        if m != old_m {
            return true;
        } else {
            return false;
        }
    }

    pub fn release_modifier(&mut self, m: u8) {
        self.modifier &= !m;
    }

    pub fn report(&self) -> [KeyboardReport; 2] {
        return KeyboardReport {
            modifier: self.modifier | self.clearable_modifier,
            reserved: 0,
            leds: 0,
            keycodes: self.keys,
        };
    }

    pub fn process_key(&mut self, key: keys::KeyStroke, down: bool) -> bool {
        if key.is_layer || key.eq(keys::NONE) {
            return false;
        }
        if down {
            self.press_key(key.hid_code);
            self.press_modifier(key.modifiers, key.clearable);
        } else {
            self.release_key(key.hid_code);
            self.release_modifier(key.modifiers);
        }
        return true;
    }
}
