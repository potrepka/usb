# USB Device Listener

A Rust command-line application for connecting to USB devices and monitoring incoming data. Lists all USB devices, allows selection of configuration and endpoint, and displays received data in hexadecimal, decimal, and ASCII formats.

## Features

- Enumerate all USB devices with detailed information (Vendor ID, Product ID, class, etc.)
- List all device configurations with power requirements
- Display available IN endpoints with transfer types
- Real-time message display in hex, decimal, and ASCII
- Graceful shutdown with Ctrl+C

## Supported Transfer Types

This application supports reading from endpoints with the following transfer types:

- **Interrupt**: Low-latency, periodic transfers (keyboards, mice)
- **Bulk**: Large, non-time-critical transfers (storage devices)

## Prerequisites

- Cargo 1.85+ (for Rust 2024 Edition support)

### Installing Cargo

Install Cargo by installing Rust using [rustup](https://rustup.rs/#).

**macOS and Linux:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Windows:**

Download and run the installer.

## Building and Running

```bash
cargo run
```

For an optimized build:

```bash
cargo run --release
```

### Permissions

USB device access may require elevated privileges.

**macOS:**
```bash
sudo cargo run
```

**Linux:**
```bash
sudo cargo run
```

## Usage

1. **Select a device** from the list of available USB devices
2. **Select a configuration** (most devices have only one)
3. **Select an IN endpoint** to listen on
4. **View incoming data** displayed in multiple formats
5. **Press Ctrl+C** to exit gracefully

## USB Concepts

### Configuration

A configuration is a mode of operation for a USB device. A device can have multiple configurations, but only one is active at a time. Each configuration defines a set of interfaces and endpoints. Most devices have a single configuration.

### Interface

An interface is a logical grouping of endpoints that provides a specific function. A keyboard might have one interface for key input and another for media controls.

### Endpoint

An endpoint is a communication channel with a direction (IN or OUT) and transfer type:

- **Control**: Device configuration (all devices have endpoint 0)
- **Interrupt**: Low-latency, periodic transfers (keyboards, mice)
- **Bulk**: Large, non-time-critical transfers (storage devices)
- **Isochronous**: Constant-rate streaming (audio, video)

### Transfer Direction

- **IN**: Device to host (reading data)
- **OUT**: Host to device (sending data)

This application only lists IN endpoints since it monitors incoming data.

## Troubleshooting

### "USB error: could not open interface for exclusive access"

Another application or kernel driver is using the device. The application attempts to detach kernel drivers automatically, but some devices may require manually unloading drivers.

### "No IN endpoints found on this device"

The selected configuration has no input endpoints. Try a different configuration if available, or the device may only support output operations.

### "No configurations found on this device"

All configuration descriptors failed to load. This may indicate a device communication problem or an unsupported device.

### Device not appearing in list

- Check physical connection
- Try a different USB port
- Verify the device is powered on
- On macOS, check System Information > Hardware > USB to confirm the OS sees it

## License

MIT
