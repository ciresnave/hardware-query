//! # Hardware Query
//!
//! A cross-platform Rust library for querying detailed system hardware information.
//! Provides a unified interface to access CPU, GPU, memory, disk, network, and other
//! hardware specifications across Windows, Linux, and macOS.
//!
//! ## Architecture
//!
//! This library uses a modular architecture:
//!
//! - Core modules (`cpu`, `gpu`, `memory`, etc.) provide platform-agnostic interfaces
//! - Platform-specific implementations in the `platform` module handle OS-specific detection
//! - Features flags enable optional vendor-specific hardware detection
//!
//! ## Platform Support
//!
//! * **Windows**: Uses Windows Management Instrumentation (WMI) and native Windows APIs
//! * **Linux**: Utilizes `/proc`, `/sys` filesystems and platform-specific commands
//! * **macOS**: Leverages Core Foundation, IOKit and system commands
//!
//! ## Performance Considerations
//!
//! Hardware detection operations can be relatively expensive, especially on first run.
//! Consider caching the results when appropriate rather than querying repeatedly.
//!
//! ## Thread Safety
//!
//! All public types in this library are `Send` and `Sync` unless explicitly documented otherwise.
//! The library is designed to be used safely in multithreaded environments.
//!
//! ## Features
//!
//! - Cross-platform hardware detection (Windows, Linux, macOS)
//! - Detailed CPU information (cores, threads, cache, features)
//! - GPU detection and capabilities (CUDA, ROCm, DirectML support)
//! - Memory configuration and status
//! - Storage device enumeration and properties
//! - Network interface detection and capabilities
//! - Hardware acceleration support detection
//! - Battery status and health monitoring
//! - Thermal sensors and fan control (where available)
//! - PCI/USB device enumeration
//! - Serializable hardware information
//!
//! ## Example
//!
//! ```rust
//! use hardware_query::HardwareInfo;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Get complete system information
//! let hw_info = HardwareInfo::query()?;
//!
//! // Access CPU information
//! let cpu = hw_info.cpu();
//! println!("CPU: {} {} with {} cores, {} threads",
//!     cpu.vendor(),
//!     cpu.model_name(),
//!     cpu.physical_cores(),
//!     cpu.logical_cores()
//! );
//!
//! // Check for specific CPU features
//! if cpu.has_feature("avx2") {
//!     println!("CPU supports AVX2 instructions");
//! }
//!
//! // Get GPU information
//! for (i, gpu) in hw_info.gpus().iter().enumerate() {
//!     println!("GPU {}: {} {} with {} GB VRAM",
//!         i + 1,
//!         gpu.vendor(),
//!         gpu.model_name(),
//!         gpu.memory_gb()
//!     );
//! }
//!
//! // Check specialized hardware
//! if !hw_info.npus().is_empty() {
//!     println!("NPUs detected: {} units", hw_info.npus().len());
//! }
//!
//! if !hw_info.tpus().is_empty() {
//!     println!("TPUs detected: {} units", hw_info.tpus().len());
//! }
//!
//! // Check ARM-specific hardware (Raspberry Pi, Jetson, etc.)
//! if let Some(arm) = hw_info.arm_hardware() {
//!     println!("ARM System: {}", arm.system_type);
//! }
//!
//! // Check FPGA hardware
//! if !hw_info.fpgas().is_empty() {
//!     println!("FPGAs detected: {} units", hw_info.fpgas().len());
//! }
//!
//! // Get system summary
//! let summary = hw_info.summary();
//! println!("System Summary:\n{}", summary);
//! # Ok(())
//! # }
//! ```

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
