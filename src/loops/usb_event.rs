use crate::commands::{self, read_command};
use alloc::{string::String, sync::Arc};
use embassy_sync::{blocking_mutex::raw::NoopRawMutex, channel::Channel, mutex::Mutex};
use esp_hal::{delay::Delay, gpio::Output, prelude::*, usb_serial_jtag::UsbSerialJtag, Blocking};
use esp_println::println;

#[embassy_executor::task]
pub async fn usb_thread(
    mut usb_serial: UsbSerialJtag<'static, Blocking>,
    user_led: Arc<Mutex<NoopRawMutex, Output<'static>>>,
    queue: Arc<Channel<NoopRawMutex, String, 10>>,
) {
    let delay = Delay::new();
    let receiver = queue.receiver();
    loop {
        if !receiver.is_empty() {
            let data = receiver.receive().await;
            println!("R0\r{}", data);
        }
        let command = read_command(&mut usb_serial);
        let mut user_led = user_led.lock().await;
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
