#![no_std]
#![no_main]

extern crate alloc;

use alloc::{
    string::{String, ToString},
    sync::Arc,
};
use embassy_executor::Spawner;
use esp_backtrace as _;
use esp_hal::{
    gpio::{Level, Output},
    rng::Rng,
    timer::timg::TimerGroup,
    usb_serial_jtag::UsbSerialJtag,
};
// use heapless::spsc::Queue;
use embassy_sync::{blocking_mutex::raw::NoopRawMutex, channel::Channel, mutex::Mutex};

mod commands;
mod loops;

#[esp_hal_embassy::main]
pub async fn main(spawner: Spawner) {
    esp_println::logger::init_logger_from_env();
    esp_alloc::heap_allocator!(72 * 1024);

    let peripherals = esp_hal::init(esp_hal::Config::default());

    let usb_serial = UsbSerialJtag::new(peripherals.USB_DEVICE);
    let user_led = Arc::new(Mutex::new(Output::new(peripherals.GPIO21, Level::High)));

    let timg0 = TimerGroup::new(peripherals.TIMG0);

    let queue: Arc<Channel<NoopRawMutex, String, 10>> = Arc::new(Channel::new());

    let sender = queue.sender();

    sender.send("init".to_string()).await;

    spawner
        .spawn(loops::usb_event::usb_thread(
            usb_serial,
            user_led.clone(),
            queue.clone(),
        ))
        .ok();
    spawner
        .spawn(loops::wifi_event::esp_now_thread(
            timg0,
            Rng::new(peripherals.RNG),
            peripherals.RADIO_CLK,
            peripherals.WIFI,
            user_led.clone(),
            queue.clone(),
        ))
        .ok();
}
