use crate::error::{Error, Result};
use rusb::{ConfigDescriptor, Context, DeviceHandle, Direction, TransferType};
use std::time::Duration;

pub struct Endpoint {
    interface: u8,
    setting: u8,
    address: u8,
    transfer_type: TransferType,
}

impl Endpoint {
    pub fn collect_in_endpoints(config: &ConfigDescriptor) -> Vec<Self> {
        config
            .interfaces()
            .flat_map(|iface| {
                let iface_num = iface.number();
                iface.descriptors().flat_map(move |desc| {
                    let setting = desc.setting_number();
                    desc.endpoint_descriptors()
                        .filter(|ep| ep.direction() == Direction::In)
                        .map(move |ep| Self {
                            interface: iface_num,
                            setting,
                            address: ep.address(),
                            transfer_type: ep.transfer_type(),
                        })
                })
            })
            .collect()
    }
    pub fn interface(&self) -> u8 {
        self.interface
    }
    pub fn address(&self) -> u8 {
        self.address
    }
    pub fn transfer_type_str(&self) -> &'static str {
        match self.transfer_type {
            TransferType::Control => "Control",
            TransferType::Isochronous => "Isochronous",
            TransferType::Bulk => "Bulk",
            TransferType::Interrupt => "Interrupt",
        }
    }
    pub fn read(
        &self,
        handle: &DeviceHandle<Context>,
        buf: &mut [u8],
        timeout: Duration,
    ) -> Result<usize> {
        match self.transfer_type {
            TransferType::Interrupt => handle
                .read_interrupt(self.address, buf, timeout)
                .map_err(Error::from),
            TransferType::Bulk => handle
                .read_bulk(self.address, buf, timeout)
                .map_err(Error::from),
            _ => Err(Error::UnsupportedTransferType),
        }
    }
}

impl std::fmt::Display for Endpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Interface {} (setting {}) - Endpoint 0x{:02X} ({})",
            self.interface,
            self.setting,
            self.address,
            self.transfer_type_str(),
        )
    }
}
