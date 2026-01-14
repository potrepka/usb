use std::fmt;

#[derive(Debug)]
pub enum Error {
    Usb(nusb::Error),
    Transfer(nusb::transfer::TransferError),
    NoDevices,
    NoConfigurations,
    NoEndpoints,
    UserCancelled,
    SignalHandler(ctrlc::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Usb(e) => write!(f, "USB error: {}", e),
            Self::Transfer(e) => write!(f, "Transfer error: {}", e),
            Self::NoDevices => write!(f, "No USB devices found"),
            Self::NoConfigurations => write!(f, "No configurations found on this device"),
            Self::NoEndpoints => write!(f, "No IN endpoints found on this device"),
            Self::UserCancelled => write!(f, "User cancelled"),
            Self::SignalHandler(e) => write!(f, "Failed to set signal handler: {}", e),
        }
    }
}

impl std::error::Error for Error {}

impl From<nusb::Error> for Error {
    fn from(e: nusb::Error) -> Self {
        Self::Usb(e)
    }
}

impl From<nusb::transfer::TransferError> for Error {
    fn from(e: nusb::transfer::TransferError) -> Self {
        Self::Transfer(e)
    }
}

impl From<ctrlc::Error> for Error {
    fn from(e: ctrlc::Error) -> Self {
        Self::SignalHandler(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
