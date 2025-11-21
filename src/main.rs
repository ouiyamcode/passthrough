use esp_idf_svc::log::EspLogger;
use esp_idf_svc::sys::*;
use std::{thread, time::Duration};
fn main() {
    //It is necessary to call this function once. Otherwise, some patches to the runtime
    esp_idf_svc::sys::link_patches();

    //Bind the log crate to the ESP Logging facilities, allow to print in the ESP monitor
    EspLogger::initialize_default();
    log::info!("Booting, initializing USB host");

    //Native ESP-IDF needs to be unsafe, because we are calling raw C functions
    unsafe{ 
        let cfg = usb_host_config_t{         //Minimal configuration, C structure to configure driver USB Host
            intr_flags: ESP_INTR_FLAG_LEVEL1 as i32,            //Interruption flags
            ..core::mem::zeroed()                               //Put all the other fields to 0 -> USB Host will use level 1 interruptions, default otherwise
        };

        let err = usb_host_install(&cfg);           //Call the C function to install the USB Host driver into the system
        if err != ESP_OK{
            log::error!("usb_host_install failed: {}", err);
        }
        else{
            log::info!("USB host installed successfully.");      //ESP_OK = 0 -> success
        }

        let client_cfg = usb_host_client_config_t{  //Register a USB Host client (mandatory to receive events)
            is_synchronous: false,                                            //async event system
            max_num_event_msg: 5,                                             //small event queue
            ..core::mem::zeroed()
        };

        let mut client_handle: usb_host_client_handle_t = core::ptr::null_mut();

        let err = usb_host_client_register(&client_cfg, &mut client_handle);
        if err != ESP_OK{
            log::error!("usb_host_client_register failed: {}", err);
        }
        else{
            log::info!("USB client registered.");
        }
    }

    loop{ //loop to test if the firmware works
        log::info!("Main loop alive (USB host ready, MSC not yet configured)...");
        thread::sleep(Duration::from_secs(5));
    }
}
