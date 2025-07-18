//! # Hardware Query
//!
//! **The easiest way to get hardware information in Rust.**
//!
//! This crate provides a simple, cross-platform API for hardware detection and system monitoring.
//! Whether you need a quick system overview or detailed hardware analysis, there's an API tier for you.
//!
//! ## Quick Start (1 line of code)
//!
//! Get a complete system overview with health status:
//!
//! ```rust
//! use hardware_query::SystemOverview;
//! 
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let overview = SystemOverview::quick()?;
//! println!("{}", overview);  // Formatted system summary with health status
//! # Ok(())
//! # }
//! ```
//!
//! ## Domain-Specific Presets (2-3 lines)
//!
//! Get assessments tailored to your use case:
//!
//! ```rust
//! use hardware_query::HardwarePresets;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // For AI/ML applications
//! let ai_assessment = HardwarePresets::ai_assessment()?;
//! println!("AI Score: {}/100", ai_assessment.ai_score);
//! println!("Supported Frameworks: {:?}", ai_assessment.frameworks);
//!
//! // For gaming applications  
//! let gaming_assessment = HardwarePresets::gaming_assessment()?;
//! println!("Gaming Score: {}/100", gaming_assessment.gaming_score);
//! println!("Recommended Settings: {}", gaming_assessment.recommended_settings);
//!
//! // For development environments
//! let dev_assessment = HardwarePresets::developer_assessment()?;
//! println!("Build Performance: {:?}", dev_assessment.build_performance);
//! # Ok(())
//! # }
//! ```
//!
//! ## Custom Queries (3-5 lines)
//!
//! Build exactly the hardware query you need:
//!
//! ```rust
//! use hardware_query::HardwareQueryBuilder;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Get basic system info
//! let basic_info = HardwareQueryBuilder::new()
//!     .with_basic()
//!     .cpu_and_memory()?;
//!
//! // Get AI-focused hardware info
//! let ai_info = HardwareQueryBuilder::new()
//!     .with_ai_focused()
//!     .gpu_and_accelerators()?;
//!
//! // Get everything for system monitoring
//! let monitoring_info = HardwareQueryBuilder::new()
//!     .with_monitoring()
//!     .all_hardware()?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Complete Hardware Analysis (Advanced)
//!
//! For detailed hardware analysis and custom processing:
//!
//! ```rust
//! use hardware_query::HardwareInfo;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Get complete system information
//! let hw_info = HardwareInfo::query()?;
//!
//! // Access detailed CPU information
//! let cpu = hw_info.cpu();
//! println!("CPU: {} {} - {} cores, {} threads",
//!     cpu.vendor(),
//!     cpu.model_name(),
//!     cpu.physical_cores(),
//!     cpu.logical_cores()
//! );
//!
//! // Check specific CPU features for optimization
//! if cpu.has_feature("avx2") && cpu.has_feature("fma") {
//!     println!("CPU optimized for SIMD operations");
//! }
//!
//! // Analyze GPU capabilities for AI workloads
//! for gpu in hw_info.gpus() {
//!     println!("GPU: {} {} - {} GB VRAM", 
//!         gpu.vendor(), gpu.model_name(), gpu.memory_gb());
//!     
//!     if gpu.supports_cuda() {
//!         println!("  CUDA support available");
//!     }
//!     if gpu.supports_opencl() {
//!         println!("  OpenCL support available");
//!     }
//! }
//!
//! // Check for specialized AI hardware
//! if !hw_info.npus().is_empty() {
//!     println!("AI accelerators found: {} NPUs", hw_info.npus().len());
//! }
//!
//! // Memory analysis for performance optimization
//! let memory = hw_info.memory();
//! println!("Memory: {} GB total, {} GB available",
//!     memory.total_gb(),
//!     memory.available_gb()
//! );
//!
//! // Storage performance characteristics
//! for storage in hw_info.storage() {
//!     println!("Storage: {} - {} GB ({})",
//!         storage.model(),
//!         storage.capacity_gb(),
//!         if storage.is_ssd() { "SSD" } else { "HDD" }
//!     );
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Monitoring and Real-time Updates
//!
//! For applications that need continuous hardware monitoring:
//!
//! ```rust,no_run
//! #[cfg(feature = "monitoring")]
//! use hardware_query::{HardwareMonitor, MonitoringConfig};
//!
//! # #[cfg(feature = "monitoring")]
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let config = MonitoringConfig::new()
//!     .with_cpu_monitoring(true)
//!     .with_thermal_monitoring(true)
//!     .with_interval_ms(1000);
//!
//! let mut monitor = HardwareMonitor::new(config);
//!
//! monitor.start_monitoring(|event| {
//!     match event {
//!         hardware_query::MonitoringEvent::TemperatureAlert { component, temp } => {
//!             println!("Warning: {} temperature: {}°C", component, temp);
//!         }
//!         hardware_query::MonitoringEvent::CpuUsageHigh { usage } => {
//!             println!("High CPU usage: {}%", usage);
//!         }
//!         _ => {}
//!     }
//! })?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Feature Flags
//!
//! - **Default**: Basic hardware detection (CPU, Memory, GPU, Storage)
//! - **`monitoring`**: Real-time monitoring capabilities, thermal sensors, power management
//! - **`serde`**: Serialization/deserialization support (automatically enabled)
//!
//! ## Platform Support
//!
//! - **Windows**: Native WMI and Windows API support
//! - **Linux**: Comprehensive `/proc`, `/sys` filesystem support  
//! - **macOS**: IOKit and system framework integration
//!
//! All APIs work consistently across platforms, with graceful degradation when specific hardware isn't available.

