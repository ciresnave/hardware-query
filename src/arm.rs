use crate::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[cfg(target_arch = "aarch64")]
use std::process::Command;

/// ARM-based system type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ARMSystemType {
    RaspberryPi,
    NVIDIAJetson,
    AppleSilicon,
    QualcommSnapdragon,
    MediaTekDimensity,
    SamsungExynos,
    HiSiliconKirin,
    AmazonGraviton,
    AmpereAltra,
    Unknown(String),
}

impl std::fmt::Display for ARMSystemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ARMSystemType::RaspberryPi => write!(f, "Raspberry Pi"),
            ARMSystemType::NVIDIAJetson => write!(f, "NVIDIA Jetson"),
            ARMSystemType::AppleSilicon => write!(f, "Apple Silicon"),
            ARMSystemType::QualcommSnapdragon => write!(f, "Qualcomm Snapdragon"),
            ARMSystemType::MediaTekDimensity => write!(f, "MediaTek Dimensity"),
            ARMSystemType::SamsungExynos => write!(f, "Samsung Exynos"),
            ARMSystemType::HiSiliconKirin => write!(f, "HiSilicon Kirin"),
            ARMSystemType::AmazonGraviton => write!(f, "Amazon Graviton"),
            ARMSystemType::AmpereAltra => write!(f, "Ampere Altra"),
            ARMSystemType::Unknown(name) => write!(f, "{name}"),
        }
    }
}

/// ARM hardware information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ARMHardwareInfo {
    /// System type
    pub system_type: ARMSystemType,
    
    /// Board model
    pub board_model: String,
    
    /// Board revision
    pub board_revision: Option<String>,
    
    /// Serial number
    pub serial_number: Option<String>,
    
    /// CPU architecture
    pub cpu_architecture: String,
    
    /// CPU cores
    pub cpu_cores: u32,
    
    /// GPU information (if available)
    pub gpu_info: Option<String>,
    
    /// Available acceleration features
    pub acceleration_features: Vec<String>,
    
    /// AI/ML capabilities
    pub ml_capabilities: HashMap<String, String>,
    
    /// Memory configuration
    pub memory_mb: Option<u64>,
    
    /// Available interfaces
    pub interfaces: Vec<String>,
    
    /// Power information
    pub power_info: Option<PowerInfo>,
}

/// Power consumption and thermal information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerInfo {
    /// Current power consumption in watts
    pub power_consumption: Option<f32>,
    
    /// CPU temperature in Celsius
    pub cpu_temperature: Option<f32>,
    
    /// GPU temperature in Celsius (if available)
    pub gpu_temperature: Option<f32>,
    
    /// Throttling status
    pub throttling: bool,
    
    /// Power supply voltage
    pub voltage: Option<f32>,
}

impl ARMHardwareInfo {
    /// Detect ARM-based hardware information
    pub fn detect() -> Result<Option<ARMHardwareInfo>> {
        #[cfg(target_arch = "aarch64")]
        {
            // Only detect on ARM64 systems
            if let Some(hardware_info) = Self::detect_raspberry_pi()? {
                return Ok(Some(hardware_info));
            }
            
            if let Some(hardware_info) = Self::detect_nvidia_jetson()? {
                return Ok(Some(hardware_info));
            }
            
            if let Some(hardware_info) = Self::detect_apple_silicon()? {
                return Ok(Some(hardware_info));
            }
            
            if let Some(hardware_info) = Self::detect_qualcomm_snapdragon()? {
                return Ok(Some(hardware_info));
            }
            
            // Generic ARM detection
            if let Some(hardware_info) = Self::detect_generic_arm()? {
                return Ok(Some(hardware_info));
            }
        }
        
        #[cfg(not(target_arch = "aarch64"))]
        {
            // On non-ARM systems, still try to detect if we're in an emulated environment
            // or if there's ARM hardware information available
        }
        
        Ok(None)
    }
    
