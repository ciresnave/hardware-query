//! Hardware query builder for customizable hardware information gathering
//!
//! This module provides a builder pattern for creating customized hardware queries,
//! allowing developers to request only the information they need without the
//! overhead of collecting all available hardware data.

use crate::{
    HardwareInfo, CPUInfo, GPUInfo, MemoryInfo, StorageInfo, NetworkInfo, 
    BatteryInfo, ThermalInfo, PCIDevice, USBDevice, VirtualizationInfo,
    Result,
};

#[cfg(feature = "monitoring")]
use crate::PowerProfile;

use serde::{Serialize, Deserialize};

/// Hardware query builder for selective information gathering
pub struct HardwareQueryBuilder {
    include_cpu: bool,
    include_gpu: bool,
    include_memory: bool,
    include_storage: bool,
    include_network: bool,
    include_battery: bool,
    include_thermal: bool,
    include_pci: bool,
    include_usb: bool,
    include_virtualization: bool,
    include_capabilities: bool,
    
    #[cfg(feature = "monitoring")]
    include_power: bool,
}

/// Customizable hardware information result
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomHardwareInfo {
    pub cpu: Option<CPUInfo>,
    pub gpus: Vec<GPUInfo>,
    pub memory: Option<MemoryInfo>,
    pub storage_devices: Vec<StorageInfo>,
    pub network_interfaces: Vec<NetworkInfo>,
    pub battery: Option<BatteryInfo>,
    pub thermal: Option<ThermalInfo>,
    pub pci_devices: Vec<PCIDevice>,
    pub usb_devices: Vec<USBDevice>,
    pub virtualization: Option<VirtualizationInfo>,
    
    #[cfg(feature = "monitoring")]
    pub power_profile: Option<PowerProfile>,
    
    /// Timestamp when this information was collected
    pub timestamp: std::time::SystemTime,
    /// Query execution time in milliseconds
    pub query_time_ms: u64,
    /// Which components were requested
    pub requested_components: Vec<String>,
}

impl HardwareQueryBuilder {
    /// Create a new hardware query builder
    pub fn new() -> Self {
        Self {
            include_cpu: false,
            include_gpu: false,
            include_memory: false,
            include_storage: false,
            include_network: false,
            include_battery: false,
            include_thermal: false,
            include_pci: false,
            include_usb: false,
            include_virtualization: false,
            include_capabilities: false,
            
            #[cfg(feature = "monitoring")]
            include_power: false,
        }
    }

    /// Include CPU information in the query
    pub fn with_cpu(mut self) -> Self {
        self.include_cpu = true;
        self
    }

    /// Include GPU information in the query
    pub fn with_gpu(mut self) -> Self {
        self.include_gpu = true;
        self
    }

    /// Include memory information in the query
    pub fn with_memory(mut self) -> Self {
        self.include_memory = true;
        self
    }

    /// Include storage information in the query
    pub fn with_storage(mut self) -> Self {
        self.include_storage = true;
        self
    }

    /// Include network information in the query
    pub fn with_network(mut self) -> Self {
        self.include_network = true;
        self
    }

    /// Include battery information in the query
    pub fn with_battery(mut self) -> Self {
        self.include_battery = true;
        self
    }

    /// Include thermal information in the query
    pub fn with_thermal(mut self) -> Self {
        self.include_thermal = true;
        self
    }

    /// Include PCI device information in the query
    pub fn with_pci(mut self) -> Self {
        self.include_pci = true;
        self
    }

    /// Include USB device information in the query
    pub fn with_usb(mut self) -> Self {
        self.include_usb = true;
        self
    }

    /// Include virtualization information in the query
    pub fn with_virtualization(mut self) -> Self {
        self.include_virtualization = true;
        self
    }

    /// Include power management information (requires monitoring feature)
    #[cfg(feature = "monitoring")]
    pub fn with_power(mut self) -> Self {
        self.include_power = true;
        self
    }

    /// Include all available hardware information
    pub fn with_all(mut self) -> Self {
        self.include_cpu = true;
        self.include_gpu = true;
        self.include_memory = true;
        self.include_storage = true;
        self.include_network = true;
        self.include_battery = true;
        self.include_thermal = true;
        self.include_pci = true;
        self.include_usb = true;
        self.include_virtualization = true;
        
        #[cfg(feature = "monitoring")]
        {
            self.include_power = true;
        }
        
        self
    }

    /// Include basic system information (CPU, memory, storage)
    pub fn with_basic(mut self) -> Self {
        self.include_cpu = true;
        self.include_memory = true;
        self.include_storage = true;
        self
    }

    /// Include AI/ML relevant information (CPU, GPU, memory)
    pub fn with_ai_focused(mut self) -> Self {
        self.include_cpu = true;
        self.include_gpu = true;
        self.include_memory = true;
        self.include_thermal = true;
        self.include_virtualization = true;
        
        #[cfg(feature = "monitoring")]
        {
            self.include_power = true;
        }
        
        self
    }

    /// Include gaming-relevant information (CPU, GPU, memory, thermal)
    pub fn with_gaming_focused(mut self) -> Self {
        self.include_cpu = true;
        self.include_gpu = true;
        self.include_memory = true;
        self.include_thermal = true;
        self.include_storage = true;
        self
    }

    /// Include server/enterprise relevant information
    pub fn with_server_focused(mut self) -> Self {
        self.include_cpu = true;
        self.include_memory = true;
        self.include_storage = true;
        self.include_network = true;
        self.include_thermal = true;
        self.include_pci = true;
        self.include_virtualization = true;
        
        #[cfg(feature = "monitoring")]
        {
            self.include_power = true;
        }
        
        self
    }

