//! # Pico USB 'Twitchy' Mouse Example
//!
//! Creates a USB HID Class Poiting device (i.e. a virtual mouse) on a Pico
//! board, with the USB driver running in the main thread.
//!
//! It generates movement reports which will twitch the cursor up and down by a
//! few pixels, several times a second.
//!
//! See the `Cargo.toml` file for Copyright and licence details.
//!
//! This is a port of
//! https://github.com/atsamd-rs/atsamd/blob/master/boards/itsybitsy_m0/examples/twitching_usb_mouse.rs

#![no_std]
#![no_main]

use cortex_m::delay::Delay;
// The macro for our start-up function
use cortex_m_rt::entry;

// The macro for marking our interrupt functions
use rp_pico::hal::pac::interrupt;

// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

// Pull in any important traits
use embedded_time::fixed_point::FixedPoint;
use rp_pico::hal::prelude::*;

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use rp_pico::hal::pac;

// A shorter alias for the Hardware Abstraction Layer, which provides
// higher-level drivers.
use rp_pico::hal;

// USB Device support
use usb_device::{class_prelude::*, prelude::*};

use usbd_hid::descriptor::KeyboardReport;
// USB Human Interface Device (HID) Class support
use usbd_hid::descriptor::generator_prelude::*;
use usbd_hid::hid_class::HIDClass;

mod keymap;

/// The USB Device Driver (shared with the interrupt).
static mut USB_DEVICE: Option<UsbDevice<hal::usb::UsbBus>> = None;

/// The USB Bus Driver (shared with the interrupt).
static mut USB_BUS: Option<UsbBusAllocator<hal::usb::UsbBus>> = None;

/// The USB Human Interface Device Driver (shared with the interrupt).
static mut USB_HID: Option<HIDClass<hal::usb::UsbBus>> = None;

/// Entry point to our bare-metal application.
///
/// The `#[entry]` macro ensures the Cortex-M start-up code calls this function
/// as soon as all global variables are initialised.
///
/// The function configures the RP2040 peripherals, then submits cursor movement
/// updates periodically.
#[entry]
fn main() -> ! {
    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    //
    // The default is to generate a 125 MHz system clock
    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // Set up the USB driver
    let usb_bus = UsbBusAllocator::new(hal::usb::UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    ));
    unsafe {
        // Note (safety): This is safe as interrupts haven't been started yet
        USB_BUS = Some(usb_bus);
    }

    // Grab a reference to the USB Bus allocator. We are promising to the
    // compiler not to take mutable access to this global variable whilst this
    // reference exists!
    let bus_ref = unsafe { USB_BUS.as_ref().unwrap() };

    // Set up the USB HID Class Device driver, providing Mouse Reports
    let usb_hid = HIDClass::new(bus_ref, KeyboardReport::desc(), 60);
    unsafe {
        // Note (safety): This is safe as interrupts haven't been started yet.
        USB_HID = Some(usb_hid);
    }

    // Create a USB device with a fake VID and PID
    let usb_dev = UsbDeviceBuilder::new(bus_ref, UsbVidPid(0x1337, 0x1337))
        .manufacturer("d0ntrash")
        .product("Rubber Ducky")
        .serial_number("1337")
        .device_class(0xEF) // misc
        .build();
    unsafe {
        // Note (safety): This is safe as interrupts haven't been started yet
        USB_DEVICE = Some(usb_dev);
    }

    unsafe {
        // Enable the USB interrupt
        pac::NVIC::unmask(hal::pac::Interrupt::USBCTRL_IRQ);
    };
    let core = pac::CorePeripherals::take().unwrap();
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().integer());
    delay.delay_ms(1000);
    // Move the cursor up and down every 200ms
    //send_keystroke(keymap::Modifier::LMeta, keymap::Keys::KeyEnter, &mut delay);
    delay.delay_ms(100);
    loop {
        send_keystroke(keymap::Modifier::None, keymap::parse_char(&'a').unwrap(), &mut delay);
        send_string("this is a string 123 even with s0me=symblos/", &mut delay);
        delay.delay_ms(100);
    }
}

fn send_string(send_string: &str, delay: &mut Delay) {
    for c in send_string.chars() {
        send_keystroke(keymap::Modifier::None, keymap::parse_char(&c).unwrap(), delay);
    }
    delay.delay_ms(1000)
}

fn send_keystroke(modifier: keymap::Modifier, keycode: keymap::Keys, delay: &mut Delay) {
    delay.delay_ms(40);
    // Build and send key report
    let keystroke = KeyboardReport {
        modifier: modifier as u8,
        reserved: 0x00,
        leds: 0x00,
        keycodes: [keycode as u8, 0x00, 0x00, 0x00, 0x00, 0x00]
    };
    push_keyboard_input(keystroke).ok().unwrap_or(0);
    delay.delay_ms(40);

    // Build and send reset report
    let reset = KeyboardReport {
        modifier: 0x00,
        reserved: 0x00,
        leds: 0x00,
        keycodes: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
    };
    push_keyboard_input(reset).ok().unwrap_or(0);
}

fn push_keyboard_input(report: KeyboardReport) -> Result<usize, usb_device::UsbError> {
    cortex_m::interrupt::free(|_| unsafe {
        // Now interrupts are disabled, grab the global variable and, if
        // available, send it a HID report
        USB_HID.as_mut().map(|hid| hid.push_input(&report))
    })
    .unwrap()
}

/// This function is called whenever the USB Hardware generates an Interrupt
/// Request.
#[allow(non_snake_case)]
#[interrupt]
unsafe fn USBCTRL_IRQ() {
    // Handle USB request
    let usb_dev = USB_DEVICE.as_mut().unwrap();
    let usb_hid = USB_HID.as_mut().unwrap();
    usb_dev.poll(&mut [usb_hid]);
}

// End of file
