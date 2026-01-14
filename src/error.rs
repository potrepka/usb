use std::fmt;

#[derive(Debug)]
pub enum Error {
    Usb(rusb::Error),
    NoDevices,
    NoConfigurations,
    NoEndpoints,
    UnsupportedTransferType,
    UserCancelled,
    SignalHandler(ctrlc::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Usb(e) => write!(f, "USB error: {}", e),
            Self::NoDevices => write!(f, "No USB devices found"),
            Self::NoConfigurations => write!(f, "No configurations found on this device"),
            Self::NoEndpoints => write!(f, "No IN endpoints found on this device"),
            Self::UnsupportedTransferType => write!(f, "Unsupported transfer type"),
            Self::UserCancelled => write!(f, "User cancelled"),
            Self::SignalHandler(e) => write!(f, "Failed to set signal handler: {}", e),
        }
    }
}

impl std::error::Error for Error {}

impl From<rusb::Error> for Error {
    fn from(e: rusb::Error) -> Self {
        Self::Usb(e)
    }
}

impl From<ctrlc::Error> for Error {
    fn from(e: ctrlc::Error) -> Self {
        Self::SignalHandler(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
