#![no_std]
#![no_main]

extern crate panic_halt;
mod keyboard;

use cortex_m::{asm::delay as asm_delay, interrupt::free as disable_interrupts, peripheral::NVIC};
use hal::{clock::GenericClockController, prelude::*, time::KiloHertz, usb::UsbBus};
use pac::{interrupt, CorePeripherals, Peripherals};
use shared_bus;
use usb_device::{bus::UsbBusAllocator, prelude::*};
use usbd_hid::{
    descriptor::{generator_prelude::*, KeyboardReport},
    hid_class::HIDClass,
};

use bsp::{entry, hal, pac, Led0};
use keyboard::{
    self,
    hid_manager::{
        key_scanner::{
            self,
            layout::{self, Column},
            KeyTracker,
        },
        HidManager,
    },
    ColumnReader, Keyboard, ReportSender,
};
use xiao_m0 as bsp;

struct XiaoM0Sender {}

impl ReportSender for XiaoM0Sender {
    fn send_report(report: KeyboardReport) {
        disable_interrupts(|_| unsafe { USB_HID.as_mut().map(|hid| hid.push_input(&report)) });
    }
}

struct XiaoM0ColumnReader<T> {
    i2c_bus: shared_bus::BusManagerSimple<T>,
}

impl ColumnReader for XiaoM0ColumnReader<T> {
    fn read_column(self, section: u8, column: u8) -> Column {
        if column > 7 {
            return;
        }
        let mut proxy = self.i2c_bus.acquire_i2c();
        let key: &mut [u8; 1] = &mut [0x00];
        proxy.write(layout::SECTION_I2C_ADDRESSES[section], &[0x07, 0x00]);
        proxy.write(layout::SECTION_I2C_ADDRESSES[section], &[0x03, 1 << column]);
        proxy.write(layout::SECTION_I2C_ADDRESSES[section], &[0x00]);
        proxy.read(layout::SECTION_I2C_ADDRESSES[section], key);
        return key[0];
    }
}

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let mut core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let pins = bsp::Pins::new(peripherals.PORT);
    let bus_allocator = unsafe {
        USB_ALLOCATOR = Some(bsp::usb_allocator(
            peripherals.USB,
            &mut clocks,
            &mut peripherals.PM,
            pins.usb_dm,
            pins.usb_dp,
        ));
        USB_ALLOCATOR.as_ref().unwrap()
    };
    let i2c = bsp::i2c_master(
        &mut clocks,
        KiloHertz(400),
        peripherals.SERCOM0,
        &mut peripherals.PM,
        pins.a4,
        pins.a5,
    );
    let i2c_bus = shared_bus::BusManagerSimple::new(i2c);
    let mut reader = XiaoM0ColumnReader { i2c_bus };
    let mut sender = XiaoM0Sender {};
    let tracker = KeyTracker::new();
    let mut hid_manage = HidManager::new();
    let mut keyboard = Keyboard::new(hid_manage, tracker, reader, sender);

    unsafe {
        USB_HID = Some(HIDClass::new(bus_allocator, KeyboardReport::desc(), 60));
        USB_BUS = Some(
            UsbDeviceBuilder::new(&bus_allocator, UsbVidPid(0xdead, 0xbeef))
                .manufacturer("Brian Weber")
                .product("Custom Dactyl")
                .serial_number("42")
                .device_class(0x03)
                .build(),
        );
        //LED_DATA = Some(pins.led1.into_mode());
    }

    unsafe {
        core.NVIC.set_priority(interrupt::USB, 1);
        NVIC::unmask(interrupt::USB);
    }

    // Flash the LED in a spin loop to demonstrate
    // that USB is entirely interrupt driven.
    //let mut led_loop: Led0 = pins.led0.into_push_pull_output();
    /*loop {
        asm_delay(15 * 1024 * 1024);
        led_loop.toggle().unwrap();
        push_keyboard_report(KeyboardReport {
            modifier: 0,
            reserved: 0,
            leds: 0,
            keycodes: [0x04, 0, 0, 0, 0, 0],
        })
        .unwrap();
        asm_delay(15 * 1024 * 1024);
        push_keyboard_report(KeyboardReport {
            modifier: 0,
            reserved: 0,
            leds: 0,
            keycodes: [0, 0, 0, 0, 0, 0],
        })
        .unwrap();
        asm_delay(15 * 1024 * 1024);
    }*/
    keyboard.run_forever();
}

fn push_keyboard_report(report: KeyboardReport) -> Result<usize, usb_device::UsbError> {}

static mut USB_ALLOCATOR: Option<UsbBusAllocator<UsbBus>> = None;
static mut USB_BUS: Option<UsbDevice<UsbBus>> = None;
static mut USB_HID: Option<HIDClass<UsbBus>> = None;

fn poll_usb() {
    unsafe {
        if let (Some(usb_dev), Some(hid)) = (USB_BUS.as_mut(), USB_HID.as_mut()) {
            usb_dev.poll(&mut [hid]);
        }
    };
}

#[interrupt]
fn USB() {
    poll_usb();
}
/*static mut USB_ALLOCATOR: Option<UsbBusAllocator<UsbBus>> = None;
static mut USB_BUS: Option<UsbDevice<UsbBus>> = None;
static mut USB_SERIAL: Option<SerialPort<UsbBus>> = None;
static mut LED_DATA: Option<Led1> = None;

fn poll_usb() {
    unsafe {
        USB_BUS.as_mut().map(|usb_dev| {
            USB_SERIAL.as_mut().map(|serial| {
                usb_dev.poll(&mut [serial]);
                let mut buf = [0u8; 64];

                if let Ok(count) = serial.read(&mut buf) {
                    for (i, c) in buf.iter().enumerate() {
                        if i >= count {
                            break;
                        }
                        serial.write(&[c.clone()]).unwrap();
                        LED_DATA.as_mut().map(|led| led.toggle());
                    }
                };
            });
        });
    };
}

#[interrupt]
fn USB() {
    poll_usb();
}*/
