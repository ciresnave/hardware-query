use crate::Result;
use serde::{Deserialize, Serialize};

#[cfg(feature = "nvidia")]
use nvml_wrapper::Nvml;

// ROCm detection will be done via system calls
#[cfg(target_os = "linux")]
use std::process::Command;

#[cfg(target_os = "windows")]
use wmi::{COMLibrary, WMIConnection};

/// GPU vendor information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GPUVendor {
    NVIDIA,
    AMD,
    Intel,
    Apple,
    ARM,
    Qualcomm,
    Unknown(String),
}

impl std::fmt::Display for GPUVendor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GPUVendor::NVIDIA => write!(f, "NVIDIA"),
            GPUVendor::AMD => write!(f, "AMD"),
            GPUVendor::Intel => write!(f, "Intel"),
            GPUVendor::Apple => write!(f, "Apple"),
            GPUVendor::ARM => write!(f, "ARM"),
            GPUVendor::Qualcomm => write!(f, "Qualcomm"),
            GPUVendor::Unknown(name) => write!(f, "{name}"),
        }
    }
}

/// GPU type classification
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GPUType {
    /// Consumer discrete GPU
    Discrete,
    /// Integrated GPU
    Integrated,
    /// Workstation GPU (Quadro, RadeonPro, etc.)
    Workstation,
    /// Datacenter GPU (Tesla, Instinct, etc.)
    Datacenter,
    /// Virtual GPU
    Virtual,
    /// Unknown type
    Unknown,
}

impl std::fmt::Display for GPUType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GPUType::Discrete => write!(f, "Discrete"),
            GPUType::Integrated => write!(f, "Integrated"),
            GPUType::Workstation => write!(f, "Workstation"),
            GPUType::Datacenter => write!(f, "Datacenter"),
            GPUType::Virtual => write!(f, "Virtual"),
            GPUType::Unknown => write!(f, "Unknown"),
        }
    }
}

/// GPU compute capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeCapabilities {
    /// CUDA support and compute capability
    pub cuda: Option<String>,
    /// ROCm support
    pub rocm: bool,
    /// DirectML support (Windows)
    pub directml: bool,
    /// OpenCL support
    pub opencl: bool,
    /// Vulkan support
    pub vulkan: bool,
    /// Metal support (macOS)
    pub metal: bool,
    /// Maximum compute units/cores
    pub compute_units: Option<u32>,
    /// Maximum workgroup size
    pub max_workgroup_size: Option<u32>,
}

/// GPU information and specifications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GPUInfo {
    /// GPU vendor
    pub vendor: GPUVendor,
    /// GPU model name
    pub model_name: String,
    /// GPU type (discrete, integrated, etc.)
    pub gpu_type: GPUType,
    /// GPU memory in MB
    pub memory_mb: u64,
    /// GPU memory type (GDDR6, HBM2, etc.)
    pub memory_type: Option<String>,
    /// GPU memory bandwidth in GB/s
    pub memory_bandwidth: Option<f32>,
    /// GPU base clock in MHz
    pub base_clock: Option<u32>,
    /// GPU boost clock in MHz
    pub boost_clock: Option<u32>,
    /// GPU memory clock in MHz
    pub memory_clock: Option<u32>,
    /// Number of shader units/cores
    pub shader_units: Option<u32>,
    /// Number of RT cores (NVIDIA)
    pub rt_cores: Option<u32>,
    /// Number of tensor cores (NVIDIA)
    pub tensor_cores: Option<u32>,
    /// Compute capabilities
    pub compute_capabilities: ComputeCapabilities,
    /// Current GPU usage percentage
    pub usage_percent: Option<f32>,
    /// Current GPU temperature in Celsius
    pub temperature: Option<f32>,
    /// Current power consumption in watts
    pub power_consumption: Option<f32>,
    /// Maximum power limit in watts
    pub power_limit: Option<f32>,
    /// Driver version
    pub driver_version: Option<String>,
    /// VBIOS version
    pub vbios_version: Option<String>,
    /// PCI device ID
    pub pci_device_id: Option<String>,
    /// PCI subsystem ID
    pub pci_subsystem_id: Option<String>,
}