    #[cfg(target_arch = "aarch64")]
    fn detect_raspberry_pi() -> Result<Option<ARMHardwareInfo>> {
        // Check for Raspberry Pi specific files
        if let Ok(model) = std::fs::read_to_string("/proc/device-tree/model") {
            if model.to_lowercase().contains("raspberry pi") {
                let mut hardware_info = ARMHardwareInfo {
                    system_type: ARMSystemType::RaspberryPi,
                    board_model: model.trim_end_matches('\0').to_string(),
                    board_revision: Self::get_pi_revision(),
                    serial_number: Self::get_pi_serial(),
                    cpu_architecture: Self::get_cpu_architecture(),
                    cpu_cores: Self::get_cpu_cores(),
                    gpu_info: Some("VideoCore GPU".to_string()),
                    acceleration_features: Self::get_pi_acceleration_features(),
                    ml_capabilities: Self::get_pi_ml_capabilities(),
                    memory_mb: Self::get_memory_size(),
                    interfaces: Self::get_pi_interfaces(),
                    power_info: Self::get_pi_power_info(),
                };
                
                // Determine specific Pi model for better capabilities
                if model.contains("Pi 5") {
                    hardware_info.acceleration_features.push("VideoCore VII GPU".to_string());
                    hardware_info.ml_capabilities.insert(
                        "inference_performance".to_string(),
                        "High (ARM Cortex-A76)".to_string(),
                    );
                } else if model.contains("Pi 4") {
                    hardware_info.acceleration_features.push("VideoCore VI GPU".to_string());
                    hardware_info.ml_capabilities.insert(
                        "inference_performance".to_string(),
                        "Medium (ARM Cortex-A72)".to_string(),
                    );
                }
                
                return Ok(Some(hardware_info));
            }
        }
        
        // Alternative detection via /proc/cpuinfo
        if let Ok(cpuinfo) = std::fs::read_to_string("/proc/cpuinfo") {
            if cpuinfo.contains("BCM") && cpuinfo.contains("Raspberry Pi") {
                // Extract model from hardware line
                for line in cpuinfo.lines() {
                    if line.starts_with("Model") {
                        return Ok(Some(ARMHardwareInfo {
                            system_type: ARMSystemType::RaspberryPi,
                            board_model: line.split(':').nth(1).unwrap_or("Unknown").trim().to_string(),
                            board_revision: Self::get_pi_revision(),
                            serial_number: Self::get_pi_serial(),
                            cpu_architecture: Self::get_cpu_architecture(),
                            cpu_cores: Self::get_cpu_cores(),
                            gpu_info: Some("VideoCore GPU".to_string()),
                            acceleration_features: Self::get_pi_acceleration_features(),
                            ml_capabilities: Self::get_pi_ml_capabilities(),
                            memory_mb: Self::get_memory_size(),
                            interfaces: Self::get_pi_interfaces(),
                            power_info: Self::get_pi_power_info(),
                        }));
                    }
                }
            }
        }
        
        Ok(None)
    }
    
    #[cfg(target_arch = "aarch64")]
    fn detect_nvidia_jetson() -> Result<Option<ARMHardwareInfo>> {
        // Check for Jetson-specific files
        if let Ok(model) = std::fs::read_to_string("/proc/device-tree/model") {
            let model_lower = model.to_lowercase();
            if model_lower.contains("jetson") || model_lower.contains("tegra") {
                let jetson_model = if model_lower.contains("nano") {
                    "NVIDIA Jetson Nano"
                } else if model_lower.contains("xavier") {
                    if model_lower.contains("nx") {
                        "NVIDIA Jetson Xavier NX"
                    } else {
                        "NVIDIA Jetson AGX Xavier"
                    }
                } else if model_lower.contains("orin") {
                    if model_lower.contains("nx") {
                        "NVIDIA Jetson Orin NX"
                    } else if model_lower.contains("nano") {
                        "NVIDIA Jetson Orin Nano"
                    } else {
                        "NVIDIA Jetson AGX Orin"
                    }
                } else {
                    "NVIDIA Jetson"
                };
                
                return Ok(Some(ARMHardwareInfo {
                    system_type: ARMSystemType::NVIDIAJetson,
                    board_model: jetson_model.to_string(),
                    board_revision: Self::get_jetson_revision(),
                    serial_number: Self::get_jetson_serial(),
                    cpu_architecture: Self::get_cpu_architecture(),
                    cpu_cores: Self::get_cpu_cores(),
                    gpu_info: Some("NVIDIA GPU with CUDA support".to_string()),
                    acceleration_features: Self::get_jetson_acceleration_features(jetson_model),
                    ml_capabilities: Self::get_jetson_ml_capabilities(jetson_model),
                    memory_mb: Self::get_memory_size(),
                    interfaces: Self::get_jetson_interfaces(),
                    power_info: Self::get_jetson_power_info(),
                }));
            }
        }
        
        // Check via lshw or nvidia-smi if available
        if let Ok(output) = Command::new("nvidia-smi").arg("-L").output() {
            if output.status.success() {
                let output_str = String::from_utf8_lossy(&output.stdout);
                if output_str.contains("Tegra") || output_str.contains("Jetson") {
                    return Ok(Some(ARMHardwareInfo {
                        system_type: ARMSystemType::NVIDIAJetson,
                        board_model: "NVIDIA Jetson (detected via nvidia-smi)".to_string(),
                        board_revision: None,
                        serial_number: None,
                        cpu_architecture: Self::get_cpu_architecture(),
                        cpu_cores: Self::get_cpu_cores(),
                        gpu_info: Some(output_str.trim().to_string()),
                        acceleration_features: vec!["CUDA".to_string(), "TensorRT".to_string()],
                        ml_capabilities: HashMap::from([
                            ("cuda_support".to_string(), "true".to_string()),
                            ("tensorrt_support".to_string(), "true".to_string()),
                        ]),
                        memory_mb: Self::get_memory_size(),
                        interfaces: vec!["USB".to_string(), "Ethernet".to_string(), "WiFi".to_string()],
                        power_info: None,
                    }));
                }
            }
        }
        
        Ok(None)
    }
    
