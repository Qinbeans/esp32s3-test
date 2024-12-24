# ESP32S3 Test

This is a test to see if I can turn an ESP32-S3 into a dongle controller.

The ESP32-S3 would be plugged into a computer or phone and interfaced with an application on the device. You should then be able to send and receive data with the ESP32-S3.

## Tooling

Rust is required

```bash
cargo install espup
espup install
```

```bash
cargo install cargo-espflash
```

## Building

This will build and flash the ESP32-S3.

```bash
cargo run
```
