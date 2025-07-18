//! Simplified hardware query interface - **Start here for easy hardware detection**
//!
//! This module provides the easiest way to get hardware information without complexity.
//! Perfect for getting started or when you need a quick system overview.
//!
//! ## Quick Examples
//!
//! ```rust
//! use hardware_query::SystemOverview;
//!
//! // Get everything in one line
//! let overview = SystemOverview::quick()?;
//! println!("{}", overview);  // Pretty-printed system summary
//!
//! // Access specific information
//! println!("CPU: {} cores", overview.cpu.cores);
//! println!("Memory: {:.1} GB", overview.memory_gb);
//! println!("Health: {:?}", overview.health.overall_status);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

use crate::{HardwareInfo, Result};
use serde::{Deserialize, Serialize};

/// Simplified system overview with the most commonly needed information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemOverview {
    /// CPU name and core count
    pub cpu: SimpleCPU,
    /// Total system memory in GB
    pub memory_gb: f64,
    /// GPU information (if available)
    pub gpu: Option<SimpleGPU>,
    /// Storage summary
    pub storage: SimpleStorage,
    /// System health status
    pub health: SystemHealth,
    /// Environment type (native, container, VM, etc.)
    pub environment: String,
    /// Overall performance score (0-100)
    pub performance_score: u8,
}

/// Simplified CPU information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleCPU {
    /// CPU model name
    pub name: String,
    /// Number of physical cores
    pub cores: u32,
    /// Number of logical cores (threads)
    pub threads: u32,
    /// Vendor (Intel, AMD, Apple, etc.)
    pub vendor: String,
    /// Supports AI acceleration features
    pub ai_capable: bool,
}

/// Simplified GPU information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleGPU {
    /// GPU model name
    pub name: String,
    /// VRAM in GB
    pub vram_gb: f64,
    /// Vendor (NVIDIA, AMD, Intel, etc.)
    pub vendor: String,
    /// Supports hardware acceleration for AI/ML
    pub ai_capable: bool,
}

/// Simplified storage summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleStorage {
    /// Total storage capacity in GB
    pub total_gb: f64,
    /// Available storage in GB
    pub available_gb: f64,
    /// Primary drive type (SSD, HDD, NVMe, etc.)
    pub drive_type: String,
    /// Storage health (Good, Warning, Critical)
    pub health: String,
}

/// System health overview
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemHealth {
    /// Overall health status
    pub status: HealthStatus,
    /// Current temperature status
    pub temperature: TemperatureStatus,
    /// Power consumption level
    pub power: PowerStatus,
    /// Any warnings or recommendations
    pub warnings: Vec<String>,
}

/// Overall system health status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealthStatus {
    /// Everything is running optimally
    Excellent,
    /// System is running well
    Good,
    /// Minor issues detected
    Fair,
    /// Significant issues that should be addressed
    Poor,
    /// Critical issues requiring immediate attention
    Critical,
}

/// Temperature status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TemperatureStatus {
    /// Temperatures are normal
    Normal,
    /// Temperatures are elevated but acceptable
    Warm,
    /// High temperatures detected
    Hot,
    /// Critical temperatures that may cause throttling
    Critical,
}

/// Power consumption status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PowerStatus {
    /// Low power consumption
    Low,
    /// Normal power consumption
    Normal,
    /// High power consumption
    High,
    /// Very high power consumption
    VeryHigh,
}

/// Simplified hardware query functions
impl SystemOverview {
    /// Get a quick system overview with the most important information
    /// 
    /// This is the fastest way to get a comprehensive system summary. Perfect for:
    /// - System diagnostics and health checks
    /// - Application compatibility verification  
    /// - Performance baseline establishment
    /// - Environment detection (native/container/VM)
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use hardware_query::SystemOverview;
    /// 
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let overview = SystemOverview::quick()?;
    /// 
    /// // Get formatted system summary
    /// println!("{}", overview);
    /// 
    /// // Access specific components
    /// println!("CPU: {} cores", overview.cpu.cores);
    /// println!("Memory: {:.1} GB", overview.memory_gb);
    /// 
    /// if let Some(gpu) = &overview.gpu {
    ///     println!("GPU: {} ({:.1} GB VRAM)", gpu.name, gpu.vram_gb);
    /// }
    /// 
    /// // Check system health
    /// println!("System Health: {:?}", overview.health.overall_status);
    /// println!("Performance Score: {}/100", overview.performance_score);
    /// # Ok(())
    /// # }
    /// ```
    pub fn quick() -> Result<Self> {
        let hw_info = HardwareInfo::query()?;
        Self::from_hardware_info(hw_info)
    }