    #[cfg(target_arch = "aarch64")]
    fn detect_apple_silicon() -> Result<Option<ARMHardwareInfo>> {
        #[cfg(target_os = "macos")]
        {
            if let Ok(output) = Command::new("sysctl")
                .args(["-n", "machdep.cpu.brand_string"])
                .output()
            {
                let cpu_brand = String::from_utf8_lossy(&output.stdout);
                if cpu_brand.contains("Apple M") {
                    let chip_name = cpu_brand.trim().to_string();
                    return Ok(Some(ARMHardwareInfo {
                        system_type: ARMSystemType::AppleSilicon,
                        board_model: format!("Apple Silicon ({})", chip_name),
                        board_revision: None,
                        serial_number: None,
                        cpu_architecture: "ARM64".to_string(),
                        cpu_cores: Self::get_cpu_cores(),
                        gpu_info: Some("Apple GPU".to_string()),
                        acceleration_features: vec![
                            "Apple Neural Engine".to_string(),
                            "Metal".to_string(),
                            "AMX".to_string(),
                        ],
                        ml_capabilities: HashMap::from([
                            ("neural_engine".to_string(), "true".to_string()),
                            ("core_ml".to_string(), "true".to_string()),
                            ("metal_performance_shaders".to_string(), "true".to_string()),
                        ]),
                        memory_mb: Self::get_memory_size(),
                        interfaces: vec!["Thunderbolt".to_string(), "USB-C".to_string(), "WiFi".to_string()],
                        power_info: None,
                    }));
                }
            }
        }
        Ok(None)
    }
    
    // Helper functions for hardware detection
    #[cfg(target_arch = "aarch64")]
    fn get_pi_revision() -> Option<String> {
        std::fs::read_to_string("/proc/cpuinfo")
            .ok()?
            .lines()
            .find(|line| line.starts_with("Revision"))
            .and_then(|line| line.split(':').nth(1))
            .map(|s| s.trim().to_string())
    }
    
    #[cfg(target_arch = "aarch64")]
    fn get_pi_serial() -> Option<String> {
        std::fs::read_to_string("/proc/cpuinfo")
            .ok()?
            .lines()
            .find(|line| line.starts_with("Serial"))
            .and_then(|line| line.split(':').nth(1))
            .map(|s| s.trim().to_string())
    }
    
    #[cfg(target_arch = "aarch64")]
    fn get_cpu_architecture() -> String {
        std::env::consts::ARCH.to_string()
    }
    
    #[cfg(target_arch = "aarch64")]
    fn get_cpu_cores() -> u32 {
        num_cpus::get() as u32
    }
    
    #[cfg(target_arch = "aarch64")]
    fn get_memory_size() -> Option<u64> {
        if let Ok(meminfo) = std::fs::read_to_string("/proc/meminfo") {
            for line in meminfo.lines() {
                if line.starts_with("MemTotal:") {
                    if let Some(kb_str) = line.split_whitespace().nth(1) {
                        if let Ok(kb) = kb_str.parse::<u64>() {
                            return Some(kb / 1024); // Convert KB to MB
                        }
                    }
                }
            }
        }
        None
    }
    