impl GPUInfo {
    /// Query all GPU information from the system
    pub fn query_all() -> Result<Vec<Self>> {
        let mut gpus = Vec::new();

        // Always try WMI detection first to get all GPUs
        if let Ok(wmi_gpus) = Self::query_generic_gpus() {
            gpus.extend(wmi_gpus);
        }

        // Try to enhance with vendor-specific information
        if let Ok(nvidia_gpus) = Self::query_nvidia_gpus() {
            // Merge NVIDIA-specific details with WMI results
            for nvidia_gpu in nvidia_gpus {
                // Check if we already have this GPU from WMI
                if let Some(existing) = gpus.iter_mut().find(|g| 
                    g.vendor == GPUVendor::NVIDIA && 
                    g.model_name.contains("RTX") == nvidia_gpu.model_name.contains("RTX")
                ) {
                    // Update with more detailed NVIDIA information
                    existing.compute_capabilities.cuda = nvidia_gpu.compute_capabilities.cuda;
                    existing.usage_percent = nvidia_gpu.usage_percent;
                    existing.temperature = nvidia_gpu.temperature;
                    existing.power_consumption = nvidia_gpu.power_consumption;
                    existing.shader_units = nvidia_gpu.shader_units;
                    existing.rt_cores = nvidia_gpu.rt_cores;
                    existing.tensor_cores = nvidia_gpu.tensor_cores;
                } else {
                    // Add as new GPU if not found in WMI results
                    gpus.push(nvidia_gpu);
                }
            }
        }

        if let Ok(amd_gpus) = Self::query_amd_gpus() {
            // Similar merge logic for AMD GPUs
            for amd_gpu in amd_gpus {
                if !gpus.iter().any(|g| g.vendor == GPUVendor::AMD && g.model_name == amd_gpu.model_name) {
                    gpus.push(amd_gpu);
                }
            }
        }

        if let Ok(intel_gpus) = Self::query_intel_gpus() {
            // Similar merge logic for Intel GPUs
            for intel_gpu in intel_gpus {
                if !gpus.iter().any(|g| g.vendor == GPUVendor::Intel && g.model_name == intel_gpu.model_name) {
                    gpus.push(intel_gpu);
                }
            }
        }

        // If still no GPUs found, return a placeholder
        if gpus.is_empty() {
            gpus.push(Self::default_gpu());
        }

        Ok(gpus)
    }

    /// Get GPU vendor
    pub fn vendor(&self) -> &GPUVendor {
        &self.vendor
    }

    /// Get GPU model name
    pub fn model_name(&self) -> &str {
        &self.model_name
    }

    /// Get GPU type
    pub fn gpu_type(&self) -> &GPUType {
        &self.gpu_type
    }

    /// Get GPU memory in GB (rounded to 1 decimal place)
    pub fn memory_gb(&self) -> f64 {
        (self.memory_mb as f64 / 1024.0 * 10.0).round() / 10.0
    }

    /// Get GPU memory in MB
    pub fn memory_mb(&self) -> u64 {
        self.memory_mb
    }

    /// Check if GPU supports CUDA
    pub fn supports_cuda(&self) -> bool {
        self.compute_capabilities.cuda.is_some()
    }

    /// Get CUDA compute capability
    pub fn cuda_capability(&self) -> Option<&str> {
        self.compute_capabilities.cuda.as_deref()
    }

    /// Check if GPU supports ROCm
    pub fn supports_rocm(&self) -> bool {
        self.compute_capabilities.rocm
    }

    /// Check if GPU supports DirectML
    pub fn supports_directml(&self) -> bool {
        self.compute_capabilities.directml
    }

    /// Check if GPU supports OpenCL
    pub fn supports_opencl(&self) -> bool {
        self.compute_capabilities.opencl
    }

    /// Check if GPU supports Vulkan
    pub fn supports_vulkan(&self) -> bool {
        self.compute_capabilities.vulkan
    }

    /// Check if GPU supports Metal
    pub fn supports_metal(&self) -> bool {
        self.compute_capabilities.metal
    }

    /// Get current GPU usage percentage
    pub fn usage_percent(&self) -> Option<f32> {
        self.usage_percent
    }

    /// Get current GPU temperature
    pub fn temperature(&self) -> Option<f32> {
        self.temperature
    }

    /// Create a default/fallback GPU for systems where no GPUs are detected
    fn default_gpu() -> Self {
        Self {
            vendor: GPUVendor::Unknown("Generic".to_string()),
            model_name: "Unknown GPU".to_string(),
            gpu_type: GPUType::Unknown,
            memory_mb: 1024, // 1GB default
            memory_type: None,
            memory_bandwidth: None,
            base_clock: None,
            boost_clock: None,
            memory_clock: None,
            shader_units: None,
            rt_cores: None,
            tensor_cores: None,
            compute_capabilities: ComputeCapabilities {
                cuda: None,
                rocm: false,
                directml: cfg!(target_os = "windows"),
                opencl: false,
                vulkan: false,
                metal: cfg!(target_os = "macos"),
                compute_units: None,
                max_workgroup_size: None,
            },
            usage_percent: None,
            temperature: None,
            power_consumption: None,
            power_limit: None,
            driver_version: None,
            vbios_version: None,
            pci_device_id: None,
            pci_subsystem_id: None,
        }
    }

