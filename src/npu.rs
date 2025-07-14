use crate::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[cfg(target_os = "windows")]
use wmi::{COMLibrary, WMIConnection};

#[cfg(target_os = "linux")]
use std::process::Command;

/// NPU vendor information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NPUVendor {
    Intel,
    Qualcomm,
    Apple,
    AMD,
    Google,
    MediaTek,
    Samsung,
    Hailo,
    Kneron,
    Unknown(String),
}

impl std::fmt::Display for NPUVendor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NPUVendor::Intel => write!(f, "Intel"),
            NPUVendor::Qualcomm => write!(f, "Qualcomm"),
            NPUVendor::Apple => write!(f, "Apple"),
            NPUVendor::AMD => write!(f, "AMD"),
            NPUVendor::Google => write!(f, "Google"),
            NPUVendor::MediaTek => write!(f, "MediaTek"),
            NPUVendor::Samsung => write!(f, "Samsung"),
            NPUVendor::Hailo => write!(f, "Hailo"),
            NPUVendor::Kneron => write!(f, "Kneron"),
            NPUVendor::Unknown(name) => write!(f, "{name}"),
        }
    }
}

/// NPU type classification
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NPUType {
    Integrated,     // Built into CPU/SoC
    Discrete,       // Separate accelerator card
    USB,           // USB-connected device
    PCIe,          // PCIe card
    M2,            // M.2 module
    Unknown,
}

/// NPU architecture information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NPUArchitecture {
    IntelMovidius,
    IntelGNA,
    IntelXDNA,
    QualcommHexagon,
    AppleNeuralEngine,
    GoogleTPU,
    AMDRyzenAI,
    MediaTekAPU,
    SamsungNPU,
    HailoNPU,
    KneronKL,
    Unknown(String),
}

/// NPU information structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NPUInfo {
    /// NPU vendor
    pub vendor: NPUVendor,
    
    /// NPU model name
    pub model_name: String,
    
    /// NPU type
    pub npu_type: NPUType,
    
    /// NPU architecture
    pub architecture: NPUArchitecture,
    
    /// Performance in TOPS (Tera Operations Per Second)
    pub tops_performance: Option<f32>,
    
    /// Memory size in MB (if applicable)
    pub memory_mb: Option<u64>,
    
    /// Driver version
    pub driver_version: Option<String>,
    
    /// Firmware version
    pub firmware_version: Option<String>,
    
    /// PCI device ID (if applicable)
    pub pci_device_id: Option<String>,
    
    /// USB device ID (if applicable)
    pub usb_device_id: Option<String>,
    
    /// Supported frameworks
    pub supported_frameworks: Vec<String>,
    
    /// Power consumption in watts
    pub power_consumption: Option<f32>,
    
    /// Operating temperature in Celsius
    pub temperature: Option<f32>,
    
    /// Clock frequency in MHz
    pub clock_frequency: Option<u32>,
    
    /// Additional capabilities
    pub capabilities: HashMap<String, String>,
}

impl NPUInfo {
    /// Query all available NPUs in the system
    pub fn query_all() -> Result<Vec<NPUInfo>> {
        let mut npus = Vec::new();
        
        // Detect various NPU types
        npus.extend(Self::detect_intel_npus()?);
        npus.extend(Self::detect_qualcomm_npus()?);
        npus.extend(Self::detect_apple_neural_engine()?);
        npus.extend(Self::detect_amd_npus()?);
        npus.extend(Self::detect_google_tpus()?);
        npus.extend(Self::detect_usb_npus()?);
        npus.extend(Self::detect_pcie_npus()?);
        
        Ok(npus)
    }
    
    /// Detect Intel NPUs (Movidius, GNA, XDNA)
    fn detect_intel_npus() -> Result<Vec<NPUInfo>> {
        let mut npus = Vec::new();
        
        // Intel Neural Compute Stick detection
        npus.extend(Self::detect_intel_ncs()?);
        
        // Intel GNA (Gaussian Neural Accelerator)
        npus.extend(Self::detect_intel_gna()?);
        
        // Intel XDNA (Meteor Lake and newer)
        npus.extend(Self::detect_intel_xdna()?);
        
        Ok(npus)
    }
    
