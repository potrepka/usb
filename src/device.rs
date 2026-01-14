use crate::error::{Error, Result};
use rusb::{ConfigDescriptor, Context, Device, DeviceDescriptor, DeviceHandle};

pub struct DeviceEntry {
    device: Device<Context>,
    descriptor: DeviceDescriptor,
    manufacturer: Option<String>,
    product: Option<String>,
    serial: Option<String>,
}

impl DeviceEntry {
    pub fn from_device(device: Device<Context>) -> Option<Self> {
        let descriptor = device.device_descriptor().ok()?;
        let (manufacturer, product, serial) = Self::fetch_strings(&device, &descriptor);
        Some(Self {
            device,
            descriptor,
            manufacturer,
            product,
            serial,
        })
    }
    fn fetch_strings(
        device: &Device<Context>,
        descriptor: &DeviceDescriptor,
    ) -> (Option<String>, Option<String>, Option<String>) {
        let handle = match device.open() {
            Ok(h) => h,
            Err(_) => return (None, None, None),
        };
        let manufacturer = descriptor
            .manufacturer_string_index()
            .and_then(|i| handle.read_string_descriptor_ascii(i).ok());
        let product = descriptor
            .product_string_index()
            .and_then(|i| handle.read_string_descriptor_ascii(i).ok());
        let serial = descriptor
            .serial_number_string_index()
            .and_then(|i| handle.read_string_descriptor_ascii(i).ok());
        (manufacturer, product, serial)
    }
    pub fn sort_key(&self) -> (u16, u16, u8, u8) {
        (
            self.descriptor.vendor_id(),
            self.descriptor.product_id(),
            self.device.bus_number(),
            self.device.address(),
        )
    }
    pub fn vendor_id(&self) -> u16 {
        self.descriptor.vendor_id()
    }
    pub fn product_id(&self) -> u16 {
        self.descriptor.product_id()
    }
    pub fn bus_number(&self) -> u8 {
        self.device.bus_number()
    }
    pub fn address(&self) -> u8 {
        self.device.address()
    }
    pub fn class_code(&self) -> u8 {
        self.descriptor.class_code()
    }
    pub fn sub_class_code(&self) -> u8 {
        self.descriptor.sub_class_code()
    }
    pub fn protocol_code(&self) -> u8 {
        self.descriptor.protocol_code()
    }
    pub fn manufacturer_str(&self) -> &str {
        self.manufacturer.as_deref().unwrap_or("Unknown")
    }
    pub fn product_str(&self) -> &str {
        self.product.as_deref().unwrap_or("Unknown")
    }
    pub fn serial(&self) -> Option<&str> {
        self.serial.as_deref()
    }
    pub fn num_configurations(&self) -> u8 {
        self.descriptor.num_configurations()
    }
    pub fn config_descriptor(&self, index: u8) -> Result<ConfigDescriptor> {
        self.device.config_descriptor(index).map_err(Error::from)
    }
    pub fn active_config_value(&self) -> Option<u8> {
        self.device
            .open()
            .ok()
            .and_then(|h| h.active_configuration().ok())
    }
    pub fn open_and_claim(&self, config_value: u8, interface: u8) -> Result<DeviceHandle<Context>> {
        let handle = self.device.open()?;
        if handle.kernel_driver_active(interface).unwrap_or(false) {
            handle.detach_kernel_driver(interface)?;
        }
        let current_config = handle.active_configuration().ok();
        if current_config != Some(config_value) {
            handle.set_active_configuration(config_value)?;
        }
        handle.claim_interface(interface)?;
        Ok(handle)
    }
}

impl std::fmt::Display for DeviceEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} - {}", self.manufacturer_str(), self.product_str())?;
        writeln!(f, "    Vendor ID:  0x{:04X}", self.vendor_id())?;
        writeln!(f, "    Product ID: 0x{:04X}", self.product_id())?;
        writeln!(f, "    Bus:        {}", self.bus_number())?;
        writeln!(f, "    Address:    {}", self.address())?;
        writeln!(f, "    Class:      0x{:02X}", self.class_code())?;
        writeln!(f, "    Subclass:   0x{:02X}", self.sub_class_code())?;
        write!(f, "    Protocol:   0x{:02X}", self.protocol_code())?;
        if let Some(serial) = self.serial() {
            write!(f, "\n    Serial:     {}", serial)?;
        }
        Ok(())
    }
}
