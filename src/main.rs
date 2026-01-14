mod device;
mod endpoint;
mod error;
mod input;
mod message;

use device::DeviceEntry;
use endpoint::Endpoint;
use error::{Error, Result};
use input::prompt_selection;
use message::print_message;
use rusb::{Context, DeviceHandle, UsbContext};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

const BUFFER_SIZE: usize = 512;
const READ_TIMEOUT_MS: u64 = 1000;

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
    }
}

fn run() -> Result<()> {
    let context = Context::new()?;
    let mut entries: Vec<DeviceEntry> = context
        .devices()?
        .iter()
        .filter_map(DeviceEntry::from_device)
        .collect();
    entries.sort_by_key(|e| e.sort_key());
    if entries.is_empty() {
        return Err(Error::NoDevices);
    }
    println!("Available USB devices:\n");
    for (i, entry) in entries.iter().enumerate() {
        println!("[{}] {}\n", i + 1, entry);
    }
    let device = &entries[prompt_selection("Select device", entries.len()).ok_or(Error::UserCancelled)?];
    let num_configs = device.num_configurations();
    if num_configs == 0 {
        return Err(Error::NoConfigurations);
    }
    let active_config = device.active_config_value();
    println!("\nAvailable configurations:\n");
    let mut configs = Vec::with_capacity(num_configs as usize);
    for i in 0..num_configs {
        match device.config_descriptor(i) {
            Ok(config) => {
                let config_value = config.number();
                let is_active = active_config == Some(config_value);
                let active_marker = if is_active { " (active)" } else { "" };
                println!(
                    "[{}] Configuration {}{} - {} interface(s), max {}mA",
                    i + 1,
                    config_value,
                    active_marker,
                    config.num_interfaces(),
                    u16::from(config.max_power()) * 2,
                );
                configs.push(config);
            }
            Err(e) => {
                eprintln!("[{}] Unknown - Error: {}", i + 1, e);
            }
        }
    }
    if configs.is_empty() {
        return Err(Error::NoConfigurations);
    }
    println!();
    let config = &configs[prompt_selection("Select configuration", configs.len()).ok_or(Error::UserCancelled)?];
    let config_value = config.number();
    let endpoints = Endpoint::collect_in_endpoints(config);
    if endpoints.is_empty() {
        return Err(Error::NoEndpoints);
    }
    println!("\nAvailable IN endpoints:\n");
    for (i, ep) in endpoints.iter().enumerate() {
        println!("[{}] {}", i + 1, ep);
    }
    println!();
    let endpoint = &endpoints[prompt_selection("Select endpoint", endpoints.len()).ok_or(Error::UserCancelled)?];
    let handle = device.open_and_claim(config_value, endpoint.interface())?;
    read_loop(&handle, endpoint)
}

fn read_loop(handle: &DeviceHandle<Context>, endpoint: &Endpoint) -> Result<()> {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })?;
    println!(
        "\nListening on endpoint 0x{:02X} (Ctrl+C to exit)...\n",
        endpoint.address(),
    );
    let mut buf = [0u8; BUFFER_SIZE];
    let timeout = Duration::from_millis(READ_TIMEOUT_MS);
    let mut result = Ok(());
    while running.load(Ordering::SeqCst) {
        match endpoint.read(handle, &mut buf, timeout) {
            Ok(len) => print_message(&buf[..len]),
            Err(Error::Usb(rusb::Error::Timeout)) => continue,
            Err(e) => {
                result = Err(e);
                break;
            }
        }
    }
    println!("\nShutting down...");
    let _ = handle.release_interface(endpoint.interface());
    result
}
