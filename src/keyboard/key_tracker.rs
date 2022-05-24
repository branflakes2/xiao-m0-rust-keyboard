const INDICES: [u8; 8] = [
    0b00000001, 0b00000010, 0b00000100, 0b00001000, 0b00010000, 0b00100000, 0b01000000, 0b10000000,
];

pub struct KeyTracker {
    keys: [u8; 16],
}

impl KeyTracker {
    pub fn new() -> Self {
        return KeyTracker { keys: [0; 16] };
    }

    /// Given a column and the state of keys in the column, returns which keys were raised and
    /// which keys were lowered (if any)
    /// column: which column the keys belong to
    /// keys: a u8 where each bit represents the state of a key in a column: 1 is lowered
    /// returns: 2 len 8 u8 slices - first is indices which were raised, second is indices which
    /// were lowered. stop parsing after an index with 255 is reached.
    pub fn check_column_changes(&mut self, column: usize, keys: u8) -> [[usize; 8]; 2] {
        if column > 15 {
            return [
                [255, 255, 255, 255, 255, 255, 255, 255],
                [255, 255, 255, 255, 255, 255, 255, 255],
            ];
        }
        let mut raised: [usize; 8] = [255, 255, 255, 255, 255, 255, 255, 255];
        let mut lowered: [usize; 8] = [255, 255, 255, 255, 255, 255, 255, 255];
        let mut raised_i = 0;
        let mut lowered_i = 0;
        let changed = keys ^ self.keys[column];
        for i in 0..7 {
            if changed & INDICES[i] > 0 {
                if keys & INDICES[i] > 0 {
                    raised[raised_i] = i;
                    raised_i += 1;
                } else {
                    lowered[lowered_i] = i;
                    lowered_i += 1;
                }
            }
        }
        self.keys[column] = keys;
        return [raised, lowered];
    }
}
