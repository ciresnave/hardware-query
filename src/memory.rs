use crate::Result;
use serde::{Deserialize, Serialize};
use sysinfo::System;

/// Memory type classification
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MemoryType {
    DDR3,
    DDR4,
    DDR5,
    LPDDR3,
    LPDDR4,
    LPDDR5,
    Unknown(String),
}

impl std::fmt::Display for MemoryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MemoryType::DDR3 => write!(f, "DDR3"),
            MemoryType::DDR4 => write!(f, "DDR4"),
            MemoryType::DDR5 => write!(f, "DDR5"),
            MemoryType::LPDDR3 => write!(f, "LPDDR3"),
            MemoryType::LPDDR4 => write!(f, "LPDDR4"),
            MemoryType::LPDDR5 => write!(f, "LPDDR5"),
            MemoryType::Unknown(name) => write!(f, "{name}"),
        }
    }
}

/// Memory module information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryModule {
    /// Memory module size in MB
    pub size_mb: u64,
    /// Memory type
    pub memory_type: MemoryType,
    /// Memory speed in MHz
    pub speed_mhz: u32,
    /// Memory manufacturer
    pub manufacturer: Option<String>,
    /// Memory part number
    pub part_number: Option<String>,
    /// Memory slot location
    pub slot: Option<String>,
    /// Memory voltage
    pub voltage: Option<f32>,
}

/// System memory information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryInfo {
    /// Total system memory in MB
    pub total_mb: u64,
    /// Available system memory in MB
    pub available_mb: u64,
    /// Used system memory in MB
    pub used_mb: u64,
    /// Memory usage percentage
    pub usage_percent: f32,
    /// Individual memory modules
    pub modules: Vec<MemoryModule>,
    /// Memory channels
    pub channels: u32,
    /// ECC support
    pub ecc_support: bool,
    /// Memory speed in MHz
    pub speed_mhz: u32,
    /// Memory bandwidth in GB/s
    pub bandwidth_gb_s: Option<f32>,
    /// Swap/virtual memory total in MB
    pub swap_total_mb: u64,
    /// Swap/virtual memory used in MB
    pub swap_used_mb: u64,
}

impl MemoryInfo {
    /// Query memory information from the system
    pub fn query() -> Result<Self> {
        let mut system = System::new_all();
        system.refresh_memory();

        let total_mb = system.total_memory() / (1024 * 1024);
        let available_mb = system.available_memory() / (1024 * 1024);
        let used_mb = system.used_memory() / (1024 * 1024);
        let usage_percent = (used_mb as f32 / total_mb as f32) * 100.0;

        let swap_total_mb = system.total_swap() / (1024 * 1024);
        let swap_used_mb = system.used_swap() / (1024 * 1024);

        Ok(Self {
            total_mb,
            available_mb,
            used_mb,
            usage_percent,
            modules: Self::detect_memory_modules()?,
            channels: Self::detect_memory_channels()?,
            ecc_support: Self::detect_ecc_support()?,
            speed_mhz: Self::detect_memory_speed()?,
            bandwidth_gb_s: Self::calculate_bandwidth(),
            swap_total_mb,
            swap_used_mb,
        })
    }

    /// Get total memory in GB
    pub fn total_gb(&self) -> f64 {
        self.total_mb as f64 / 1024.0
    }

    /// Get available memory in GB
    pub fn available_gb(&self) -> f64 {
        self.available_mb as f64 / 1024.0
    }

    /// Get used memory in GB
    pub fn used_gb(&self) -> f64 {
        self.used_mb as f64 / 1024.0
    }

    /// Get total memory in MB
    pub fn total_mb(&self) -> u64 {
        self.total_mb
    }

    /// Get available memory in MB
    pub fn available_mb(&self) -> u64 {
        self.available_mb
    }

    /// Get used memory in MB
    pub fn used_mb(&self) -> u64 {
        self.used_mb
    }

    /// Get memory usage percentage
    pub fn usage_percent(&self) -> f32 {
        self.usage_percent
    }

    /// Get memory modules
    pub fn modules(&self) -> &[MemoryModule] {
        &self.modules
    }

    /// Get number of memory channels
    pub fn channels(&self) -> u32 {
        self.channels
    }

    /// Check if ECC is supported
    pub fn ecc_support(&self) -> bool {
        self.ecc_support
    }

    /// Get memory speed in MHz
    pub fn speed_mhz(&self) -> u32 {
        self.speed_mhz
    }

    /// Get memory bandwidth in GB/s
    pub fn bandwidth_gb_s(&self) -> Option<f32> {
        self.bandwidth_gb_s
    }

    /// Get swap total in GB
    pub fn swap_total_gb(&self) -> f64 {
        self.swap_total_mb as f64 / 1024.0
    }

    /// Get swap used in GB
    pub fn swap_used_gb(&self) -> f64 {
        self.swap_used_mb as f64 / 1024.0
    }

    /// Check if system has sufficient memory for a workload
    pub fn has_sufficient_memory(&self, required_gb: f64) -> bool {
        self.available_gb() >= required_gb
    }

    fn detect_memory_modules() -> Result<Vec<MemoryModule>> {
        // Platform-specific implementation would go here
        // For now, return a placeholder module
        Ok(vec![MemoryModule {
            size_mb: 8192,
            memory_type: MemoryType::DDR4,
            speed_mhz: 3200,
            manufacturer: Some("Unknown".to_string()),
            part_number: None,
            slot: Some("DIMM1".to_string()),
            voltage: Some(1.35),
        }])
    }

    fn detect_memory_channels() -> Result<u32> {
        // Platform-specific implementation would go here
        Ok(2) // Assume dual channel
    }

    fn detect_ecc_support() -> Result<bool> {
        // Platform-specific implementation would go here
        Ok(false)
    }

    fn detect_memory_speed() -> Result<u32> {
        // Platform-specific implementation would go here
        Ok(3200)
    }

    fn calculate_bandwidth() -> Option<f32> {
        // Calculate theoretical bandwidth
        // For DDR4-3200 dual channel: 3200 MHz * 2 channels * 8 bytes = 51.2 GB/s
        Some(51.2)
    }
}
