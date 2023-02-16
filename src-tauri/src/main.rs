#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{collections::HashSet, sync::Mutex};
use tauri_plugin_log::LogTarget;
use usb_enumeration::UsbDevice;

// modules
mod commands;
mod usb;

// GLOBALS
const USB_VENDOR_ID: u16 = 0x0483;
// const USB_PRODUCT_ID: u16 = 0x5740;
const USB_PRODUCT_DFU_ID: u16 = 0xDF11;
const GITHUB_API_URL: &str = "https://api.github.com";
const GITHUB_ORG: &str = "Pirate-MIDI";
const BRIDGE_GITHUB_REPO: &str = "Pirate-MIDI-BridgeOS";
const CLICK_GITHUB_REPO: &str = "Pirate-MIDI-CLiCK";

// state
#[derive(Default)]
pub struct UsbState {
    pub devices: Mutex<HashSet<UsbDevice>>,
}

fn main() {
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .menu(tauri::Menu::os_default(&context.package_info().name))
        .setup(|app| {
            // setup our global usb listener
            usb::setup_usb_listener(app.handle())?;

            // update state
            // app.

            Ok(())
        })
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([LogTarget::LogDir, LogTarget::Stdout, LogTarget::Webview])
                .build(),
        )
        // .menu(menu)
        .manage(UsbState {
            devices: Default::default(),
        })
        .invoke_handler(tauri::generate_handler![
            crate::commands::github::fetch_releases,
            crate::commands::github::fetch_asset,
            crate::commands::dfu::install_binary,
            crate::commands::dfu::enter_bootloader,
            crate::commands::dfu::prompt_local_file,
        ])
        .run(context)
        .expect("error while running tauri application");
}
