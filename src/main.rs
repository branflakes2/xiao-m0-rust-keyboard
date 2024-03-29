#![no_std]
#![no_main]

extern crate panic_halt;
mod keyboard;

use cortex_m::{asm::delay, interrupt::free as disable_interrupts, peripheral::NVIC};
use hal::{clock::GenericClockController, prelude::*, time::KiloHertz, usb::UsbBus};
use pac::{interrupt, CorePeripherals, Peripherals};
use shared_bus;
use usb_device::{bus::UsbBusAllocator, prelude::*};
use usbd_hid::{
    descriptor::{generator_prelude::*, KeyboardReport},
    hid_class::{
        HIDClass, HidClassSettings, HidCountryCode, HidProtocol, HidSubClass, ProtocolModeConfig,
    },
};

use bsp::{entry, hal, pac, Led2};
use keyboard::{
    //self,
    hid_manager::{
        key_scanner::{
            //self,
            layout::{self, Column},
            KeyTracker,
        },
        HidManager,
    },
    ColumnReader,
    Keyboard,
    ReportSender,
};
use xiao_m0 as bsp;

struct XiaoM0Sender {}

pub const DEBOUNCE_SENSE: usize = 3;

impl ReportSender for XiaoM0Sender {
    fn send_report(&self, report: KeyboardReport) {
        disable_interrupts(|_| unsafe {
            USB_HID.as_mut().map(|hid| {
                while let Some(err) = hid.push_input(&report).err() {
                    match err {
                        UsbError::WouldBlock => delay(128),
                        _ => break,
                    }
                }
            })
        });
    }
}

struct XiaoM0ColumnReader<T> {
    i2c_bus: shared_bus::BusManagerSimple<T>,
    is_setup: [bool; layout::N_SECTIONS],
}

impl<
        T: cortex_m::prelude::_embedded_hal_blocking_i2c_Read
            + cortex_m::prelude::_embedded_hal_blocking_i2c_Write,
    > ColumnReader for XiaoM0ColumnReader<T>
{
    fn read_column(&mut self, section: u8, column: u8) -> Option<Column> {
        if column > 7 {
            return None;
        }
        let mut proxy = self.i2c_bus.acquire_i2c();
        if !self.is_setup[section as usize] {
            let res = proxy.write(
                layout::SECTION_I2C_ADDRESSES[section as usize],
                &[0x07, 0x00],
            );
            if res.is_err() {
                return None;
            }
            self.is_setup[section as usize] = true;
        }
        let key: &mut [u8; 1] = &mut [0x00];
        let res = proxy.write(
            layout::SECTION_I2C_ADDRESSES[section as usize],
            &[0x03, 1 << column],
        );
        if res.is_err() {
            return None;
        }
        let res = proxy.write(layout::SECTION_I2C_ADDRESSES[section as usize], &[0x00]);
        if res.is_err() {
            return None;
        }
        let res = proxy.read(layout::SECTION_I2C_ADDRESSES[section as usize], key);
        if res.is_err() {
            return None;
        }
        return Some(key[0]);
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
    let is_setup = [false; layout::N_SECTIONS];
    let led0 = pins.led0.into_push_pull_output();
    unsafe {
        LED = Some(pins.led2.into_push_pull_output());
    }
    let mut reader = XiaoM0ColumnReader { i2c_bus, is_setup };
    let sender = XiaoM0Sender {};
    let mut tracker = KeyTracker::new(DEBOUNCE_SENSE);
    let mut hid_manage = HidManager::new();
    let mut keyboard = Keyboard::new(&mut hid_manage, &mut tracker, &mut reader, &sender, led0);
    let hid_settings = HidClassSettings {
        subclass: HidSubClass::NoSubClass,
        protocol: HidProtocol::Generic,
        config: ProtocolModeConfig::DefaultBehavior,
        locale: HidCountryCode::NotSupported,
    };

    unsafe {
        USB_HID = Some(HIDClass::new_ep_in_with_settings(
            &bus_allocator,
            KeyboardReport::desc(),
            10,
            hid_settings,
        ));
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

    keyboard.run_forever()
}

//fn push_keyboard_report(report: KeyboardReport) -> Result<usize, usb_device::UsbError> {}

static mut USB_ALLOCATOR: Option<UsbBusAllocator<UsbBus>> = None;
static mut USB_BUS: Option<UsbDevice<UsbBus>> = None;
static mut USB_HID: Option<HIDClass<UsbBus>> = None;
static mut LED: Option<Led2> = None;

fn poll_usb() {
    unsafe {
        if let (Some(usb_dev), Some(hid), Some(led)) =
            (USB_BUS.as_mut(), USB_HID.as_mut(), LED.as_mut())
        {
            led.toggle().unwrap();
            usb_dev.poll(&mut [hid]);
            led.toggle().unwrap();
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
