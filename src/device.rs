use crate::error::{Error, Result};
use nusb::descriptors::ConfigurationDescriptor;
use nusb::{Device, DeviceInfo, Interface, MaybeFuture};

pub struct DeviceEntry {
    info: DeviceInfo,
}

impl DeviceEntry {
    pub fn from_info(info: DeviceInfo) -> Self {
        Self { info }
    }
    pub fn sort_key(&self) -> (u16, u16, u8) {
        (
            self.info.vendor_id(),
            self.info.product_id(),
            self.info.device_address(),
        )
    }
    pub fn vendor_id(&self) -> u16 {
        self.info.vendor_id()
    }
    pub fn product_id(&self) -> u16 {
        self.info.product_id()
    }
    pub fn device_address(&self) -> u8 {
        self.info.device_address()
    }
    pub fn class_code(&self) -> u8 {
        self.info.class()
    }
    pub fn sub_class_code(&self) -> u8 {
        self.info.subclass()
    }
    pub fn protocol_code(&self) -> u8 {
        self.info.protocol()
    }
    pub fn manufacturer_str(&self) -> &str {
        self.info.manufacturer_string().unwrap_or("Unknown")
    }
    pub fn product_str(&self) -> &str {
        self.info.product_string().unwrap_or("Unknown")
    }
    pub fn serial(&self) -> Option<&str> {
        self.info.serial_number()
    }
    pub fn bus_id(&self) -> &str {
        self.info.bus_id()
    }
    pub fn open(&self) -> Result<Device> {
        self.info.open().wait().map_err(Error::from)
    }
    pub fn open_and_claim(&self, config_value: u8, interface_num: u8, alt_setting: u8) -> Result<(Device, Interface)> {
        let device = self.info.open().wait()?;
        let active = device.active_configuration().ok().map(|c: ConfigurationDescriptor| c.configuration_value());
        if active != Some(config_value) {
            device.set_configuration(config_value).wait()?;
        }
        let interface = device.detach_and_claim_interface(interface_num).wait()?;
        if alt_setting != 0 {
            interface.set_alt_setting(alt_setting).wait()?;
        }
        Ok((device, interface))
    }
}

impl std::fmt::Display for DeviceEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} - {}", self.manufacturer_str(), self.product_str())?;
        writeln!(f, "    Vendor ID:  0x{:04X}", self.vendor_id())?;
        writeln!(f, "    Product ID: 0x{:04X}", self.product_id())?;
        writeln!(f, "    Bus:        {}", self.bus_id())?;
        writeln!(f, "    Address:    {}", self.device_address())?;
        writeln!(f, "    Class:      0x{:02X}", self.class_code())?;
        writeln!(f, "    Subclass:   0x{:02X}", self.sub_class_code())?;
        write!(f, "    Protocol:   0x{:02X}", self.protocol_code())?;
        if let Some(serial) = self.serial() {
            write!(f, "\n    Serial:     {}", serial)?;
        }
        Ok(())
    }
}
