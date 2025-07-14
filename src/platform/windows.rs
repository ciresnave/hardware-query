/// Enhanced platform-specific hardware detection for Windows
use crate::{HardwareQueryError, Result};
use std::collections::HashMap;
use wmi::{COMLibrary, Variant, WMIConnection};

/// Windows-specific CPU information
#[derive(Debug, Clone)]
pub struct WindowsCPUInfo {
    pub name: String,
    pub vendor: String,
    pub description: String,
    pub family: Option<u32>,
    pub model: Option<u32>,
    pub stepping: Option<u32>,
    pub microcode: Option<String>,
    pub cores: u32,
    pub threads: u32,
    pub base_frequency: u32,
    pub max_frequency: u32,
    pub l1_cache: u32,
    pub l2_cache: u32,
    pub l3_cache: u32,
    pub voltage: Option<f32>,
    pub power_consumption: Option<f32>,
    pub temperature: Option<f32>,
}

impl WindowsCPUInfo {
    /// Query detailed CPU information from Windows WMI
    pub fn query() -> Result<Self> {
        let com_lib = COMLibrary::new()?;
        let wmi_con = WMIConnection::new(com_lib)?;

        // Query processor information
        let processor_query = "SELECT Name, Manufacturer, Description, Family, Model, Stepping, NumberOfCores, NumberOfLogicalProcessors, MaxClockSpeed, L1CacheSize, L2CacheSize, L3CacheSize, Voltage, CurrentVoltage FROM Win32_Processor";
        let processors: Vec<HashMap<String, Variant>> = wmi_con.raw_query(processor_query)?;

        if processors.is_empty() {
            return Err(HardwareQueryError::system_info_unavailable(
                "No processor information found",
            ));
        }

        let processor = &processors[0];

        Ok(Self {
            name: Self::get_string_value(processor, "Name").unwrap_or_default(),
            vendor: Self::get_string_value(processor, "Manufacturer").unwrap_or_default(),
            description: Self::get_string_value(processor, "Description").unwrap_or_default(),
            family: Self::get_u32_value(processor, "Family"),
            model: Self::get_u32_value(processor, "Model"),
            stepping: Self::get_u32_value(processor, "Stepping"),
            microcode: None, // Not available through WMI
            cores: Self::get_u32_value(processor, "NumberOfCores").unwrap_or(0),
            threads: Self::get_u32_value(processor, "NumberOfLogicalProcessors").unwrap_or(0),
            base_frequency: Self::get_u32_value(processor, "MaxClockSpeed").unwrap_or(0),
            max_frequency: Self::get_u32_value(processor, "MaxClockSpeed").unwrap_or(0),
            l1_cache: Self::get_u32_value(processor, "L1CacheSize").unwrap_or(0),
            l2_cache: Self::get_u32_value(processor, "L2CacheSize").unwrap_or(0),
            l3_cache: Self::get_u32_value(processor, "L3CacheSize").unwrap_or(0),
            voltage: Self::get_f32_value(processor, "CurrentVoltage"),
            power_consumption: None, // Requires additional queries
            temperature: None,       // Requires additional queries
        })
    }

    /// Get CPU temperature from thermal sensors
    pub fn get_temperature(&self) -> Result<Option<f32>> {
        let com_lib = COMLibrary::new()?;
        let wmi_con = WMIConnection::new(com_lib)?;

        // Try to get temperature from thermal zone
        let thermal_query = "SELECT Temperature FROM Win32_TemperatureProbe";
        let thermal_info: Vec<HashMap<String, Variant>> =
            wmi_con.raw_query(thermal_query).unwrap_or_default();

        for info in thermal_info {
            if let Some(temp) = Self::get_f32_value(&info, "Temperature") {
                // Convert from tenths of Kelvin to Celsius
                return Ok(Some((temp / 10.0) - 273.15));
            }
        }

        Ok(None)
    }

    /// Get CPU power consumption
    pub fn get_power_consumption(&self) -> Result<Option<f32>> {
        let com_lib = COMLibrary::new()?;
        let wmi_con = WMIConnection::new(com_lib)?;

        // Try to get power information from performance counters
        let power_query = "SELECT PowerConsumption FROM Win32_Processor";
        let power_info: Vec<HashMap<String, Variant>> =
            wmi_con.raw_query(power_query).unwrap_or_default();

        for info in power_info {
            if let Some(power) = Self::get_f32_value(&info, "PowerConsumption") {
                return Ok(Some(power));
            }
        }

        Ok(None)
    }