    fn query_nvidia_gpus() -> Result<Vec<Self>> {
        #[cfg(feature = "nvidia")]
        {
            let nvml = match Nvml::init() {
                Ok(nvml) => nvml,
                Err(_) => return Ok(vec![]),
            };

            let mut gpus = Vec::new();
            let device_count = nvml.device_count().unwrap_or(0);

            for i in 0..device_count {
                if let Ok(device) = nvml.device_by_index(i) {
                    let name = device.name().unwrap_or_default();
                    let memory_info = device.memory_info().ok();
                    let cuda_capability = device.cuda_compute_capability().ok();
                    let driver_version = nvml.sys_driver_version().unwrap_or_default();

                    let gpu = Self {
                        vendor: GPUVendor::NVIDIA,
                        model_name: name,
                        gpu_type: GPUType::Discrete,
                        memory_mb: memory_info.map(|m| m.total / 1024 / 1024).unwrap_or(0),
                        memory_type: Some("GDDR6".to_string()),
                        memory_bandwidth: None,
                        base_clock: None,
                        boost_clock: None,
                        memory_clock: None,
                        shader_units: None,
                        rt_cores: None,
                        tensor_cores: None,
                        compute_capabilities: ComputeCapabilities {
                            cuda: cuda_capability.map(|c| format!("{}.{}", c.major, c.minor)),
                            rocm: false,
                            directml: cfg!(target_os = "windows"),
                            opencl: true,
                            vulkan: true,
                            metal: cfg!(target_os = "macos"),
                            compute_units: None,
                            max_workgroup_size: None,
                        },
                        usage_percent: device.utilization_rates().ok().map(|u| u.gpu as f32),
                        temperature: device
                            .temperature(
                                nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu,
                            )
                            .ok()
                            .map(|t| t as f32),
                        power_consumption: device.power_usage().ok().map(|p| p as f32 / 1000.0),
                        power_limit: device
                            .power_management_limit_default()
                            .ok()
                            .map(|p| p as f32 / 1000.0),
                        driver_version: Some(driver_version),
                        vbios_version: device.vbios_version().ok(),
                        pci_device_id: None,
                        pci_subsystem_id: None,
                    };

                    gpus.push(gpu);
                }
            }

            Ok(gpus)
        }
        #[cfg(not(feature = "nvidia"))]
        {
            Ok(vec![])
        }
    }

    fn query_amd_gpus() -> Result<Vec<Self>> {
        #[cfg(feature = "amd")]
        {
            // ROCm/AMD GPU detection would require additional dependencies
            // For now, return empty vector as AMD feature is not implemented
            Ok(vec![])
        }
        #[cfg(not(feature = "amd"))]
        {
            Ok(vec![])
        }
    }

    fn query_intel_gpus() -> Result<Vec<Self>> {
        #[cfg(target_os = "windows")]
        {
            // Use WMI to query Intel GPUs
            match WMIConnection::new(COMLibrary::new()?) {
                Ok(wmi_con) => {
                    let results: Vec<std::collections::HashMap<String, wmi::Variant>> = wmi_con
                        .raw_query("SELECT Name, AdapterRAM FROM Win32_VideoController WHERE Name LIKE '%Intel%'")
                        .unwrap_or_default();

                    let mut gpus = Vec::new();
                    for result in results {
                        if let (Some(wmi::Variant::String(name)), Some(wmi::Variant::UI4(ram))) =
                            (result.get("Name"), result.get("AdapterRAM"))
                        {
                            let gpu = Self {
                                vendor: GPUVendor::Intel,
                                model_name: name.clone(),
                                gpu_type: GPUType::Integrated,
                                memory_mb: *ram as u64 / 1024 / 1024,
                                memory_type: Some("System".to_string()),
                                memory_bandwidth: None,
                                base_clock: None,
                                boost_clock: None,
                                memory_clock: None,
                                shader_units: None,
                                rt_cores: None,
                                tensor_cores: None,
                                compute_capabilities: ComputeCapabilities {
                                    cuda: None,
                                    rocm: false,
                                    directml: true,
                                    opencl: true,
                                    vulkan: true,
                                    metal: false,
                                    compute_units: None,
                                    max_workgroup_size: None,
                                },
                                usage_percent: None,
                                temperature: None,
                                power_consumption: None,
                                power_limit: None,
                                driver_version: None,
                                vbios_version: None,
                                pci_device_id: None,
                                pci_subsystem_id: None,
                            };

                            gpus.push(gpu);
                        }
                    }

                    Ok(gpus)
                }
                Err(_) => Ok(vec![]),
            }
        }
        #[cfg(not(target_os = "windows"))]
        {
            Ok(vec![])
        }
    }