    /// Create a system overview from detailed hardware information
    pub fn from_hardware_info(hw_info: HardwareInfo) -> Result<Self> {
        let cpu = SimpleCPU {
            name: hw_info.cpu().model_name().to_string(),
            cores: hw_info.cpu().physical_cores(),
            threads: hw_info.cpu().logical_cores(),
            vendor: hw_info.cpu().vendor().to_string(),
            ai_capable: Self::check_cpu_ai_capabilities(&hw_info),
        };

        let memory_gb = hw_info.memory().total_gb();

        let gpu = if !hw_info.gpus().is_empty() {
            let primary_gpu = &hw_info.gpus()[0];
            Some(SimpleGPU {
                name: primary_gpu.model_name().to_string(),
                vram_gb: primary_gpu.memory_gb(),
                vendor: primary_gpu.vendor().to_string(),
                ai_capable: Self::check_gpu_ai_capabilities(primary_gpu),
            })
        } else {
            None
        };

        let storage = Self::calculate_storage_summary(&hw_info)?;
        let health = Self::assess_system_health(&hw_info)?;
        let environment = hw_info.virtualization().environment_type.to_string();
        let performance_score = Self::calculate_performance_score(&hw_info);

        Ok(Self {
            cpu,
            memory_gb,
            gpu,
            storage,
            health,
            environment,
            performance_score,
        })
    }

    /// Check if the system is suitable for AI/ML workloads
    pub fn is_ai_ready(&self) -> bool {
        // Basic AI readiness check
        self.cpu.ai_capable || 
        self.gpu.as_ref().map_or(false, |gpu| gpu.ai_capable) ||
        self.memory_gb >= 8.0
    }

    /// Get AI/ML suitability score (0-100)
    pub fn ai_score(&self) -> u8 {
        let mut score = 0;

        // GPU contribution (50 points max)
        if let Some(gpu) = &self.gpu {
            if gpu.ai_capable {
                score += 30;
                if gpu.vram_gb >= 8.0 {
                    score += 15;
                } else if gpu.vram_gb >= 4.0 {
                    score += 10;
                } else {
                    score += 5;
                }
            }
        }

        // CPU contribution (25 points max)
        if self.cpu.ai_capable {
            score += 15;
        }
        if self.cpu.cores >= 8 {
            score += 10;
        } else if self.cpu.cores >= 4 {
            score += 5;
        }

        // Memory contribution (25 points max)
        if self.memory_gb >= 32.0 {
            score += 25;
        } else if self.memory_gb >= 16.0 {
            score += 20;
        } else if self.memory_gb >= 8.0 {
            score += 15;
        } else {
            score += 5;
        }

        score.min(100)
    }

    /// Get simple recommendations for improving system performance
    pub fn get_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();

        // Memory recommendations
        if self.memory_gb < 16.0 {
            recommendations.push("Consider upgrading to 16GB+ RAM for better performance".to_string());
        }

        // GPU recommendations
        if self.gpu.is_none() {
            recommendations.push("Add a dedicated GPU for AI/ML acceleration".to_string());
        } else if let Some(gpu) = &self.gpu {
            if gpu.vram_gb < 4.0 {
                recommendations.push("Consider a GPU with more VRAM for large AI models".to_string());
            }
        }

        // Storage recommendations
        if self.storage.drive_type.to_lowercase().contains("hdd") {
            recommendations.push("Upgrade to SSD for faster data access".to_string());
        }

        // Health recommendations
        for warning in &self.health.warnings {
            recommendations.push(warning.clone());
        }

