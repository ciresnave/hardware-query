use crate::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[cfg(target_os = "linux")]
use std::process::Command;

/// TPU vendor information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TPUVendor {
    Google,
    Intel,
    Groq,
    Cerebras,
    Unknown(String),
}

impl std::fmt::Display for TPUVendor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TPUVendor::Google => write!(f, "Google"),
            TPUVendor::Intel => write!(f, "Intel"),
            TPUVendor::Groq => write!(f, "Groq"),
            TPUVendor::Cerebras => write!(f, "Cerebras"),
            TPUVendor::Unknown(name) => write!(f, "{name}"),
        }
    }
}

/// TPU generation and architecture
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TPUArchitecture {
    GoogleTPUv2,
    GoogleTPUv3,
    GoogleTPUv4,
    GoogleTPUv5,
    GoogleCoralEdge,
    IntelHabanaGaudi,
    IntelHabanaGaudi2,
    IntelHabanaGoya,
    GroqLPU,
    CerebrasWSE,
    Unknown(String),
}

/// TPU connection type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TPUConnectionType {
    CloudTPU,    // Google Cloud TPU
    PCIe,        // PCIe card
    USB,         // USB device (Edge TPU)
    M2,          // M.2 module
    Network,     // Network-attached
    Unknown,
}

/// TPU information structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TPUInfo {
    /// TPU vendor
    pub vendor: TPUVendor,
    
    /// TPU model name
    pub model_name: String,
    
    /// TPU architecture/generation
    pub architecture: TPUArchitecture,
    
    /// Connection type
    pub connection_type: TPUConnectionType,
    
    /// Performance in TOPS (Tera Operations Per Second)
    pub tops_performance: Option<f32>,
    
    /// Memory size in GB
    pub memory_gb: Option<f32>,
    
    /// Memory bandwidth in GB/s
    pub memory_bandwidth_gbps: Option<f32>,
    
    /// Number of cores/processing units
    pub core_count: Option<u32>,
    
    /// Driver version
    pub driver_version: Option<String>,
    
    /// Firmware version
    pub firmware_version: Option<String>,
    
    /// Device ID
    pub device_id: Option<String>,
    
    /// Supported frameworks
    pub supported_frameworks: Vec<String>,
    
    /// Power consumption in watts
    pub power_consumption: Option<f32>,
    
    /// Operating temperature in Celsius
    pub temperature: Option<f32>,
    
    /// Clock frequency in MHz
    pub clock_frequency: Option<u32>,
    
    /// Supported data types
    pub supported_dtypes: Vec<String>,
    
    /// Additional capabilities
    pub capabilities: HashMap<String, String>,
}

impl TPUInfo {
    /// Query all available TPUs in the system
    pub fn query_all() -> Result<Vec<TPUInfo>> {
        let mut tpus = Vec::new();
        
        // Detect various TPU types
        tpus.extend(Self::detect_google_tpus()?);
        tpus.extend(Self::detect_intel_habana()?);
        tpus.extend(Self::detect_edge_tpus()?);
        tpus.extend(Self::detect_groq_lpus()?);
        tpus.extend(Self::detect_cerebras_wse()?);
        
        Ok(tpus)
    }
    
    /// Detect Google TPUs (Cloud and Edge)
    fn detect_google_tpus() -> Result<Vec<TPUInfo>> {
        let mut tpus = Vec::new();
        
        // Check for Google Cloud TPU via environment
        if let Ok(tpu_name) = std::env::var("TPU_NAME") {
            // Parse TPU version from name or metadata
            let (architecture, tops, memory_gb) = Self::parse_google_tpu_specs(&tpu_name);
            
            tpus.push(TPUInfo {
                vendor: TPUVendor::Google,
                model_name: format!("Google Cloud TPU ({tpu_name})"),
                architecture,
                connection_type: TPUConnectionType::CloudTPU,
                tops_performance: Some(tops),
                memory_gb: Some(memory_gb),
                memory_bandwidth_gbps: Some(600.0), // Typical for TPU v4
                core_count: Some(2), // Typical core count
                driver_version: Self::get_tpu_driver_version(),
                firmware_version: None,
                device_id: Some(tpu_name),
                supported_frameworks: vec![
                    "TensorFlow".to_string(),
                    "JAX".to_string(),
                    "PyTorch/XLA".to_string(),
                ],
                power_consumption: Some(200.0), // Estimated
                temperature: None,
                clock_frequency: Some(1000), // ~1GHz
                supported_dtypes: vec![
                    "bfloat16".to_string(),
                    "float32".to_string(),
                    "int8".to_string(),
                    "int32".to_string(),
                ],
                capabilities: HashMap::from([
                    ("matrix_units".to_string(), "true".to_string()),
                    ("vector_units".to_string(), "true".to_string()),
                    ("scalar_units".to_string(), "true".to_string()),
                ]),
            });
        }
        
        Ok(tpus)
    }
    
