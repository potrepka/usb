mod device;
mod endpoint;
mod error;
mod input;
mod message;

use device::DeviceEntry;
use endpoint::{Endpoint, TransferType};
use error::{Error, Result};
use input::prompt_selection;
use message::print_message;
use nusb::descriptors::ConfigurationDescriptor;
use nusb::transfer::{Bulk, In, Interrupt, TransferError};
use nusb::{Interface, MaybeFuture};
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
    let mut entries: Vec<DeviceEntry> = nusb::list_devices()
        .wait()?
        .map(DeviceEntry::from_info)
        .collect();
    entries.sort_by_key(|e: &DeviceEntry| e.sort_key());
    if entries.is_empty() {
        return Err(Error::NoDevices);
    }
    println!("Available USB devices:\n");
    for (i, entry) in entries.iter().enumerate() {
        println!("[{}] {}\n", i + 1, entry);
    }
    let device = &entries[prompt_selection("Select device", entries.len()).ok_or(Error::UserCancelled)?];
    let opened = device.open()?;
    let configs: Vec<ConfigurationDescriptor> = opened.configurations().collect();
    if configs.is_empty() {
        return Err(Error::NoConfigurations);
    }
    println!("\nAvailable configurations:\n");
    let active_config = opened.active_configuration().ok();
    let active_value = active_config.as_ref().map(|c: &ConfigurationDescriptor| c.configuration_value());
    for (i, config) in configs.iter().enumerate() {
        let config_value = config.configuration_value();
        let is_active = active_value == Some(config_value);
        let active_marker = if is_active { " (active)" } else { "" };
        println!(
            "[{}] Configuration {}{} - {} interface(s), max {}mA",
            i + 1,
            config_value,
            active_marker,
            config.num_interfaces(),
            config.max_power(),
        );
    }
    println!();
    let config = &configs[prompt_selection("Select configuration", configs.len()).ok_or(Error::UserCancelled)?];
    let config_value = config.configuration_value();
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
    drop(opened);
    let (_device, interface) = device.open_and_claim(config_value, endpoint.interface(), endpoint.setting())?;
    read_loop(&interface, endpoint)
}

fn read_loop(interface: &Interface, endpoint: &Endpoint) -> Result<()> {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })?;
    println!(
        "\nListening on endpoint 0x{:02X} (Ctrl+C to exit)...\n",
        endpoint.address(),
    );
    let timeout = Duration::from_millis(READ_TIMEOUT_MS);
    match endpoint.transfer_type() {
        TransferType::Bulk => {
            let mut ep = interface.endpoint::<Bulk, In>(endpoint.address())?;
            read_loop_impl(&mut ep, &running, timeout)
        }
        TransferType::Interrupt => {
            let mut ep = interface.endpoint::<Interrupt, In>(endpoint.address())?;
            read_loop_impl(&mut ep, &running, timeout)
        }
    }
}

fn read_loop_impl<T: nusb::transfer::BulkOrInterrupt>(
    ep: &mut nusb::Endpoint<T, In>,
    running: &Arc<AtomicBool>,
    timeout: Duration,
) -> Result<()> {
    let mut result = Ok(());
    while running.load(Ordering::SeqCst) {
        let buf = ep.allocate(BUFFER_SIZE);
        let completion = ep.transfer_blocking(buf, timeout);
        match completion.status {
            Ok(()) => print_message(&completion.buffer[..completion.actual_len]),
            Err(TransferError::Cancelled) => break,
            Err(TransferError::Stall) => {
                ep.clear_halt().wait()?;
            }
            Err(e) => {
                result = Err(Error::Transfer(e));
                break;
            }
        }
    }
    println!("\nShutting down...");
    result
}
