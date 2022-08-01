pub mod key_scanner;

use key_scanner::layout::keys;
use usbd_hid::descriptor::KeyboardReport;

pub struct HidManager {
    modifier: u8,
    keys: [u8; 6],
}

impl HidManager {
    pub fn new() -> Self {
        return HidManager {
            modifier: 0,
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
                let mut j = i;
                while j < 5 {
                    self.keys[j] = self.keys[j + 1];
                    if self.keys[j] == 0 {
                        return;
                    }
                    j += 1;
                }
                break;
            }
        }
    }

    pub fn press_modifier(&mut self, m: u8) {
        if m < 8 {
            self.modifier |= 1 << m;
        }
    }

    pub fn release_modifier(&mut self, m: u8) {
        if m < 8 {
            self.modifier &= !(1 << m);
        }
    }

    pub fn report(&self) -> KeyboardReport {
        return KeyboardReport {
            modifier: self.modifier,
            reserved: 0,
            leds: 0,
            keycodes: self.keys,
        };
    }

    pub fn process_key(&mut self, key: keys::KeyStroke, down: bool) {
        if key.is_layer {
            return;
        }
        if down {
            self.press_key(key.hid_code);
            self.press_modifier(key.modifiers);
        } else {
            self.release_key(key.hid_code);
            self.release_modifier(key.modifiers);
        }
    }
}
