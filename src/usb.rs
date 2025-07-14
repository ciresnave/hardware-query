use crate::Result;
use serde::{Deserialize, Serialize};

/// USB device information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct USBDevice {
    /// USB vendor ID
    pub vendor_id: String,
    /// USB product ID
    pub product_id: String,
    /// Vendor name
    pub vendor_name: String,
    /// Product name
    pub product_name: String,
    /// USB device class
    pub device_class: String,
    /// USB version (1.0, 1.1, 2.0, 3.0, etc.)
    pub usb_version: String,
    /// Serial number (if available)
    pub serial_number: Option<String>,
    /// Bus number
    pub bus_number: u8,
    /// Device address
    pub device_address: u8,
    /// Port path
    pub port_path: Option<String>,
    /// Driver name (if loaded)
    pub driver: Option<String>,
    /// Is device currently connected
    pub connected: bool,
}

impl USBDevice {
    /// Query all USB devices
    pub fn query_all() -> Result<Vec<Self>> {
        // Platform-specific implementation would go here
        // For now, return empty vector
        Ok(vec![])
    }

    /// Get vendor ID
    pub fn vendor_id(&self) -> &str {
        &self.vendor_id
    }

    /// Get product ID
    pub fn product_id(&self) -> &str {
        &self.product_id
    }

    /// Get vendor name
    pub fn vendor_name(&self) -> &str {
        &self.vendor_name
    }

    /// Get product name
    pub fn product_name(&self) -> &str {
        &self.product_name
    }

    /// Get device class
    pub fn device_class(&self) -> &str {
        &self.device_class
    }

    /// Get USB version
    pub fn usb_version(&self) -> &str {
        &self.usb_version
    }

    /// Check if device is connected
    pub fn is_connected(&self) -> bool {
        self.connected
    }

    /// Check if device is a storage device
    pub fn is_storage_device(&self) -> bool {
        self.device_class.to_lowercase().contains("mass storage")
            || self.device_class.to_lowercase().contains("storage")
    }

    /// Check if device is an input device
    pub fn is_input_device(&self) -> bool {
        self.device_class.to_lowercase().contains("hid")
            || self.device_class.to_lowercase().contains("human interface")
            || self.device_class.to_lowercase().contains("input")
    }

    /// Check if device is an audio device
    pub fn is_audio_device(&self) -> bool {
        self.device_class.to_lowercase().contains("audio")
    }

    /// Check if device is a video device
    pub fn is_video_device(&self) -> bool {
        self.device_class.to_lowercase().contains("video")
            || self.device_class.to_lowercase().contains("camera")
    }

    /// Check if device supports USB 3.0 or higher
    pub fn is_high_speed(&self) -> bool {
        self.usb_version.contains("3.")
            || self.usb_version.contains("3")
                && !self.usb_version.contains("2.")
                && !self.usb_version.contains("1.")
    }
}
