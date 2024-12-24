#![no_std]
#![no_main]

extern crate alloc;

use crate::commands::read_command;
use alloc::string::String;
use embassy_executor::Spawner;
use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{Level, Output},
    prelude::*,
    usb_serial_jtag::UsbSerialJtag,
    Blocking,
};
use esp_println::println;
use heapless::spsc::Queue;

mod commands;

#[embassy_executor::task]
async fn usb_thread(
    mut usb_serial: UsbSerialJtag<'static, Blocking>,
    mut queue: Queue<String, 10>,
    mut user_led: Output<'static>,
) {
    let delay = Delay::new();

    loop {
        if let Some(data) = queue.dequeue() {
            println!("Sending command: {:?}", data);
        }
        let command = read_command(&mut usb_serial);

        match command {
            commands::Commands::Init => {
                println!("0");
            }
            commands::Commands::LightOn => {
                user_led.set_low();
                println!("0");
            }
            commands::Commands::LightOff => {
                user_led.set_high();
                println!("0");
            }
            commands::Commands::WifiActivate => {
                println!("0");
            }
            commands::Commands::WifiDeactivate => {
                println!("0");
            }
            _ => {
                println!("Unknown command");
            }
        }

        delay.delay(500.millis());
    }
}

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    esp_println::logger::init_logger_from_env();
    esp_alloc::heap_allocator!(72 * 1024);

    let peripherals = esp_hal::init(esp_hal::Config::default());

    let usb_serial = UsbSerialJtag::new(peripherals.USB_DEVICE);
    let user_led = Output::new(peripherals.GPIO21, Level::High);

    let queue: Queue<String, 10> = Queue::new();

    spawner.spawn(usb_thread(usb_serial, queue, user_led)).ok();
}
