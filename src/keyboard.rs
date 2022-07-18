pub mod hid_manager;

use hid_manager::key_scanner::{
    layout::{self, Column},
    KeyTracker,
};
use hid_manager::HidManager;
use usbd_hid::descriptor::KeyboardReport;

pub trait ColumnReader {
    fn read_column(self, section: u8, column: u8) -> Column;
}

pub trait ReportSender {
    fn send_report(self, report: KeyboardReport);
}

pub struct Keyboard<'a> {
    hid: HidManager,
    tracker: KeyTracker,
    reader: &'a ColumnReader,
    sender: &'a ReportSender,
}

impl Keyboard<'a> {
    pub fn new(
        hid: HidManager,
        tracker: KeyTracker,
        reader: &'a ColumnReader,
        sender: &'a ReportSender,
    ) -> Self {
        return Keyboard {
            hid,
            tracker,
            reader,
            sender,
        };
    }

    pub fn run_forever(mut self) {
        loop {
            for section in layout::N_SECTIONS {
                for column in layout::SECTION_COLS {
                    let c = self.reader.read_column(section, column);
                    let strokes = self.tracker.process_column(section, c, column);

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