        recommendations
    }

    fn check_cpu_ai_capabilities(hw_info: &HardwareInfo) -> bool {
        let cpu = hw_info.cpu();
        // Check for AI-relevant features
        cpu.has_feature("avx2") || 
        cpu.has_feature("avx512") || 
        cpu.has_feature("amx") ||
        !hw_info.npus().is_empty()
    }

    fn check_gpu_ai_capabilities(gpu: &crate::GPUInfo) -> bool {
        // Check if GPU supports common AI frameworks
        let vendor = gpu.vendor().to_string().to_lowercase();
        let name = gpu.model_name().to_lowercase();
        
        // NVIDIA GPUs generally support CUDA
        if vendor.contains("nvidia") {
            return true;
        }
        
        // AMD GPUs with ROCm support
        if vendor.contains("amd") && (name.contains("rx") || name.contains("radeon")) {
            return true;
        }
        
        // Intel Arc GPUs
        if vendor.contains("intel") && name.contains("arc") {
            return true;
        }
        
        false
    }

    fn calculate_storage_summary(hw_info: &HardwareInfo) -> Result<SimpleStorage> {
        let storage_devices = hw_info.storage_devices();
        
        if storage_devices.is_empty() {
            return Ok(SimpleStorage {
                total_gb: 0.0,
                available_gb: 0.0,
                drive_type: "Unknown".to_string(),
                health: "Unknown".to_string(),
            });
        }

        let total_gb: f64 = storage_devices.iter()
            .map(|device| device.capacity_gb())
            .sum();

        let available_gb: f64 = storage_devices.iter()
            .map(|device| device.available_gb())
            .sum();

        // Get primary drive type
        let drive_type = storage_devices[0].drive_type().to_string();

        // Simple health assessment
        let health = if available_gb / total_gb < 0.1 {
            "Critical - Low space".to_string()
        } else if available_gb / total_gb < 0.2 {
            "Warning - Low space".to_string()
        } else {
            "Good".to_string()
        };

        Ok(SimpleStorage {
            total_gb,
            available_gb,
            drive_type,
            health,
        })
    }

    fn assess_system_health(hw_info: &HardwareInfo) -> Result<SystemHealth> {
        let mut warnings = Vec::new();
        
        // Temperature assessment
        let thermal = hw_info.thermal();
        let temperature = if let Some(max_temp) = thermal.max_temperature() {
            if max_temp >= 90.0 {
                warnings.push("High CPU/GPU temperatures detected".to_string());
                TemperatureStatus::Critical
            } else if max_temp >= 80.0 {
                warnings.push("Elevated temperatures detected".to_string());
                TemperatureStatus::Hot
            } else if max_temp >= 70.0 {
                TemperatureStatus::Warm
            } else {
                TemperatureStatus::Normal
            }
        } else {
            TemperatureStatus::Normal
        };

        // Power assessment
        let power = if let Some(power_profile) = hw_info.power_profile() {
            if let Some(power_draw) = power_profile.total_power_draw {
                if power_draw > 200.0 {
                    PowerStatus::VeryHigh
                } else if power_draw > 100.0 {
                    PowerStatus::High
                } else if power_draw > 50.0 {
                    PowerStatus::Normal
                } else {
                    PowerStatus::Low
                }
            } else {
                PowerStatus::Normal
            }
        } else {
            PowerStatus::Normal
        };

        // Overall health status
        let status = match (&temperature, &power, warnings.len()) {
            (TemperatureStatus::Critical, _, _) => HealthStatus::Critical,
            (TemperatureStatus::Hot, PowerStatus::VeryHigh, _) => HealthStatus::Poor,
            (TemperatureStatus::Hot, _, _) => HealthStatus::Fair,
            (_, PowerStatus::VeryHigh, _) => HealthStatus::Fair,
            (_, _, n) if n > 2 => HealthStatus::Poor,
            (_, _, n) if n > 0 => HealthStatus::Fair,
            (TemperatureStatus::Normal, PowerStatus::Normal | PowerStatus::Low, 0) => HealthStatus::Excellent,
            _ => HealthStatus::Good,
        };

        Ok(SystemHealth {
            status,
            temperature,
            power,
            warnings,
        })
    }

    fn calculate_performance_score(hw_info: &HardwareInfo) -> u8 {
        let mut score = 0;

        // CPU score (30 points)
        let cpu_cores = hw_info.cpu().logical_cores();
        score += match cpu_cores {
            cores if cores >= 16 => 30,
            cores if cores >= 8 => 25,
            cores if cores >= 4 => 20,
            _ => 10,
        };

        // Memory score (25 points)
        let memory_gb = hw_info.memory().total_gb();
        score += match memory_gb {
            mem if mem >= 32.0 => 25,
            mem if mem >= 16.0 => 20,
            mem if mem >= 8.0 => 15,
            _ => 5,
        };

        // GPU score (30 points)
        if !hw_info.gpus().is_empty() {
            let gpu = &hw_info.gpus()[0];
            let vram = gpu.memory_gb();
            score += match vram {
                vram if vram >= 12.0 => 30,
                vram if vram >= 8.0 => 25,
                vram if vram >= 4.0 => 20,
                _ => 10,
            };
        }

        // Storage score (15 points)
        let storage_devices = hw_info.storage_devices();
        if !storage_devices.is_empty() {
            let storage_type = storage_devices[0].drive_type().to_string().to_lowercase();
            score += if storage_type.contains("nvme") {
                15
            } else if storage_type.contains("ssd") {
                12
            } else {
                5
            };
        }

        // Virtualization penalty
        let virt_factor = hw_info.virtualization().get_performance_factor();
        score = ((score as f64) * virt_factor) as u8;

        score.min(100)
    }
}

// Display implementations for better debugging
impl std::fmt::Display for HealthStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HealthStatus::Excellent => write!(f, "Excellent"),
            HealthStatus::Good => write!(f, "Good"),
            HealthStatus::Fair => write!(f, "Fair"),
            HealthStatus::Poor => write!(f, "Poor"),
            HealthStatus::Critical => write!(f, "Critical"),
        }
    }
}

impl std::fmt::Display for TemperatureStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TemperatureStatus::Normal => write!(f, "Normal"),
            TemperatureStatus::Warm => write!(f, "Warm"),
            TemperatureStatus::Hot => write!(f, "Hot"),
            TemperatureStatus::Critical => write!(f, "Critical"),
        }
    }
}

impl std::fmt::Display for PowerStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PowerStatus::Low => write!(f, "Low"),
            PowerStatus::Normal => write!(f, "Normal"),
            PowerStatus::High => write!(f, "High"),
            PowerStatus::VeryHigh => write!(f, "Very High"),
        }
    }
}