    fn detect_intel_ncs() -> Result<Vec<NPUInfo>> {
        let mut npus = Vec::new();
        
        #[cfg(target_os = "linux")]
        {
            // Check for Movidius devices via lsusb
            if let Ok(output) = Command::new("lsusb").output() {
                let output_str = String::from_utf8_lossy(&output.stdout);
                for line in output_str.lines() {
                    if line.contains("03e7") { // Intel Movidius VID
                        if line.contains("2485") { // Neural Compute Stick 2
                            npus.push(NPUInfo {
                                vendor: NPUVendor::Intel,
                                model_name: "Intel Neural Compute Stick 2".to_string(),
                                npu_type: NPUType::USB,
                                architecture: NPUArchitecture::IntelMovidius,
                                tops_performance: Some(4.0), // ~4 TOPS
                                memory_mb: None,
                                driver_version: Self::get_intel_ncs_driver_version(),
                                firmware_version: None,
                                pci_device_id: None,
                                usb_device_id: Some("03e7:2485".to_string()),
                                supported_frameworks: vec![
                                    "OpenVINO".to_string(),
                                    "ONNX Runtime".to_string(),
                                ],
                                power_consumption: Some(1.0), // ~1W
                                temperature: None,
                                clock_frequency: None,
                                capabilities: HashMap::new(),
                            });
                        } else if line.contains("f63b") { // Neural Compute Stick 1
                            npus.push(NPUInfo {
                                vendor: NPUVendor::Intel,
                                model_name: "Intel Neural Compute Stick".to_string(),
                                npu_type: NPUType::USB,
                                architecture: NPUArchitecture::IntelMovidius,
                                tops_performance: Some(0.1), // ~0.1 TOPS
                                memory_mb: None,
                                driver_version: Self::get_intel_ncs_driver_version(),
                                firmware_version: None,
                                pci_device_id: None,
                                usb_device_id: Some("03e7:f63b".to_string()),
                                supported_frameworks: vec!["OpenVINO".to_string()],
                                power_consumption: Some(1.0),
                                temperature: None,
                                clock_frequency: None,
                                capabilities: HashMap::new(),
                            });
                        }
                    }
                }
            }
        }
        
        #[cfg(target_os = "windows")]
        {
            // Windows detection via WMI and device manager
            if let Ok(com_con) = COMLibrary::new() {
                if let Ok(wmi_con) = WMIConnection::new(com_con) {
                    let query = "SELECT * FROM Win32_USBHub WHERE DeviceID LIKE '%VID_03E7%'";
                    if let Ok(results) = wmi_con.raw_query(query) {
                        let results: Vec<HashMap<String, wmi::Variant>> = results;
                        for device in results {
                            if let Some(device_id) = device.get("DeviceID") {
                                let device_id_str = format!("{device_id:?}");
                                if device_id_str.contains("PID_2485") {
                                    npus.push(NPUInfo {
                                        vendor: NPUVendor::Intel,
                                        model_name: "Intel Neural Compute Stick 2".to_string(),
                                        npu_type: NPUType::USB,
                                        architecture: NPUArchitecture::IntelMovidius,
                                        tops_performance: Some(4.0),
                                        memory_mb: None,
                                        driver_version: Self::get_intel_ncs_driver_version(),
                                        firmware_version: None,
                                        pci_device_id: None,
                                        usb_device_id: Some("03e7:2485".to_string()),
                                        supported_frameworks: vec![
                                            "OpenVINO".to_string(),
                                            "ONNX Runtime".to_string(),
                                        ],
                                        power_consumption: Some(1.0),
                                        temperature: None,
                                        clock_frequency: None,
                                        capabilities: HashMap::new(),
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
        
        Ok(npus)
    }
    
    fn detect_intel_gna() -> Result<Vec<NPUInfo>> {
        let mut npus = Vec::new();
        
        // Intel GNA detection - typically integrated in Tiger Lake and newer
        #[cfg(target_os = "linux")]
        {
            // Check for GNA device in /sys/class
            if std::path::Path::new("/sys/class/intel_gna").exists() {
                npus.push(NPUInfo {
                    vendor: NPUVendor::Intel,
                    model_name: "Intel Gaussian Neural Accelerator".to_string(),
                    npu_type: NPUType::Integrated,
                    architecture: NPUArchitecture::IntelGNA,
                    tops_performance: Some(1.0), // ~1 TOPS for inference
                    memory_mb: None, // Uses system memory
                    driver_version: Self::get_intel_gna_driver_version(),
                    firmware_version: None,
                    pci_device_id: None,
                    usb_device_id: None,
                    supported_frameworks: vec![
                        "OpenVINO".to_string(),
                        "Intel GNA Library".to_string(),
                    ],
                    power_consumption: Some(0.5), // Very low power
                    temperature: None,
                    clock_frequency: Some(400), // ~400MHz
                    capabilities: HashMap::from([
                        ("keyword_spotting".to_string(), "true".to_string()),
                        ("noise_reduction".to_string(), "true".to_string()),
                    ]),
                });
            }
        }
        
        #[cfg(target_os = "windows")]
        {
            // Windows GNA detection via device manager
            if let Ok(com_con) = COMLibrary::new() {
                if let Ok(wmi_con) = WMIConnection::new(com_con) {
                    let query = "SELECT * FROM Win32_PnPEntity WHERE Description LIKE '%GNA%' OR Name LIKE '%Gaussian%'";
                    if let Ok(results) = wmi_con.raw_query(query) {
                        let results: Vec<HashMap<String, wmi::Variant>> = results;
                        if !results.is_empty() {
                            npus.push(NPUInfo {
                                vendor: NPUVendor::Intel,
                                model_name: "Intel Gaussian Neural Accelerator".to_string(),
                                npu_type: NPUType::Integrated,
                                architecture: NPUArchitecture::IntelGNA,
                                tops_performance: Some(1.0),
                                memory_mb: None,
                                driver_version: Self::get_intel_gna_driver_version(),
                                firmware_version: None,
                                pci_device_id: None,
                                usb_device_id: None,
                                supported_frameworks: vec![
                                    "OpenVINO".to_string(),
                                    "Intel GNA Library".to_string(),
                                ],
                                power_consumption: Some(0.5),
                                temperature: None,
                                clock_frequency: Some(400),
                                capabilities: HashMap::from([
                                    ("keyword_spotting".to_string(), "true".to_string()),
                                    ("noise_reduction".to_string(), "true".to_string()),
                                ]),
                            });
                        }
                    }
                }
            }
        }
        
        Ok(npus)
    }
    
    fn detect_intel_xdna() -> Result<Vec<NPUInfo>> {
        let mut npus = Vec::new();
        
        // Intel XDNA (Meteor Lake and newer integrated NPU)
        #[cfg(target_os = "windows")]
        {
            if let Ok(com_con) = COMLibrary::new() {
                if let Ok(wmi_con) = WMIConnection::new(com_con) {
                    let query = "SELECT * FROM Win32_PnPEntity WHERE Description LIKE '%NPU%' OR Name LIKE '%Neural%'";
                    if let Ok(results) = wmi_con.raw_query(query) {
                        let results: Vec<HashMap<String, wmi::Variant>> = results;
                        for device in results {
                            if let Some(name) = device.get("Name") {
                                let name_str = format!("{name:?}");
                                if name_str.contains("Intel") && (name_str.contains("NPU") || name_str.contains("Neural")) {
                                    npus.push(NPUInfo {
                                        vendor: NPUVendor::Intel,
                                        model_name: "Intel XDNA NPU".to_string(),
                                        npu_type: NPUType::Integrated,
                                        architecture: NPUArchitecture::IntelXDNA,
                                        tops_performance: Some(11.5), // Meteor Lake NPU
                                        memory_mb: None,
                                        driver_version: Self::get_intel_npu_driver_version(),
                                        firmware_version: None,
                                        pci_device_id: None,
                                        usb_device_id: None,
                                        supported_frameworks: vec![
                                            "OpenVINO".to_string(),
                                            "ONNX Runtime".to_string(),
                                            "DirectML".to_string(),
                                        ],
                                        power_consumption: Some(2.0),
                                        temperature: None,
                                        clock_frequency: Some(1400), // ~1.4GHz
                                        capabilities: HashMap::from([
                                            ("int8".to_string(), "true".to_string()),
                                            ("fp16".to_string(), "true".to_string()),
                                            ("dynamic_shapes".to_string(), "true".to_string()),
                                        ]),
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
        
        Ok(npus)
    }
    
    // Helper functions for driver version detection
    fn get_intel_ncs_driver_version() -> Option<String> {
        // Try to get OpenVINO version as proxy for NCS driver
        #[cfg(target_os = "linux")]
        {
            if let Ok(output) = Command::new("python3")
                .args(["-c", "import openvino; print(openvino.__version__)"])
                .output()
            {
                if output.status.success() {
                    return Some(String::from_utf8_lossy(&output.stdout).trim().to_string());
                }
            }
        }
        None
    }
    
    fn get_intel_gna_driver_version() -> Option<String> {
        // Check for Intel GNA driver version
        #[cfg(target_os = "linux")]
        {
            if let Ok(contents) = std::fs::read_to_string("/sys/module/intel_gna/version") {
                return Some(contents.trim().to_string());
            }
        }
        None
    }
    
    fn get_intel_npu_driver_version() -> Option<String> {
        // Windows: Check registry or device manager for NPU driver version
        None // Placeholder - would need Windows registry access
    }
    
    // Placeholder implementations for other vendors
    fn detect_qualcomm_npus() -> Result<Vec<NPUInfo>> {
        // Note: Qualcomm Hexagon NPU detection requires proprietary Qualcomm SDK
        Ok(Vec::new())
    }
    
    fn detect_apple_neural_engine() -> Result<Vec<NPUInfo>> {
        #[cfg(target_os = "macos")]
        {
            // Detect Apple Neural Engine on M-series chips
            if let Ok(output) = Command::new("sysctl")
                .args(["-n", "machdep.cpu.brand_string"])
                .output()
            {
                let cpu_brand = String::from_utf8_lossy(&output.stdout);
                if cpu_brand.contains("Apple M") {
                    let mut npus = Vec::new();
                    
                    // Determine performance based on chip
                    let (tops, model_name) = if cpu_brand.contains("M1") {
                        (15.8, "Apple Neural Engine (M1)")
                    } else if cpu_brand.contains("M2") {
                        (15.8, "Apple Neural Engine (M2)")
                    } else if cpu_brand.contains("M3") {
                        (18.0, "Apple Neural Engine (M3)")
                    } else {
                        (15.8, "Apple Neural Engine")
                    };
                    
                    npus.push(NPUInfo {
                        vendor: NPUVendor::Apple,
                        model_name: model_name.to_string(),
                        npu_type: NPUType::Integrated,
                        architecture: NPUArchitecture::AppleNeuralEngine,
                        tops_performance: Some(tops),
                        memory_mb: None, // Unified memory architecture
                        driver_version: None,
                        firmware_version: None,
                        pci_device_id: None,
                        usb_device_id: None,
                        supported_frameworks: vec![
                            "Core ML".to_string(),
                            "ONNX Runtime".to_string(),
                            "TensorFlow Lite".to_string(),
                        ],
                        power_consumption: Some(1.0),
                        temperature: None,
                        clock_frequency: None,
                        capabilities: HashMap::from([
                            ("16_core".to_string(), "true".to_string()),
                            ("matrix_operations".to_string(), "true".to_string()),
                            ("convolution".to_string(), "true".to_string()),
                        ]),
                    });
                    
                    return Ok(npus);
                }
            }
        }
        Ok(Vec::new())
    }
    
    fn detect_amd_npus() -> Result<Vec<NPUInfo>> {
        // Note: AMD Ryzen AI NPU detection requires AMD ROCm drivers and SDK
        Ok(Vec::new())
    }
    
    fn detect_google_tpus() -> Result<Vec<NPUInfo>> {
        // Note: Google Coral Edge TPU detection requires libcoral and Edge TPU runtime
        Ok(Vec::new())
    }
    
    fn detect_usb_npus() -> Result<Vec<NPUInfo>> {
        // Note: Generic USB NPU detection requires device-specific drivers
        Ok(Vec::new())
    }
    
    fn detect_pcie_npus() -> Result<Vec<NPUInfo>> {
        // Note: PCIe NPU detection requires device-specific identification methods
        Ok(Vec::new())
    }
    
    /// Get NPU vendor
    pub fn vendor(&self) -> &NPUVendor {
        &self.vendor
    }
    
    /// Get NPU model name
    pub fn model_name(&self) -> &str {
        &self.model_name
    }
    
    /// Get NPU type
    pub fn npu_type(&self) -> &NPUType {
        &self.npu_type
    }
    
    /// Get performance in TOPS
    pub fn tops_performance(&self) -> Option<f32> {
        self.tops_performance
    }
    
    /// Check if NPU supports a specific framework
    pub fn supports_framework(&self, framework: &str) -> bool {
        self.supported_frameworks.iter()
            .any(|f| f.to_lowercase().contains(&framework.to_lowercase()))
    }
}
