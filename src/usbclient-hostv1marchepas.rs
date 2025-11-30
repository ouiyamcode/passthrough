/*use esp_idf_svc::log::EspLogger;
use esp_idf_svc::sys::*;
use std::{thread, time::Duration};

/// USB host client event callback.
/// Called by the host core when events occur.
unsafe extern "C" fn client_event_cb(
    event_msg: *const usb_host_client_event_msg_t,
    _arg: *mut core::ffi::c_void,
) {
    if event_msg.is_null() {
        log::warn!("USB client callback: null event pointer");
        return;
    }

    let msg = *event_msg;
    log::info!("USB event callback: event = {}", msg.event);
}

fn main() {
    esp_idf_svc::sys::link_patches();
    EspLogger::initialize_default();
    log::info!("Booting, initializing USB host...");

    let mut client_handle: usb_host_client_handle_t = core::ptr::null_mut();

    unsafe {
        // 1) Install USB Host core
        let host_cfg = usb_host_config_t {
            intr_flags: ESP_INTR_FLAG_LEVEL1 as i32,
            ..core::mem::zeroed()
        };

        let err = usb_host_install(&host_cfg);
        if err != ESP_OK {
            log::error!("usb_host_install failed: {}", err);
            return;
        }
        log::info!("USB host installed successfully.");

        // 2) Register client
        let client_cfg = usb_host_client_config_t {
            is_synchronous: false,
            max_num_event_msg: 5,
            __bindgen_anon_1: usb_host_client_config_t__bindgen_ty_1 {
                async_: usb_host_client_config_t__bindgen_ty_1__bindgen_ty_1 {
                    client_event_callback: Some(client_event_cb),
                    callback_arg: core::ptr::null_mut(),
                },
            },
        };

        let err = usb_host_client_register(&client_cfg, &mut client_handle);
        if err != ESP_OK {
            log::error!("usb_host_client_register failed: {}", err);
            return;
        }

        log::info!("USB client registered successfully! handle = {:?}", client_handle);
    }

    // 3) Event loop
    loop {
        unsafe {
            let res = usb_host_client_handle_events(client_handle, 10);

            match res {
                ESP_OK => {}
                ESP_ERR_TIMEOUT => {}
                ESP_ERR_INVALID_STATE => {
                    log::error!("USB host is not installed!");
                }
                ESP_ERR_INVALID_ARG => {
                    log::error!("Invalid argument for usb_host_client_handle_events");
                }
                other => {
                    log::warn!("usb_host_client_handle_events returned {}", other);
                }
            }
        }

        thread::sleep(Duration::from_millis(10));
    }
}*/
