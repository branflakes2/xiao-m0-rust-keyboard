pub mod hid_manager;

use hid_manager::key_scanner::{
    layout::{self, Column},
    KeyTracker,
};
use hid_manager::HidManager;
use usbd_hid::descriptor::KeyboardReport;

pub trait ColumnReader {
    fn read_column(&self, section: u8, column: u8) -> Option<Column>;
}

pub trait ReportSender {
    fn send_report(&self, report: KeyboardReport);
}

pub struct Keyboard<'a> {
    hid: &'a mut HidManager,
    tracker: &'a mut KeyTracker,
    reader: &'a dyn ColumnReader,
    sender: &'a dyn ReportSender,
    _disabled_sections: [bool; layout::N_SECTIONS],
}

impl<'a> Keyboard<'a> {
    pub fn new(
        hid: &'a mut HidManager,
        tracker: &'a mut KeyTracker,
        reader: &'a impl ColumnReader,
        sender: &'a impl ReportSender,
    ) -> Self {
        let mut _disabled_sections: [bool; layout::N_SECTIONS] = [false; layout::N_SECTIONS];
        for section in 0..layout::N_SECTIONS {
            let op = reader.read_column(section as u8, 0);
            if op.is_none() {
                _disabled_sections[section] = true;
            }
        }
        return Keyboard {
            hid,
            tracker,
            reader,
            sender,
            _disabled_sections,
        };
    }

    pub fn run_forever(&mut self) {
        loop {
            for section in 0..layout::N_SECTIONS {
                for column in 0..layout::SECTION_COLS {
                    let c = self.reader.read_column(section as u8, column as u8);
                    if c.is_none() {
                        continue;
                    }
                    let strokes = self.tracker.process_column(section, c.unwrap(), column);

                    // press keys
                    for key in 0..strokes[0].len() {
                        self.hid.process_key(strokes[0][key], true);
                    }

                    // release keys
                    for key in 0..strokes[1].len() {
                        self.hid.process_key(strokes[1][key], false);
                    }

                    let report = self.hid.report();
                    self.sender.send_report(report);
                }
            }
        }
    }
}
