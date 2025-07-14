use crate::{
    BatteryInfo, CPUInfo, GPUInfo, HardwareQueryError,
    MemoryInfo, NetworkInfo, NPUInfo, PCIDevice, Result, StorageInfo, ThermalInfo, TPUInfo, USBDevice,
    ARMHardwareInfo, FPGAInfo,
};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;

/// Complete system hardware information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareInfo {
    /// Timestamp when the hardware information was collected
    pub timestamp: u64,
    /// CPU information
    pub cpu: CPUInfo,
    /// GPU information (multiple GPUs supported)
    pub gpus: Vec<GPUInfo>,
    /// NPU information (Neural Processing Units)
    pub npus: Vec<NPUInfo>,
    /// TPU information (Tensor Processing Units)
    pub tpus: Vec<TPUInfo>,
    /// ARM-specific hardware information (if running on ARM)
    pub arm_hardware: Option<ARMHardwareInfo>,
    /// FPGA accelerators
    pub fpgas: Vec<FPGAInfo>,
    /// Memory information
    pub memory: MemoryInfo,
    /// Storage devices
    pub storage_devices: Vec<StorageInfo>,
    /// Network interfaces
    pub network_interfaces: Vec<NetworkInfo>,
    /// Battery information (if available)
    pub battery: Option<BatteryInfo>,
    /// Thermal sensors and fans
    pub thermal: ThermalInfo,
    /// PCI devices
    pub pci_devices: Vec<PCIDevice>,
    /// USB devices
    pub usb_devices: Vec<USBDevice>,
}

impl HardwareInfo {
    /// Query all available hardware information
    pub fn query() -> Result<Self> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| HardwareQueryError::unknown(format!("Failed to get timestamp: {e}")))?
            .as_secs();

        Ok(Self {
            timestamp,
            cpu: CPUInfo::query()?,
            gpus: GPUInfo::query_all()?,
            npus: NPUInfo::query_all()?,
            tpus: TPUInfo::query_all()?,
            arm_hardware: ARMHardwareInfo::detect().ok().flatten(),
            fpgas: FPGAInfo::detect_fpgas().unwrap_or_default(),
            memory: MemoryInfo::query()?,
            storage_devices: StorageInfo::query_all()?,
            network_interfaces: NetworkInfo::query_all()?,
            battery: BatteryInfo::query().ok(),
            thermal: ThermalInfo::query()?,
            pci_devices: PCIDevice::query_all()?,
            usb_devices: USBDevice::query_all()?,
        })
    }

    /// Get CPU information
    pub fn cpu(&self) -> &CPUInfo {
        &self.cpu
    }

    /// Get GPU information
    pub fn gpus(&self) -> &[GPUInfo] {
        &self.gpus
    }

    /// Get NPU information
    pub fn npus(&self) -> &[NPUInfo] {
        &self.npus
    }

    /// Get TPU information
    pub fn tpus(&self) -> &[TPUInfo] {
        &self.tpus
    }

    /// Get ARM hardware information (if available)
    pub fn arm_hardware(&self) -> Option<&ARMHardwareInfo> {
        self.arm_hardware.as_ref()
    }

    /// Get FPGA accelerator information
    pub fn fpgas(&self) -> &[FPGAInfo] {
        &self.fpgas
    }

    /// Get memory information
    pub fn memory(&self) -> &MemoryInfo {
        &self.memory
    }

    /// Get storage devices
    pub fn storage_devices(&self) -> &[StorageInfo] {
        &self.storage_devices
    }

    /// Get network interfaces
    pub fn network_interfaces(&self) -> &[NetworkInfo] {
        &self.network_interfaces
    }

    /// Get battery information (if available)
    pub fn battery(&self) -> Option<&BatteryInfo> {
        self.battery.as_ref()
    }

    /// Get thermal information
    pub fn thermal(&self) -> &ThermalInfo {
        &self.thermal
    }

    /// Get PCI devices
    pub fn pci_devices(&self) -> &[PCIDevice] {
        &self.pci_devices
    }

    /// Get USB devices
    pub fn usb_devices(&self) -> &[USBDevice] {
        &self.usb_devices
    }

    /// Check if system is ARM-based
    pub fn is_arm_system(&self) -> bool {
        self.arm_hardware.is_some()
    }

    /// Check if system has FPGA accelerators
    pub fn has_fpgas(&self) -> bool {
        !self.fpgas.is_empty()
    }

    /// Get count of specialized accelerators (NPUs + TPUs + FPGAs)
    pub fn accelerator_count(&self) -> usize {
        self.npus.len() + self.tpus.len() + self.fpgas.len()
    }

    /// Get comprehensive accelerator information
    pub fn accelerator_summary(&self) -> HashMap<String, usize> {
        let mut summary = HashMap::new();
        
        if !self.npus.is_empty() {
            summary.insert("NPUs".to_string(), self.npus.len());
        }
        if !self.tpus.is_empty() {
            summary.insert("TPUs".to_string(), self.tpus.len());
        }
        if !self.fpgas.is_empty() {
            summary.insert("FPGAs".to_string(), self.fpgas.len());
        }
        
        summary
    }

    /// Export hardware information as JSON
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self).map_err(Into::into)
    }

    /// Import hardware information from JSON
    pub fn from_json(json: &str) -> Result<Self> {
        serde_json::from_str(json).map_err(Into::into)
    }

    /// Get a summary of the most important hardware information
    pub fn summary(&self) -> HardwareSummary {
        HardwareSummary {
            cpu_model: format!("{} {}", self.cpu.vendor(), self.cpu.model_name()),
            cpu_cores: self.cpu.physical_cores(),
            cpu_threads: self.cpu.logical_cores(),
            total_memory_gb: self.memory.total_gb(),
            primary_gpu: self.gpus.first().map(|gpu| {
                format!(
                    "{} {} ({} GB)",
                    gpu.vendor(),
                    gpu.model_name(),
                    gpu.memory_gb()
                )
            }),
            storage_total_gb: self
                .storage_devices
                .iter()
                .map(|storage| storage.capacity_gb())
                .sum(),
        }
    }
}

/// Summary of key hardware specifications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareSummary {
    pub cpu_model: String,
    pub cpu_cores: u32,
    pub cpu_threads: u32,
    pub total_memory_gb: f64,
    pub primary_gpu: Option<String>,
    pub storage_total_gb: f64,
}

impl std::fmt::Display for HardwareSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Hardware Summary:")?;
        writeln!(
            f,
            "  CPU: {} ({} cores, {} threads)",
            self.cpu_model, self.cpu_cores, self.cpu_threads
        )?;
        writeln!(f, "  Memory: {:.1} GB", self.total_memory_gb)?;
        if let Some(gpu) = &self.primary_gpu {
            writeln!(f, "  Primary GPU: {gpu}")?;
        }
        writeln!(f, "  Total Storage: {:.1} GB", self.storage_total_gb)?;
        Ok(())
    }
}
