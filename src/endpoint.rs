use nusb::descriptors::{ConfigurationDescriptor, TransferType as NusbTransferType};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TransferType {
    Bulk,
    Interrupt,
}

pub struct Endpoint {
    interface_num: u8,
    setting: u8,
    address: u8,
    transfer_type: TransferType,
}

impl Endpoint {
    pub fn collect_in_endpoints(config: &ConfigurationDescriptor) -> Vec<Self> {
        let mut endpoints = Vec::new();
        for iface in config.interfaces() {
            for alt in iface.alt_settings() {
                let interface_num = alt.interface_number();
                let setting = alt.alternate_setting();
                for ep in alt.endpoints() {
                    if ep.direction() != nusb::transfer::Direction::In {
                        continue;
                    }
                    let transfer_type = match ep.transfer_type() {
                        NusbTransferType::Bulk => TransferType::Bulk,
                        NusbTransferType::Interrupt => TransferType::Interrupt,
                        _ => continue,
                    };
                    endpoints.push(Self {
                        interface_num,
                        setting,
                        address: ep.address(),
                        transfer_type,
                    });
                }
            }
        }
        endpoints
    }
    pub fn interface(&self) -> u8 {
        self.interface_num
    }
    pub fn setting(&self) -> u8 {
        self.setting
    }
    pub fn address(&self) -> u8 {
        self.address
    }
    pub fn transfer_type(&self) -> TransferType {
        self.transfer_type
    }
    pub fn transfer_type_str(&self) -> &'static str {
        match self.transfer_type {
            TransferType::Bulk => "Bulk",
            TransferType::Interrupt => "Interrupt",
        }
    }
}

impl std::fmt::Display for Endpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Interface {} (setting {}) - Endpoint 0x{:02X} ({})",
            self.interface_num,
            self.setting,
            self.address,
            self.transfer_type_str(),
        )
    }
}
