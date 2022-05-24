mod hid;
mod key_tracker;
mod layout;

use hid::HidManager;
use key_tracker::KeyTracker;
use layout::keys::KeyStroke;
use layout::BOARD;
//use usbd_hid::descriptor::KeyboardReport;

pub struct Keyboard<'a> {
    tracker: KeyTracker,
    layout: &'a [[KeyStroke; 8]; 16],
    hid_manager: HidManager,
}

impl<'a> Keyboard<'a> {
    pub fn new() -> Self {
        return Keyboard {
            tracker: KeyTracker::new(),
            layout: BOARD,
            hid_manager: HidManager::new(),
        };
    }
}
