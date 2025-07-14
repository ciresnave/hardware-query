use crate::Result;
use serde::{Deserialize, Serialize};

/// Storage device type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StorageType {
    SSD,
    HDD,
    NVMe,
    EMmc,
    SD,
    USB,
    Unknown,
}

impl std::fmt::Display for StorageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StorageType::SSD => write!(f, "SSD"),
            StorageType::HDD => write!(f, "HDD"),
            StorageType::NVMe => write!(f, "NVMe"),
            StorageType::EMmc => write!(f, "eMMC"),
            StorageType::SD => write!(f, "SD Card"),
            StorageType::USB => write!(f, "USB"),
            StorageType::Unknown => write!(f, "Unknown"),
        }
    }
}

/// Storage device information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageInfo {
    /// Device name/model
    pub model: String,
    /// Storage type
    pub storage_type: StorageType,
    /// Total capacity in GB
    pub capacity_gb: f64,
    /// Available space in GB
    pub available_gb: f64,
    /// Used space in GB
    pub used_gb: f64,
    /// Mount point or drive letter
    pub mount_point: String,
    /// File system type
    pub file_system: Option<String>,
    /// Is removable
    pub removable: bool,
    /// Read speed in MB/s (if available)
    pub read_speed_mb_s: Option<f32>,
    /// Write speed in MB/s (if available)
    pub write_speed_mb_s: Option<f32>,
}

impl StorageInfo {
    /// Query all storage devices
    pub fn query_all() -> Result<Vec<Self>> {
        // Note: The current version of sysinfo doesn't expose disk APIs
        // This would be implemented using platform-specific APIs
        let storage_devices = vec![Self {
            model: "System Disk".to_string(),
            storage_type: StorageType::SSD,
            capacity_gb: 256.0,
            available_gb: 128.0,
            used_gb: 128.0,
            mount_point: "C:".to_string(),
            file_system: Some("NTFS".to_string()),
            removable: false,
            read_speed_mb_s: None,
            write_speed_mb_s: None,
        }];

        Ok(storage_devices)
    }

    /// Get device model/name
    pub fn model(&self) -> &str {
        &self.model
    }

    /// Get storage type
    pub fn drive_type(&self) -> &StorageType {
        &self.storage_type
    }

    /// Get total capacity in GB
    pub fn capacity_gb(&self) -> f64 {
        self.capacity_gb
    }

    /// Get available space in GB
    pub fn available_gb(&self) -> f64 {
        self.available_gb
    }

    /// Get used space in GB
    pub fn used_gb(&self) -> f64 {
        self.used_gb
    }

    /// Get usage percentage
    pub fn usage_percent(&self) -> f64 {
        if self.capacity_gb > 0.0 {
            (self.used_gb / self.capacity_gb) * 100.0
        } else {
            0.0
        }
    }

    /// Check if device has sufficient free space
    pub fn has_free_space(&self, required_gb: f64) -> bool {
        self.available_gb >= required_gb
    }
}
