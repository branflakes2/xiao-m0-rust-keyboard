#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m::{asm::delay as asm_delay, interrupt::free as disable_interrupts, peripheral::NVIC};
use hal::{clock::GenericClockController, prelude::*, usb::UsbBus};
use pac::{interrupt, CorePeripherals, Peripherals};
use usb_device::{bus::UsbBusAllocator, prelude::*};
use usbd_hid::{
    descriptor::{generator_prelude::*, KeyboardReport},
    hid_class::HIDClass,
};

use bsp::{entry, hal, pac, Led0};
use xiao_m0 as bsp;

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

    unsafe {
        core.NVIC.set_priority(interrupt::USB, 1);
        NVIC::unmask(interrupt::USB);
    }

    // Flash the LED in a spin loop to demonstrate
    // that USB is entirely interrupt driven.
    let mut led_loop: Led0 = pins.led0.into_push_pull_output();
    loop {
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
    }
}

fn push_keyboard_report(report: KeyboardReport) -> Result<usize, usb_device::UsbError> {
    disable_interrupts(|_| unsafe { USB_HID.as_mut().map(|hid| hid.push_input(&report)) }).unwrap()
}

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