    /// Get CPU vulnerabilities from registry or system info
    pub fn get_vulnerabilities(&self) -> Result<Vec<String>> {
        let vulnerabilities = Vec::new();

        // Check for common CPU vulnerabilities
        // This would require reading from registry or system files
        // For now, return empty list

        Ok(vulnerabilities)
    }

    /// Helper function to extract string values from WMI results
    fn get_string_value(map: &HashMap<String, Variant>, key: &str) -> Option<String> {
        match map.get(key) {
            Some(Variant::String(s)) => Some(s.clone()),
            Some(Variant::Null) => None,
            _ => None,
        }
    }

    /// Helper function to extract u32 values from WMI results
    fn get_u32_value(map: &HashMap<String, Variant>, key: &str) -> Option<u32> {
        match map.get(key) {
            Some(Variant::UI4(val)) => Some(*val),
            Some(Variant::UI2(val)) => Some(*val as u32),
            Some(Variant::UI1(val)) => Some(*val as u32),
            Some(Variant::I4(val)) => Some(*val as u32),
            Some(Variant::I2(val)) => Some(*val as u32),
            Some(Variant::I1(val)) => Some(*val as u32),
            Some(Variant::Null) => None,
            _ => None,
        }
    }

    /// Helper function to extract f32 values from WMI results
    fn get_f32_value(map: &HashMap<String, Variant>, key: &str) -> Option<f32> {
        match map.get(key) {
            Some(Variant::R4(val)) => Some(*val),
            Some(Variant::R8(val)) => Some(*val as f32),
            Some(Variant::UI4(val)) => Some(*val as f32),
            Some(Variant::I4(val)) => Some(*val as f32),
            Some(Variant::Null) => None,
            _ => None,
        }
    }
}

/// Windows-specific GPU information
#[derive(Debug, Clone)]
pub struct WindowsGPUInfo {
    pub name: String,
    pub vendor: String,
    pub memory_mb: u64,
    pub driver_version: Option<String>,
    pub device_id: Option<String>,
    pub adapter_ram: Option<u64>,
    pub dedicated_memory: Option<u64>,
    pub shared_memory: Option<u64>,
    pub current_usage: Option<f32>,
}

impl WindowsGPUInfo {
    /// Query GPU information from Windows WMI
    pub fn query_all() -> Result<Vec<Self>> {
        let com_lib = COMLibrary::new()?;
        let wmi_con = WMIConnection::new(com_lib)?;

        let gpu_query = "SELECT Name, AdapterCompatibility, AdapterRAM, DriverVersion, DeviceID, DedicatedVideoMemory, SharedSystemMemory FROM Win32_VideoController";
        let gpus: Vec<HashMap<String, Variant>> = wmi_con.raw_query(gpu_query)?;

        let mut gpu_info = Vec::new();

        for gpu in gpus {
            let info = Self {
                name: Self::get_string_value(&gpu, "Name").unwrap_or_default(),
                vendor: Self::get_string_value(&gpu, "AdapterCompatibility").unwrap_or_default(),
                memory_mb: Self::get_u64_value(&gpu, "AdapterRAM").unwrap_or(0) / (1024 * 1024),
                driver_version: Self::get_string_value(&gpu, "DriverVersion"),
                device_id: Self::get_string_value(&gpu, "DeviceID"),
                adapter_ram: Self::get_u64_value(&gpu, "AdapterRAM"),
                dedicated_memory: Self::get_u64_value(&gpu, "DedicatedVideoMemory"),
                shared_memory: Self::get_u64_value(&gpu, "SharedSystemMemory"),
                current_usage: None, // Would require performance counters
            };
            gpu_info.push(info);
        }

        Ok(gpu_info)
    }

    /// Helper function to extract string values from WMI results
    fn get_string_value(map: &HashMap<String, Variant>, key: &str) -> Option<String> {
        match map.get(key) {
            Some(Variant::String(s)) => Some(s.clone()),
            Some(Variant::Null) => None,
            _ => None,
        }
    }

