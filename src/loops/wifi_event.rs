use alloc::{string::String, sync::Arc};
use embassy_sync::{blocking_mutex::raw::NoopRawMutex, channel::Channel, mutex::Mutex};
use esp_hal::{
    delay::Delay, gpio::Output, peripheral::Peripheral, peripherals::TIMG0, prelude::*, rng::Rng,
    timer::timg::TimerGroup, Blocking,
};
use esp_println::println;
use esp_wifi::esp_now::{EspNow, PeerInfo, BROADCAST_ADDRESS};

fn wifi_established(user_led: &mut Output<'static>) {
    for _ in 0..3 {
        user_led.set_low();
        Delay::new().delay(100.millis());
        user_led.set_high();
        Delay::new().delay(100.millis());
    }
}

#[embassy_executor::task]
pub async fn esp_now_thread(
    timg0: TimerGroup<'static, <TIMG0 as Peripheral>::P, Blocking>,
    rng: Rng,
    radio_clk: esp_hal::peripherals::RADIO_CLK,
    wifi_peripheral: esp_hal::peripherals::WIFI,
    user_led: Arc<Mutex<NoopRawMutex, Output<'static>>>,
    _queue: Arc<Channel<NoopRawMutex, String, 10>>,
) {
    let wifi_init = esp_wifi::init(timg0.timer0, rng, radio_clk).unwrap();
    let mut wifi = EspNow::new(&wifi_init, wifi_peripheral).unwrap();

    let mut user_led = user_led.lock().await;
    wifi_established(&mut user_led);
    drop(user_led);
    loop {
        let esp_request = wifi.receive_async().await;
        let other_address = esp_request.info.src_address;
        if esp_request.info.dst_address == BROADCAST_ADDRESS {
            println!("Broadcast from {:?}", other_address);
            if !wifi.peer_exists(&other_address) {
                wifi.add_peer(PeerInfo {
                    peer_address: other_address,
                    lmk: None,
                    channel: None,
                    encrypt: false,
                })
                .unwrap();
            }
        }
    }
}