    fn parse_google_tpu_specs(tpu_name: &str) -> (TPUArchitecture, f32, f32) {
        if tpu_name.contains("v5") {
            (TPUArchitecture::GoogleTPUv5, 275.0, 16.0)
        } else if tpu_name.contains("v4") {
            (TPUArchitecture::GoogleTPUv4, 275.0, 32.0)
        } else if tpu_name.contains("v3") {
            (TPUArchitecture::GoogleTPUv3, 123.0, 16.0)
        } else if tpu_name.contains("v2") {
            (TPUArchitecture::GoogleTPUv2, 45.0, 8.0)
        } else {
            (TPUArchitecture::GoogleTPUv4, 275.0, 32.0) // Default to v4
        }
    }
    
    /// Detect Google Coral Edge TPUs
    fn detect_edge_tpus() -> Result<Vec<TPUInfo>> {
        let tpus = Vec::new();
        
        #[cfg(target_os = "linux")]
        {
            // Check for Edge TPU via USB
            if let Ok(output) = Command::new("lsusb").output() {
                let output_str = String::from_utf8_lossy(&output.stdout);
                for line in output_str.lines() {
                    if line.contains("18d1") && line.contains("9302") { // Google Edge TPU USB
                        tpus.push(TPUInfo {
                            vendor: TPUVendor::Google,
                            model_name: "Google Coral Edge TPU".to_string(),
                            architecture: TPUArchitecture::GoogleCoralEdge,
                            connection_type: TPUConnectionType::USB,
                            tops_performance: Some(4.0), // 4 TOPS at INT8
                            memory_gb: None, // Uses host memory
                            memory_bandwidth_gbps: Some(2.0), // USB 3.0 bandwidth
                            core_count: Some(1),
                            driver_version: Self::get_edge_tpu_driver_version(),
                            firmware_version: None,
                            device_id: Some("18d1:9302".to_string()),
                            supported_frameworks: vec![
                                "TensorFlow Lite".to_string(),
                                "PyCoral".to_string(),
                                "OpenVINO".to_string(),
                            ],
                            power_consumption: Some(2.0), // ~2W
                            temperature: None,
                            clock_frequency: Some(500), // ~500MHz
                            supported_dtypes: vec![
                                "int8".to_string(),
                                "uint8".to_string(),
                            ],
                            capabilities: HashMap::from([
                                ("quantized_only".to_string(), "true".to_string()),
                                ("edge_optimized".to_string(), "true".to_string()),
                            ]),
                        });
                    }
                }
            }
            
            // Check for Edge TPU via PCIe (M.2 or Mini PCIe)
            if let Ok(output) = Command::new("lspci").output() {
                let output_str = String::from_utf8_lossy(&output.stdout);
                for line in output_str.lines() {
                    if line.contains("Coral") || (line.contains("Google") && line.contains("Edge")) {
                        tpus.push(TPUInfo {
                            vendor: TPUVendor::Google,
                            model_name: "Google Coral Edge TPU (PCIe)".to_string(),
                            architecture: TPUArchitecture::GoogleCoralEdge,
                            connection_type: TPUConnectionType::M2,
                            tops_performance: Some(4.0),
                            memory_gb: None,
                            memory_bandwidth_gbps: Some(8.0), // PCIe bandwidth
                            core_count: Some(1),
                            driver_version: Self::get_edge_tpu_driver_version(),
                            firmware_version: None,
                            device_id: None,
                            supported_frameworks: vec![
                                "TensorFlow Lite".to_string(),
                                "PyCoral".to_string(),
                            ],
                            power_consumption: Some(2.5), // Slightly higher for PCIe
                            temperature: None,
                            clock_frequency: Some(500),
                            supported_dtypes: vec![
                                "int8".to_string(),
                                "uint8".to_string(),
                            ],
                            capabilities: HashMap::from([
                                ("quantized_only".to_string(), "true".to_string()),
                                ("edge_optimized".to_string(), "true".to_string()),
                                ("pcie_interface".to_string(), "true".to_string()),
                            ]),
                        });
                    }
                }
            }
        }
        
        Ok(tpus)
    }
    