    fn query_generic_gpus() -> Result<Vec<Self>> {
        // Generic GPU detection using WMI on Windows
        #[cfg(target_os = "windows")]
        {
            use std::collections::HashMap;
            use wmi::{COMLibrary, WMIConnection, Variant};

            let com_con = COMLibrary::new()?;
            let wmi_con = WMIConnection::new(com_con)?;

            let results: Vec<HashMap<String, Variant>> = wmi_con
                .raw_query("SELECT * FROM Win32_VideoController WHERE PNPDeviceID IS NOT NULL")?;

            let mut gpus = Vec::new();

            for gpu in results {
                let name = gpu.get("Name")
                    .and_then(|v| match v {
                        Variant::String(s) => Some(s.clone()),
                        _ => None,
                    })
                    .unwrap_or_else(|| "Unknown GPU".to_string());

                let adapter_ram = gpu.get("AdapterRAM")
                    .and_then(|v| match v {
                        Variant::UI4(val) => Some(*val as u64),
                        Variant::UI8(val) => Some(*val),
                        _ => None,
                    })
                    .unwrap_or(0);

                let device_id = gpu.get("PNPDeviceID")
                    .and_then(|v| match v {
                        Variant::String(s) => Some(s.clone()),
                        _ => None,
                    })
                    .unwrap_or_else(|| "".to_string());

                let driver_version = gpu.get("DriverVersion")
                    .and_then(|v| match v {
                        Variant::String(s) => Some(s.clone()),
                        _ => None,
                    });

                // Determine vendor from name or device ID
                let vendor = if name.to_lowercase().contains("nvidia") || device_id.contains("VEN_10DE") {
                    GPUVendor::NVIDIA
                } else if name.to_lowercase().contains("amd") || name.to_lowercase().contains("radeon") || device_id.contains("VEN_1002") {
                    GPUVendor::AMD
                } else if name.to_lowercase().contains("intel") || device_id.contains("VEN_8086") {
                    GPUVendor::Intel
                } else {
                    GPUVendor::Unknown("Generic".to_string())
                };

                // Determine GPU type using comprehensive classification
                let gpu_type = Self::classify_gpu_type(&name, &vendor, adapter_ram);

                // Convert memory from bytes to MB
                let memory_mb = if adapter_ram > 0 {
                    adapter_ram / (1024 * 1024)
                } else {
                    // Fallback estimates based on GPU type and vendor
                    match (&vendor, &gpu_type) {
                        (GPUVendor::NVIDIA, GPUType::Datacenter) => 32768,    // 32GB for datacenter (A100, H100)
                        (GPUVendor::NVIDIA, GPUType::Workstation) => 16384,   // 16GB for workstation (RTX A6000)
                        (GPUVendor::NVIDIA, GPUType::Discrete) => 8192,       // 8GB for consumer RTX
                        (GPUVendor::AMD, GPUType::Datacenter) => 32768,       // 32GB for Instinct
                        (GPUVendor::AMD, GPUType::Workstation) => 16384,      // 16GB for Radeon Pro
                        (GPUVendor::AMD, GPUType::Discrete) => 8192,          // 8GB for discrete AMD
                        (_, GPUType::Integrated) => 512,                      // 512MB for integrated
                        _ => 4096,                                            // Default 4GB
                    }
                };

                // Set compute capabilities based on vendor
                let compute_capabilities = ComputeCapabilities {
                    cuda: if vendor == GPUVendor::NVIDIA { Some("Unknown".to_string()) } else { None },
                    rocm: vendor == GPUVendor::AMD && gpu_type == GPUType::Discrete,
                    directml: true, // DirectML is available on Windows for most modern GPUs
                    opencl: true,   // Most modern GPUs support OpenCL
                    vulkan: true,   // Most modern GPUs support Vulkan
                    metal: false,   // Metal is macOS only
                    compute_units: None,
                    max_workgroup_size: None,
                };

                gpus.push(Self {
                    vendor,
                    model_name: name,
                    gpu_type,
                    memory_mb,
                    memory_type: None,
                    memory_bandwidth: None,
                    base_clock: None,
                    boost_clock: None,
                    memory_clock: None,
                    shader_units: None,
                    rt_cores: None,
                    tensor_cores: None,
                    compute_capabilities,
                    usage_percent: None,
                    temperature: None,
                    power_consumption: None,
                    power_limit: None,
                    driver_version,
                    vbios_version: None,
                    pci_device_id: Some(device_id),
                    pci_subsystem_id: None,
                });
            }

            Ok(gpus)
        }
        #[cfg(not(target_os = "windows"))]
        {
            // For non-Windows platforms, use system-specific detection
            Ok(vec![])
        }
    }

