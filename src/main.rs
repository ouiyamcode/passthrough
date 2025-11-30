use esp_idf_svc::log::EspLogger;
use esp_idf_sys::{self as sys, link_patches};
use std::io::Read;
use std::{thread, time::Duration};

const EOT: u8 = 0x04;

fn main() {
    link_patches();
    EspLogger::initialize_default();

    unsafe {
        sys::esp_task_wdt_deinit();
    }

    log::info!("ESP32-S3 READY - waiting for data");

    //unsafe { sys::esp_vfs_dev_usb_serial_jtag_register(); } //car on veut mettre les logs en tx rx

    let stdin = std::io::stdin();
    let mut handle = stdin.lock();
    let mut buf = [0u8; 256];
    let mut output: Vec<u8> = Vec::new();

    loop {
        match handle.read(&mut buf){
            Ok(n) if n > 0 => {
                if buf[..n].contains(&EOT){
                    let pos = buf[..n].iter().position(|&b| b == EOT).unwrap();
                    output.extend_from_slice(&buf[..pos]);

                    log::info!("EOF received. Total: {} bytes", output.len());

                    match std::str::from_utf8(&output){
                        Ok(text) => log::info!("FILE CONTENT:\n{}", text),
                        Err(_) => log::info!("FILE IS BINARY (non-UTF8)"),
                    }
                    
                    output.clear();
                    log::info!("READY FOR NEXT FILE!");
                    continue;
                }
                output.extend_from_slice(&buf[..n]);
                log::info!("Received {} bytes (total {})", n, output.len());
            }
            _ => thread::sleep(Duration::from_millis(5)),
        }
    }
}
