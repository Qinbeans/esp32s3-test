use esp_hal::{usb_serial_jtag::UsbSerialJtag, Blocking};

/**
 * Commands have a maximum of 4 arguments
 */
#[derive(Clone)]
pub enum Commands {
    Init,
    LightOn,
    LightOff,
    WifiActivate,
    WifiDeactivate,
    Error,
}

pub const COMMAND_MAP: [(&str, Commands); 5] = [
    ("i0", Commands::Init),
    ("l1", Commands::LightOn),
    ("l0", Commands::LightOff),
    ("w1", Commands::WifiActivate),
    ("w0", Commands::WifiDeactivate),
];

pub fn read_command(usb_serial: &mut UsbSerialJtag<'_, Blocking>) -> Commands {
    let mut bytes = [0u8; 64];
    let mut index = 0;

    loop {
        if let Ok(byte) = usb_serial.read_byte() {
            if byte == b'\r' {
                break;
            }
            bytes[index] = byte;
            index += 1;
        }
    }

    let command = core::str::from_utf8(&bytes[..index]).unwrap();

    match COMMAND_MAP.iter().find(|(key, _)| key == &command) {
        Some((_, command)) => command.clone(),
        None => Commands::Error,
    }
}
