use crate::Result;
use serde::{Deserialize, Serialize};

/// PCI device information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PCIDevice {
    /// PCI device ID (vendor:device)
    pub device_id: String,
    /// PCI vendor name
    pub vendor_name: String,
    /// PCI device name
    pub device_name: String,
    /// PCI bus location
    pub bus_location: String,
    /// PCI device class
    pub device_class: String,
    /// PCI subsystem ID
    pub subsystem_id: Option<String>,
    /// Driver name (if loaded)
    pub driver: Option<String>,
    /// Device revision
    pub revision: Option<String>,
    /// IRQ number
    pub irq: Option<u32>,
    /// Memory regions
    pub memory_regions: Vec<String>,
}

impl PCIDevice {
    /// Query all PCI devices
    pub fn query_all() -> Result<Vec<Self>> {
        // Platform-specific implementation would go here
        // For now, return empty vector
        Ok(vec![])
    }

    /// Get device ID
    pub fn device_id(&self) -> &str {
        &self.device_id
    }

    /// Get vendor name
    pub fn vendor_name(&self) -> &str {
        &self.vendor_name
    }

    /// Get device name
    pub fn device_name(&self) -> &str {
        &self.device_name
    }

    /// Get device class
    pub fn device_class(&self) -> &str {
        &self.device_class
    }

    /// Check if device is a graphics card
    pub fn is_graphics_device(&self) -> bool {
        self.device_class.to_lowercase().contains("vga")
            || self.device_class.to_lowercase().contains("display")
            || self.device_class.to_lowercase().contains("graphics")
    }

    /// Check if device is a network controller
    pub fn is_network_device(&self) -> bool {
        self.device_class.to_lowercase().contains("network")
            || self.device_class.to_lowercase().contains("ethernet")
            || self.device_class.to_lowercase().contains("wireless")
    }

    /// Check if device is a storage controller
    pub fn is_storage_device(&self) -> bool {
        self.device_class.to_lowercase().contains("storage")
            || self.device_class.to_lowercase().contains("sata")
            || self.device_class.to_lowercase().contains("nvme")
            || self.device_class.to_lowercase().contains("scsi")
    }
}