    /// Classify GPU type based on model name and characteristics
    fn classify_gpu_type(name: &str, vendor: &GPUVendor, adapter_ram: u64) -> GPUType {
        let name_lower = name.to_lowercase();
        
        // Check for datacenter GPUs first
        if Self::is_datacenter_gpu(&name_lower, vendor) {
            return GPUType::Datacenter;
        }
        
        // Check for workstation GPUs
        if Self::is_workstation_gpu(&name_lower, vendor) {
            return GPUType::Workstation;
        }
        
        // Check for integrated GPUs
        if Self::is_integrated_gpu(&name_lower, vendor, adapter_ram) {
            return GPUType::Integrated;
        }
        
        // Default to discrete for remaining GPUs
        GPUType::Discrete
    }
    
    /// Check if GPU is a datacenter model
    fn is_datacenter_gpu(name: &str, vendor: &GPUVendor) -> bool {
        match vendor {
            GPUVendor::NVIDIA => {
                name.contains("tesla") ||
                name.contains("a100") ||
                name.contains("h100") ||
                name.contains("h200") ||
                name.contains("v100") ||
                name.contains("p100") ||
                name.contains("k80") ||
                name.contains("k40") ||
                name.contains("l40") ||
                name.contains("l4") ||
                name.contains("data center") ||
                name.contains("datacenter") ||
                name.contains("dgx") ||
                name.contains("hgx")
            },
            GPUVendor::AMD => {
                name.contains("instinct") ||
                name.contains("mi50") ||
                name.contains("mi100") ||
                name.contains("mi200") ||
                name.contains("mi250") ||
                name.contains("mi300") ||
                name.contains("cdna") ||
                name.contains("datacenter") ||
                name.contains("server")
            },
            GPUVendor::Intel => {
                name.contains("ponte vecchio") ||
                name.contains("data center") ||
                name.contains("max") && (name.contains("1100") || name.contains("1550"))
            },
            _ => false,
        }
    }
    
    /// Check if GPU is a workstation model
    fn is_workstation_gpu(name: &str, vendor: &GPUVendor) -> bool {
        match vendor {
            GPUVendor::NVIDIA => {
                name.contains("quadro") ||
                name.contains("rtx a") ||        // RTX A series (A4000, A5000, A6000)
                name.contains("rtx 4000") ||     // Quadro RTX 4000
                name.contains("rtx 5000") ||     // Quadro RTX 5000, etc.
                name.contains("rtx 6000") ||
                name.contains("rtx 8000") ||
                name.contains("titan") ||        // Titan series
                name.contains("nvs") ||          // NVS series
                name.contains("t1000") ||        // T-series workstation
                name.contains("t400") ||
                name.contains("t600") ||
                (name.contains("professional") && !name.contains("geforce"))
            },
            GPUVendor::AMD => {
                name.contains("radeon pro") ||
                name.contains("firepro") ||
                name.contains("wx ") ||          // WX series
                name.contains("w6") ||           // W6000 series  
                name.contains("w7") ||           // W7000 series
                name.contains("workstation") ||
                name.contains("professional")
            },
            GPUVendor::Intel => {
                name.contains("pro") ||
                name.contains("workstation") ||
                name.contains("professional")
            },
            _ => false,
        }
    }
    
    /// Check if GPU is integrated
    fn is_integrated_gpu(name: &str, vendor: &GPUVendor, adapter_ram: u64) -> bool {
        // Standard integrated GPU indicators
        let integrated_keywords = name.contains("integrated") ||
                                 name.contains("uhd") ||
                                 name.contains("iris") ||
                                 name.contains("vega") && name.contains("graphics") ||
                                 name.contains("radeon graphics") ||
                                 name.contains("apu") ||
                                 name.contains("mobile") && !name.contains("rtx") ||
                                 name.contains("embedded");
        
        // Memory-based heuristic (integrated GPUs typically have < 2GB dedicated VRAM)
        let low_memory = vendor == &GPUVendor::AMD && adapter_ram < 2_000_000_000;
        
        integrated_keywords || low_memory
    }
}