    /// Filter GPUs by a custom predicate  
    pub fn filter_gpus<F>(self, _filter: F) -> Self 
    where
        F: Fn(&GPUInfo) -> bool + 'static,
    {
        // TODO: Implement filtering in a future version
        self
    }

    /// Filter storage devices by a custom predicate
    pub fn filter_storage<F>(self, _filter: F) -> Self 
    where
        F: Fn(&StorageInfo) -> bool + 'static,
    {
        // TODO: Implement filtering in a future version
        self
    }

    /// Filter network interfaces by a custom predicate
    pub fn filter_network<F>(self, _filter: F) -> Self 
    where
        F: Fn(&NetworkInfo) -> bool + 'static,
    {
        // TODO: Implement filtering in a future version
        self
    }

    /// Execute the query and return the requested hardware information
    pub fn query(self) -> Result<CustomHardwareInfo> {
        let start_time = std::time::Instant::now();
        let timestamp = std::time::SystemTime::now();
        
        // Track which components were requested
        let mut requested_components = Vec::new();
        
        // Get full hardware info first (we'll optimize this later)
        let full_hw = HardwareInfo::query()?;
        
        // Extract requested components
        let cpu = if self.include_cpu {
            requested_components.push("CPU".to_string());
            Some(full_hw.cpu().clone())
        } else {
            None
        };

        let gpus = if self.include_gpu {
            requested_components.push("GPU".to_string());
            full_hw.gpus().to_vec()
        } else {
            Vec::new()
        };

        let memory = if self.include_memory {
            requested_components.push("Memory".to_string());
            Some(full_hw.memory().clone())
        } else {
            None
        };

        let storage_devices = if self.include_storage {
            requested_components.push("Storage".to_string());
            full_hw.storage_devices().to_vec()
        } else {
            Vec::new()
        };

        let network_interfaces = if self.include_network {
            requested_components.push("Network".to_string());
            full_hw.network_interfaces().to_vec()
        } else {
            Vec::new()
        };

        let battery = if self.include_battery {
            requested_components.push("Battery".to_string());
            full_hw.battery().cloned()
        } else {
            None
        };

        let thermal = if self.include_thermal {
            requested_components.push("Thermal".to_string());
            Some(full_hw.thermal().clone())
        } else {
            None
        };

        let pci_devices = if self.include_pci {
            requested_components.push("PCI".to_string());
            full_hw.pci_devices().to_vec()
        } else {
            Vec::new()
        };

        let usb_devices = if self.include_usb {
            requested_components.push("USB".to_string());
            full_hw.usb_devices().to_vec()
        } else {
            Vec::new()
        };

        let virtualization = if self.include_virtualization {
            requested_components.push("Virtualization".to_string());
            Some(full_hw.virtualization().clone())
        } else {
            None
        };

        #[cfg(feature = "monitoring")]
        let power_profile = if self.include_power {
            requested_components.push("Power".to_string());
            full_hw.power_profile().cloned()
        } else {
            None
        };

        let query_time_ms = start_time.elapsed().as_millis() as u64;

        Ok(CustomHardwareInfo {
            cpu,
            gpus,
            memory,
            storage_devices,
            network_interfaces,
            battery,
            thermal,
            pci_devices,
            usb_devices,
            virtualization,
            
            #[cfg(feature = "monitoring")]
            power_profile,
            
            timestamp,
            query_time_ms,
            requested_components,
        })
    }

    /// Execute a quick query that only gathers essential information
    pub fn quick_query(self) -> Result<CustomHardwareInfo> {
        // For quick queries, we can optimize by avoiding expensive operations
        self.with_basic().query()
    }
}

impl CustomHardwareInfo {
    /// Check if a specific component was included in the query
    pub fn has_component(&self, component: &str) -> bool {
        self.requested_components.iter().any(|c| c.eq_ignore_ascii_case(component))
    }

    /// Get the total number of components that were queried
    pub fn component_count(&self) -> usize {
        self.requested_components.len()
    }

    /// Convert to a full HardwareInfo struct (filling missing components with defaults)
    pub fn to_full_hardware_info(&self) -> HardwareInfo {
        // This would require implementing Default for all the component types
        // For now, we'll return an error if conversion is attempted with missing components
        todo!("Implement conversion to full HardwareInfo")
    }

    /// Get a summary of what was queried
    pub fn query_summary(&self) -> String {
        format!(
            "Queried {} components in {}ms: {}",
            self.component_count(),
            self.query_time_ms,
            self.requested_components.join(", ")
        )
    }
}

// Convenience functions for common query patterns
impl HardwareQueryBuilder {
    /// Quick CPU and memory information
    pub fn cpu_and_memory() -> Result<CustomHardwareInfo> {
        Self::new().with_cpu().with_memory().query()
    }

    /// Quick GPU information for AI/gaming
    pub fn gpu_info() -> Result<CustomHardwareInfo> {
        Self::new().with_gpu().with_memory().query()
    }

    /// System health overview
    pub fn health_check() -> Result<CustomHardwareInfo> {
        Self::new()
            .with_cpu()
            .with_memory()
            .with_thermal()
            .with_battery()
            .query()
    }

    /// Performance overview for gaming/AI
    pub fn performance_check() -> Result<CustomHardwareInfo> {
        Self::new()
            .with_cpu()
            .with_gpu()
            .with_memory()
            .with_storage()
            .with_thermal()
            .query()
    }
}
