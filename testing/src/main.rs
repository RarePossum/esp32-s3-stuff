#![no_std]
#![no_main]

extern crate alloc;
use core::mem::MaybeUninit;
use esp_backtrace as _;
use esp_println::println;
use hal::{clock::ClockControl, gpio::IO, peripherals::Peripherals, prelude::*, Delay};
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