    /// Detect Intel Habana accelerators
    fn detect_intel_habana() -> Result<Vec<TPUInfo>> {
        let tpus = Vec::new();
        
        #[cfg(target_os = "linux")]
        {
            // Check for Habana devices via sysfs
            if std::path::Path::new("/sys/class/accel").exists() {
                if let Ok(entries) = std::fs::read_dir("/sys/class/accel") {
                    for entry in entries.flatten() {
                        if let Some(name) = entry.file_name().to_str() {
                            if name.starts_with("accel") {
                                // Try to determine if it's Habana
                                let device_path = format!("/sys/class/accel/{}/device", name);
                                if let Ok(vendor) = std::fs::read_to_string(format!("{}/vendor", device_path)) {
                                    if vendor.trim() == "0x1da3" { // Intel vendor ID for Habana
                                        if let Ok(device) = std::fs::read_to_string(format!("{}/device", device_path)) {
                                            let (model_name, architecture, tops) = match device.trim() {
                                                "0x1000" => ("Intel Habana Gaudi", TPUArchitecture::IntelHabanaGaudi, 400.0),
                                                "0x1020" => ("Intel Habana Gaudi2", TPUArchitecture::IntelHabanaGaudi2, 800.0),
                                                "0x1050" => ("Intel Habana Goya", TPUArchitecture::IntelHabanaGoya, 100.0),
                                                _ => ("Intel Habana Device", TPUArchitecture::IntelHabanaGaudi, 400.0),
                                            };
                                            
                                            tpus.push(TPUInfo {
                                                vendor: TPUVendor::Intel,
                                                model_name: model_name.to_string(),
                                                architecture,
                                                connection_type: TPUConnectionType::PCIe,
                                                tops_performance: Some(tops),
                                                memory_gb: Some(32.0), // Typical for Gaudi
                                                memory_bandwidth_gbps: Some(2400.0), // HBM2E bandwidth
                                                core_count: Some(8), // Typical core count
                                                driver_version: Self::get_habana_driver_version(),
                                                firmware_version: None,
                                                device_id: Some(device.trim().to_string()),
                                                supported_frameworks: vec![
                                                    "PyTorch".to_string(),
                                                    "TensorFlow".to_string(),
                                                    "ONNX Runtime".to_string(),
                                                    "Habana SynapseAI".to_string(),
                                                ],
                                                power_consumption: Some(350.0), // High performance = high power
                                                temperature: None,
                                                clock_frequency: Some(1300), // ~1.3GHz
                                                supported_dtypes: vec![
                                                    "float32".to_string(),
                                                    "bfloat16".to_string(),
                                                    "float16".to_string(),
                                                    "int8".to_string(),
                                                ],
                                                capabilities: HashMap::from([
                                                    ("matrix_multiply_engine".to_string(), "true".to_string()),
                                                    ("tensor_processor_core".to_string(), "true".to_string()),
                                                    ("high_bandwidth_memory".to_string(), "true".to_string()),
                                                ]),
                                            });
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        Ok(tpus)
    }
    
    // Placeholder implementations for other vendors
    fn detect_groq_lpus() -> Result<Vec<TPUInfo>> {
        // Note: Groq LPU detection requires Groq SDK and runtime
        Ok(Vec::new())
    }
    
    fn detect_cerebras_wse() -> Result<Vec<TPUInfo>> {
        // Note: Cerebras WSE detection requires Cerebras SDK and runtime
        Ok(Vec::new())
    }
    
    // Helper functions for driver version detection
    fn get_tpu_driver_version() -> Option<String> {
        // Try to get TensorFlow version with TPU support
        #[cfg(target_os = "linux")]
        {
            if let Ok(output) = Command::new("python3")
                .args(["-c", "import tensorflow; print(tensorflow.__version__)"])
                .output()
            {
                if output.status.success() {
                    return Some(format!("TensorFlow {}", String::from_utf8_lossy(&output.stdout).trim()));
                }
            }
        }
        None
    }
    
    /// Get TPU vendor
    pub fn vendor(&self) -> &TPUVendor {
        &self.vendor
    }
    
    /// Get TPU model name
    pub fn model_name(&self) -> &str {
        &self.model_name
    }
    
    /// Get TPU architecture
    pub fn architecture(&self) -> &TPUArchitecture {
        &self.architecture
    }
    
    /// Get performance in TOPS
    pub fn tops_performance(&self) -> Option<f32> {
        self.tops_performance
    }
    
    /// Check if TPU supports a specific framework
    pub fn supports_framework(&self, framework: &str) -> bool {
        self.supported_frameworks.iter()
            .any(|f| f.to_lowercase().contains(&framework.to_lowercase()))
    }
    
    /// Check if TPU supports a specific data type
    pub fn supports_dtype(&self, dtype: &str) -> bool {
        self.supported_dtypes.iter()
            .any(|d| d.to_lowercase() == dtype.to_lowercase())
    }
}