mod battery;
mod cpu;
mod error;
mod gpu;
mod hardware_info;
mod memory;
mod network;
mod npu;
mod pci;
pub mod platform;
mod storage;
mod thermal;
mod tpu;
mod usb;
mod arm;
mod fpga;
mod power;
mod virtualization;

#[cfg(feature = "monitoring")]
mod monitoring;

// Simplified API modules
pub mod simple;
pub mod builder;
pub mod presets;

pub use battery::{BatteryInfo, BatteryStatus};
pub use cpu::{CPUFeature, CPUInfo, CPUVendor};
pub use error::{HardwareQueryError, Result};
pub use gpu::{GPUInfo, GPUType, GPUVendor};
pub use hardware_info::HardwareInfo;
pub use memory::{MemoryInfo, MemoryType};
pub use network::{NetworkInfo, NetworkType};
pub use npu::{NPUInfo, NPUVendor, NPUType, NPUArchitecture};
pub use pci::PCIDevice;
pub use storage::{StorageInfo, StorageType};
pub use thermal::{FanInfo, ThermalInfo, ThermalSensor, ThrottlingPrediction, CoolingRecommendation, ThrottlingSeverity};
pub use tpu::{TPUInfo, TPUVendor, TPUArchitecture, TPUConnectionType};
pub use usb::USBDevice;
pub use arm::{ARMHardwareInfo, ARMSystemType, PowerInfo};
pub use fpga::{FPGAInfo, FPGAVendor, FPGAFamily, FPGAInterface};
pub use power::{PowerProfile, PowerState, ThrottlingRisk, PowerOptimization, OptimizationCategory};
pub use virtualization::{VirtualizationInfo, VirtualizationType, ContainerRuntime, ResourceLimits};

#[cfg(feature = "monitoring")]
pub use monitoring::{HardwareMonitor, MonitoringConfig, MonitoringEvent, MonitoringStats, MonitoringCallback};

// Simplified API exports - these are the recommended entry points for most users
pub use simple::{SystemOverview, SimpleCPU, SimpleGPU, SimpleStorage, SystemHealth, 
                 HealthStatus, TemperatureStatus, PowerStatus};
pub use builder::{HardwareQueryBuilder, CustomHardwareInfo};
pub use presets::{HardwarePresets, AIHardwareAssessment, GamingHardwareAssessment, 
                  DeveloperHardwareAssessment, ServerHardwareAssessment};
