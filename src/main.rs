#![no_std]
#![no_main]

extern crate panic_halt;
mod keyboard;

use cortex_m::{asm::delay as asm_delay, interrupt::free as disable_interrupts, peripheral::NVIC};
use hal::{
    clock::GenericClockController,
    prelude::*,
    sercom::I2CError,
    time::{Hertz, KiloHertz},
    usb::UsbBus,
};
use pac::{interrupt, CorePeripherals, Peripherals};
use usb_device::{bus::UsbBusAllocator, prelude::*};
use usbd_hid::{
    descriptor::{generator_prelude::*, KeyboardReport},
    hid_class::HIDClass,
};

use bsp::{
    entry,
    hal,
    //hal::sercom::v2::uart::{self, Word},
    pac,
    Led0,
    Led1,
    Uart,
};

use shared_bus;
use xiao_m0 as bsp;

fn serial_print(u: &mut Uart, s: &str) {
    for c in s.chars() {
        nb::block!(u.write(c as u8)).unwrap();
    }
}

fn print_err(u: &mut Uart, r: Result<(), I2CError>) {
    match r {
        Err(e) => match e {
            I2CError::ArbitrationLost => serial_print(u, "I2C Arbitration Lost\r\n"),
            I2CError::AddressError => serial_print(u, "I2C Address Error\r\n"),
            I2CError::BusError => serial_print(u, "I2C Bus Error\r\n"),
            I2CError::Timeout => serial_print(u, "I2C Timeout\r\n"),
            I2CError::Nack => serial_print(u, "I2C Nack\r\n"),
        },
        Ok(_) => (),
    }
}

fn get_indices(n: u8) -> [i8; 8] {
    let mask: u8 = 1;
    let mut ret: [i8; 8] = [-1, -1, -1, -1, -1, -1, -1, -1];
    for i in 0..7 {
        if (mask << i) & n > 0 {
            ret[i] = i as i8;
        }
    }
    return ret;
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

    let mut uart = bsp::uart(
        &mut clocks,
        Hertz(115200),
        peripherals.SERCOM4,
        &mut peripherals.PM,
        pins.a7,
        pins.a6,
    );

    serial_print(&mut uart, "Hello, World!\r\n");
    serial_print(&mut uart, "Hello, World!\r\n");

    let mut led_loop: Led0 = pins.led0.into_push_pull_output();
    unsafe {
        LED1 = Some(pins.led1.into_push_pull_output());
    }
    //led_loop.toggle().unwrap();
    asm_delay(15 * 1024 * 1024);
    unsafe {
        USB_HID = Some(HIDClass::new(bus_allocator, KeyboardReport::desc(), 60));
        USB_BUS = Some(
            UsbDeviceBuilder::new(&bus_allocator, UsbVidPid(0xdead, 0xbeef))
                .manufacturer("Hackers University")
                .product("xiao_useless_keyboard")
                .serial_number("42")
                .device_class(0x03)
                .build(),
        );
        //LED_DATA = Some(pins.led1.into_mode());
    }
    //led_loop.toggle().unwrap();
    asm_delay(15 * 1024 * 1024);

    unsafe {
        core.NVIC.set_priority(interrupt::USB, 1);
        NVIC::unmask(interrupt::USB);
    }

    // Flash the LED in a spin loop to demonstrate
    // that USB is entirely interrupt driven.
    //led_loop.toggle().unwrap();
    let i2c_bus = shared_bus::BusManagerSimple::new(i2c);
    let mut proxy = i2c_bus.acquire_i2c();
    print_err(&mut uart, proxy.write(0x27, &[0x07, 0x00]));
    led_loop.toggle().unwrap();
    asm_delay(15 * 1024 * 1024);
    //led_loop.toggle().unwrap();
    let columns: [u8; 8] = [0, 1, 2, 3, 4, 5, 255, 255];
    let row_mask: u8 = 0x3F;

    //let keymap: [u8; 8] = [0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25];
    let mut hid_manager = HidManager::new();
    loop {
        let keys: [u8; 6] = [0, 0, 0, 0, 0, 0];
        for n in columns {
            if n > 7 {
                continue;
            }
            let key: &mut [u8; 1] = &mut [0x00];
            print_err(&mut uart, proxy.write(0x27, &[0x03, 1 << n])); //.unwrap();
            print_err(&mut uart, proxy.write(0x27, &[0x00])); //;.unwrap();
            print_err(&mut uart, proxy.read(0x27, key)); //.unwrap();
            for key in get_indices(key[0] & row_mask) {
                hid_manager.press_key(layout::LEFT_BOARD[n as usize][key as usize].hid_code);
            }
        }
        push_keyboard_report(KeyboardReport {
            modifier: 0,
            reserved: 0,
            leds: 0,
            keycodes: keys,
        });
        /*asm_delay(15 * 1024 * 1024);
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
        */
    }
}

fn push_keyboard_report(report: KeyboardReport) -> Result<usize, usb_device::UsbError> {
    disable_interrupts(|_| unsafe { USB_HID.as_mut().map(|hid| hid.push_input(&report)) }).unwrap()
}

static mut USB_ALLOCATOR: Option<UsbBusAllocator<UsbBus>> = None;
static mut USB_BUS: Option<UsbDevice<UsbBus>> = None;
static mut USB_HID: Option<HIDClass<UsbBus>> = None;
static mut LED1: Option<Led1> = None;

fn poll_usb() {
    unsafe {
        if let (Some(usb_dev), Some(hid)) = (USB_BUS.as_mut(), USB_HID.as_mut()) {
            usb_dev.poll(&mut [hid]);
        }
        match LED1.as_mut() {
            None => None,
            Some(thing) => Some(thing.toggle()),
        };
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
