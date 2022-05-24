pub mod keys;

use keys::KeyStroke;

type Layer = [[KeyStroke; 8]; 16];

pub struct Layout<'a> {
    layers: &'a [&'a Layer; 20],
    default_layer: usize,
    current_layer: usize,
    layer_keys: [[usize; 3]; 20],
}

impl<'a> Layout<'a> {
    pub fn new(layers: &'a [&'a Layer; 20]) -> Self {
        return Layout {
            layers,
            default_layer: 0,
            current_layer: 0,
            layer_keys: [[255; 3]; 20],
        };
    }

    fn press_layer_key(mut self, column: usize, row: usize, key: KeyStroke) {
        self.current_layer = key.layer;
        self.layer_keys.rotate_right(1);
        self.layer_keys[0] = [column, row, key.layer];
    }

    fn release_layer_key(mut self, column: usize, row: usize) {
        for i in 0..19 {
            if column == self.layer_keys[i][0] && row == self.layer_keys[i][1] {
                for j in i..18 {
                    self.layer_keys[j] = self.layer_keys[j + 1];
                }
                self.layer_keys[19] = [255, 255, 255];
                break;
            }
        }
        self.current_layer = self.layer_keys[0][2];
    }

    pub fn keys_to_hids(
        mut self,
        column: usize,
        raised_keys: [usize; 8],
        lowered_keys: [usize; 8],
    ) -> [u8; 9] {
        let ret: [u8; 9] = [0; 9];
        if column > 15 {
            return ret;
        }
        let mut hid_i: usize = 1;
        let mut modifiers: u8 = 0;
        for key in raised_keys {
            if key > 7 {
                break;
            }
            let stroke = self.layers[self.current_layer][column][key];
            if stroke.is_layer {
                self.release_layer_key(column, key)
            } else {
            }
        }
        return ret;
    }
}
