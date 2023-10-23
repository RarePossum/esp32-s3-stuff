#![no_std]
#![no_main]

extern crate alloc;
use core::mem::MaybeUninit;
use esp_backtrace as _;
use esp_println::println;
use hal::{clock::ClockControl, gpio, peripherals::Peripherals, prelude::*, Delay, IO};
use hal::otg_fs::{UsbBus, USB};
use hal::{timer::TimerGroup, Rng};

use usb_device::{bus::UsbBusAllocator, device::UsbDevice, prelude::*};
use usbd_hid::hid_class::{
    HIDClass, HidClassSettings, HidCountryCode, HidProtocol, HidSubClass, ProtocolModeConfig,
};
#[global_allocator]
static ALLOCATOR: esp_alloc::EspHeap = esp_alloc::EspHeap::empty();
static mut USB_BUFFER: [u32; 1024] = [0; 1024];
fn init_heap() {
    const HEAP_SIZE: usize = 32 * 1024;
    static mut HEAP: MaybeUninit<[u8; HEAP_SIZE]> = MaybeUninit::uninit();

    unsafe {
        ALLOCATOR.init(HEAP.as_mut_ptr() as *mut u8, HEAP_SIZE);
    }
}

const REPORT_DESCRIPTOR: &[u8] = &[
    0x05, 0x01,        // Usage Page (Generic Desktop Ctrls)
    0x09, 0x06,        // Usage (Keyboard)
    0xA1, 0x01,        // Collection (Application)
    0x05, 0x07,        //   Usage Page (Kbrd/Keypad)
    0x19, 0xE0,        //   Usage Minimum (0xE0)
    0x29, 0xE7,        //   Usage Maximum (0xE7)
    0x15, 0x00,        //   Logical Minimum (0)
    0x25, 0x01,        //   Logical Maximum (1)
    0x95, 0x08,        //   Report Count (8)
    0x75, 0x01,        //   Report Size (1)
    0x81, 0x02,        //   Input (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
    0x95, 0x01,        //   Report Count (1)
    0x75, 0x08,        //   Report Size (8)
    0x81, 0x03,        //   Input (Const,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)
    0x05, 0x07,        //   Usage Page (Kbrd/Keypad)
    0x19, 0x00,        //   Usage Minimum (0x00)
    0x29, 0xFF,        //   Usage Maximum (0xFF)
    0x15, 0x00,        //   Logical Minimum (0)
    0x26, 0xFF, 0x00,  //   Logical Maximum (255)
    0x95, 0x06,        //   Report Count (6)
    0x75, 0x08,        //   Report Size (8)
    0x81, 0x00,        //   Input (Data,Array,Abs,No Wrap,Linear,Preferred State,No Null Position)
    0x05, 0x08,        //   Usage Page (LEDs)
    0x19, 0x01,        //   Usage Minimum (Num Lock)
    0x29, 0x05,        //   Usage Maximum (Kana)
    0x95, 0x05,        //   Report Count (5)
    0x75, 0x01,        //   Report Size (1)
    0x91, 0x02,        //   Output (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    0x95, 0x01,        //   Report Count (1)
    0x75, 0x03,        //   Report Size (3)
    0x91, 0x03,        //   Output (Const,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)
    0xC0,              // End Collection
];

#[entry]
fn main() -> ! {
    init_heap();
    let peripherals = Peripherals::take();
    let mut system = peripherals.SYSTEM.split();
    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let pins = io.pins;
    let mut led = pins.gpio4.into_push_pull_output();
    let mut out = pins.gpio8.into_push_pull_output();
    let input = pins.gpio9.into_pull_down_input();
    led.set_low().unwrap();
    out.set_high().unwrap();

    // setup logger
    // To change the log_level change the env section in .cargo/config.toml
    // or remove it and set ESP_LOGLEVEL manually before running cargo run
    // this requires a clean rebuild because of https://github.com/rust-lang/cargo/issues/10358
    esp_println::logger::init_logger_from_env();
    log::info!("Logger is setup");
    println!("Hello world!");
    let timer = TimerGroup::new(
        peripherals.TIMG1,
        &clocks,
        &mut system.peripheral_clock_control,
    )
    .timer0;

    // Set up USB
    let usb = USB::new(
        peripherals.USB0,
        pins.gpio18,
        pins.gpio19,
        pins.gpio20,
        &mut system.peripheral_clock_control,
    );

    // Allocate USB buffer
    let usb_bus = UsbBus::new(usb, unsafe {
        &mut USB_BUFFER
    });

    // Set up ESP32 as a HID device
    let hid_endpoint = HIDClass::new_ep_in_with_settings(
        &usb_bus,
        REPORT_DESCRIPTOR,
        1,
        HidClassSettings { 
            subclass: (HidSubClass::NoSubClass),
            protocol: (HidProtocol::Keyboard), 
            config: (ProtocolModeConfig::ForceBoot), 
            locale: (HidCountryCode::US) 
        }
    );

    // Build USB
    let usb_dev_builder = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x0000, 0x0528))
        .manufacturer("The Rarest")
        .product("Three Split Keyboard")
        .serial_number("8484528");

    let _usb_dev = usb_dev_builder.build();

    loop {

        /*
        if input.is_high().unwrap() {
            led.set_high().unwrap();
        } else {
            led.set_low().unwrap();
        }

        delay.delay_us(7500u32); //7.5ms is the maximum transmission rate of BLE
        */
    }
}