    // Placeholder implementations - would be expanded with real detection
    #[cfg(target_arch = "aarch64")]
    fn detect_qualcomm_snapdragon() -> Result<Option<ARMHardwareInfo>> {
        Ok(None)
    }
    
    #[cfg(target_arch = "aarch64")]
    fn detect_generic_arm() -> Result<Option<ARMHardwareInfo>> {
        Ok(None)
    }
    
    #[cfg(target_arch = "aarch64")]
    fn get_pi_acceleration_features() -> Vec<String> {
        vec!["VideoCore GPU".to_string(), "Hardware Video Decode".to_string()]
    }
    
    #[cfg(target_arch = "aarch64")]
    fn get_pi_ml_capabilities() -> HashMap<String, String> {
        HashMap::from([
            ("cpu_inference".to_string(), "true".to_string()),
            ("frameworks".to_string(), "TensorFlow Lite, PyTorch".to_string()),
        ])
    }
    
    #[cfg(target_arch = "aarch64")]
    fn get_pi_interfaces() -> Vec<String> {
        vec!["GPIO".to_string(), "I2C".to_string(), "SPI".to_string(), "UART".to_string()]
    }
    
    #[cfg(target_arch = "aarch64")]
    fn get_pi_power_info() -> Option<PowerInfo> {
        // Try to read Pi-specific thermal info
        let cpu_temp = std::fs::read_to_string("/sys/class/thermal/thermal_zone0/temp")
            .ok()
            .and_then(|temp_str| temp_str.trim().parse::<f32>().ok())
            .map(|temp| temp / 1000.0); // Convert millidegrees to degrees
        
        Some(PowerInfo {
            power_consumption: None, // Pi doesn't expose this easily
            cpu_temperature: cpu_temp,
            gpu_temperature: None,
            throttling: false, // Would need to check throttling flags
            voltage: None,
        })
    }
    
    // Jetson-specific helper functions
    #[cfg(target_arch = "aarch64")]
    fn get_jetson_revision() -> Option<String> {
        None // Placeholder
    }
    
    #[cfg(target_arch = "aarch64")]
    fn get_jetson_serial() -> Option<String> {
        None // Placeholder
    }
    
    #[cfg(target_arch = "aarch64")]
    fn get_jetson_acceleration_features(model: &str) -> Vec<String> {
        let mut features = vec!["CUDA".to_string(), "TensorRT".to_string()];
        
        if model.contains("Orin") {
            features.push("Ampere GPU".to_string());
            features.push("NVENC/NVDEC".to_string());
        } else if model.contains("Xavier") {
            features.push("Volta GPU".to_string());
            features.push("Deep Learning Accelerator".to_string());
        }
        
        features
    }
    
    #[cfg(target_arch = "aarch64")]
    fn get_jetson_ml_capabilities(model: &str) -> HashMap<String, String> {
        let mut capabilities = HashMap::from([
            ("cuda_support".to_string(), "true".to_string()),
            ("tensorrt_support".to_string(), "true".to_string()),
            ("deep_learning_accelerator".to_string(), "true".to_string()),
        ]);
        
        if model.contains("Orin") {
            capabilities.insert("inference_performance".to_string(), "Very High".to_string());
            capabilities.insert("training_support".to_string(), "Yes".to_string());
        } else if model.contains("Xavier") {
            capabilities.insert("inference_performance".to_string(), "High".to_string());
            capabilities.insert("training_support".to_string(), "Limited".to_string());
        }
        
        capabilities
    }
    
    #[cfg(target_arch = "aarch64")]
    fn get_jetson_interfaces() -> Vec<String> {
        vec![
            "USB".to_string(),
            "Ethernet".to_string(),
            "WiFi".to_string(),
            "GPIO".to_string(),
            "I2C".to_string(),
            "SPI".to_string(),
            "UART".to_string(),
            "CSI Camera".to_string(),
            "HDMI".to_string(),
        ]
    }
    
    #[cfg(target_arch = "aarch64")]
    fn get_jetson_power_info() -> Option<PowerInfo> {
        None // Placeholder - would read from Jetson power monitoring
    }
}
