use usbd_hid::descriptor::KeyboardReport;

pub struct HidManager {
    modifier: u8,
    hid_tracker: [[u8; 8]; 16],
    keys: [u8; 6],
}

impl HidManager {
    pub fn new() -> Self {
        return HidManager {
            modifier: 0,
            hid_tracker: [[0; 8]; 16],
            keys: [0, 0, 0, 0, 0, 0],
        };
    }

    pub fn press_key(&mut self, column: usize, row: usize, key: u8) {
        if column > 15 || row > 7 || key == 0 {
            return;
        }
        self.hid_tracker[column][row] = key;
        if key == 0 {
            return;
        }
        for i in 0..5 {
            if self.keys[i] == key {
                return;
            }
            if self.keys[i] == 0 {
                self.keys[i] = key;
            }
        }
    }

    fn _release_key(mut self, key: u8) {
        if key > 0 {
            for i in 0..5 {
                if self.keys[i] == key {
                    while i < 5 && self.keys[i + 1] != 0 {
                        self.keys[i] = self.keys[i];
                    }
                }
            }
        }
    }

    pub fn release_key(mut self, column: usize, row: usize) {
        if column > 15 || row > 7 {
            return;
        }
        let hid_code = self.hid_tracker[column][row];
        if hid_code == 0 {
            return;
        }
        self._release_key(hid_code);
    }

    pub fn press_modifier(mut self, m: u8) {
        if m < 7 {
            self.modifier |= 1 << m;
        }
    }

    pub fn release_modifier(mut self, m: u8) {
        if m < 7 {
            self.modifier &= !(1 << m);
        }
    }

    pub fn report(self) -> KeyboardReport {
        return KeyboardReport {
            modifier: self.modifier,
            reserved: 0,
            leds: 0,
            keycodes: self.keys,
        };
    }
}