    /// Helper function to extract u64 values from WMI results
    fn get_u64_value(map: &HashMap<String, Variant>, key: &str) -> Option<u64> {
        match map.get(key) {
            Some(Variant::UI8(val)) => Some(*val),
            Some(Variant::UI4(val)) => Some(*val as u64),
            Some(Variant::UI2(val)) => Some(*val as u64),
            Some(Variant::UI1(val)) => Some(*val as u64),
            Some(Variant::I8(val)) => Some(*val as u64),
            Some(Variant::I4(val)) => Some(*val as u64),
            Some(Variant::I2(val)) => Some(*val as u64),
            Some(Variant::I1(val)) => Some(*val as u64),
            Some(Variant::Null) => None,
            _ => None,
        }
    }
}

/// Windows-specific memory information
#[derive(Debug, Clone)]
pub struct WindowsMemoryInfo {
    pub total_physical_mb: u64,
    pub available_physical_mb: u64,
    pub total_virtual_mb: u64,
    pub available_virtual_mb: u64,
    pub modules: Vec<WindowsMemoryModule>,
}

#[derive(Debug, Clone)]
pub struct WindowsMemoryModule {
    pub capacity_mb: u64,
    pub speed_mhz: u32,
    pub memory_type: String,
    pub manufacturer: Option<String>,
    pub part_number: Option<String>,
    pub bank_label: Option<String>,
    pub device_locator: Option<String>,
}

impl WindowsMemoryInfo {
    /// Query memory information from Windows WMI
    pub fn query() -> Result<Self> {
        let com_lib = COMLibrary::new()?;
        let wmi_con = WMIConnection::new(com_lib)?;

        // Query physical memory
        let memory_query = "SELECT TotalPhysicalMemory, AvailablePhysicalMemory, TotalVirtualMemorySize, AvailableVirtualMemory FROM Win32_OperatingSystem";
        let memory_info: Vec<HashMap<String, Variant>> = wmi_con.raw_query(memory_query)?;

        let mut total_physical_mb = 0;
        let mut available_physical_mb = 0;
        let mut total_virtual_mb = 0;
        let mut available_virtual_mb = 0;

        if let Some(info) = memory_info.first() {
            total_physical_mb = WindowsGPUInfo::get_u64_value(info, "TotalPhysicalMemory")
                .unwrap_or(0)
                / (1024 * 1024);
            available_physical_mb = WindowsGPUInfo::get_u64_value(info, "AvailablePhysicalMemory")
                .unwrap_or(0)
                / (1024 * 1024);
            total_virtual_mb = WindowsGPUInfo::get_u64_value(info, "TotalVirtualMemorySize")
                .unwrap_or(0)
                / (1024 * 1024);
            available_virtual_mb = WindowsGPUInfo::get_u64_value(info, "AvailableVirtualMemory")
                .unwrap_or(0)
                / (1024 * 1024);
        }

        // Query memory modules
        let modules_query = "SELECT Capacity, Speed, MemoryType, Manufacturer, PartNumber, BankLabel, DeviceLocator FROM Win32_PhysicalMemory";
        let modules: Vec<HashMap<String, Variant>> = wmi_con.raw_query(modules_query)?;

        let mut memory_modules = Vec::new();
        for module in modules {
            let module_info = WindowsMemoryModule {
                capacity_mb: WindowsGPUInfo::get_u64_value(&module, "Capacity").unwrap_or(0)
                    / (1024 * 1024),
                speed_mhz: WindowsGPUInfo::get_u64_value(&module, "Speed").unwrap_or(0) as u32,
                memory_type: Self::parse_memory_type(
                    WindowsGPUInfo::get_u64_value(&module, "MemoryType").unwrap_or(0),
                ),
                manufacturer: WindowsGPUInfo::get_string_value(&module, "Manufacturer"),
                part_number: WindowsGPUInfo::get_string_value(&module, "PartNumber"),
                bank_label: WindowsGPUInfo::get_string_value(&module, "BankLabel"),
                device_locator: WindowsGPUInfo::get_string_value(&module, "DeviceLocator"),
            };
            memory_modules.push(module_info);
        }

        Ok(Self {
            total_physical_mb,
            available_physical_mb,
            total_virtual_mb,
            available_virtual_mb,
            modules: memory_modules,
        })
    }

    /// Parse memory type from WMI integer value
    fn parse_memory_type(type_code: u64) -> String {
        match type_code {
            20 => "DDR".to_string(),
            21 => "DDR2".to_string(),
            22 => "DDR2 FB-DIMM".to_string(),
            24 => "DDR3".to_string(),
            26 => "DDR4".to_string(),
            34 => "DDR5".to_string(),
            _ => format!("Unknown ({type_code})"),
        }
    }
}
