pub mod hid_manager;

use atsamd_hal::prelude::_atsamd_hal_embedded_hal_digital_v2_ToggleableOutputPin;
use hid_manager::key_scanner::{
    layout::{self, Column},
    KeyTracker,
};
use hid_manager::HidManager;
use usbd_hid::descriptor::KeyboardReport;
use xiao_m0::Led0;

pub trait ColumnReader {
    fn read_column(&mut self, section: u8, column: u8) -> Option<Column>;
}

pub trait ReportSender {
    fn send_report(&self, report: KeyboardReport);
}

pub struct Keyboard<'a> {
    hid: &'a mut HidManager,
    tracker: &'a mut KeyTracker,
    reader: &'a mut dyn ColumnReader,
    sender: &'a dyn ReportSender,
    led0: Led0,
    _disabled_sections: [bool; layout::N_SECTIONS],
}

impl<'a> Keyboard<'a> {
    pub fn new(
        hid: &'a mut HidManager,
        tracker: &'a mut KeyTracker,
        reader: &'a mut impl ColumnReader,
        sender: &'a impl ReportSender,
        led0: Led0,
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
            led0,
            _disabled_sections,
        };
    }

    pub fn run_forever(&mut self) -> ! {
        loop {
            for i in 0..100 {
                if i == 0 {
                    self.led0.toggle().unwrap();
                }
                for section in 0..layout::N_SECTIONS {
                    if self._disabled_sections[section] {
                        continue;
                    }
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

                        let mut which_reports: u8 = 0;
                        let reports = self.hid.report(&mut which_reports);
                        if which_reports & 1 > 0 {
                            self.sender.send_report(reports[0]);
                        }
                        if which_reports & 2 > 0 {
                            self.sender.send_report(reports[1]);
                        }
                    }
                }
            }
        }
    }
}
