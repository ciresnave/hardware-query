# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2025-07-18

### Added

#### 🔄 Real-time Hardware Monitoring
- **Real-time monitoring system** with configurable update intervals
- **Event-driven notifications** for thermal and power alerts
- **Background monitoring support** with async/await patterns
- **Comprehensive monitoring statistics** and callback system
- **HardwareMonitor** struct for continuous hardware tracking
- **MonitoringConfig** for customizable monitoring parameters

#### ⚡ Power Management & Efficiency
- **PowerProfile** module for comprehensive power analysis
- **Real-time power consumption tracking** across system components
- **Battery life estimation** based on current power draw
- **Power efficiency scoring** and optimization recommendations
- **Thermal throttling risk assessment** integration
- **Power optimization suggestions** with categorized recommendations
- **Multiple power state detection** (High Performance, Balanced, Power Saver, etc.)

#### 🌡️ Enhanced Thermal Management
- **Advanced thermal monitoring** with temperature history tracking
- **Thermal throttling prediction algorithms** based on workload intensity
- **Cooling optimization recommendations** with cost and difficulty ratings
- **Sustained performance capability analysis** considering thermal limits
- **Fan curve analysis** and optimization suggestions
- **TDP (Thermal Design Power) information** tracking
- **Ambient temperature detection** where supported

#### 🐳 Virtualization & Container Detection
- **Comprehensive virtualization environment detection** (Docker, Kubernetes, VM, WSL, etc.)
- **Container runtime identification** with detailed metadata
- **Resource limits and restrictions analysis** (CPU, memory, I/O, network, GPU)
- **GPU passthrough capability detection** and performance impact assessment
- **Security feature analysis** (Secure Boot, IOMMU, etc.)
- **Performance impact estimation** for different virtualization types
- **VirtualizationInfo** struct with detailed environment analysis

### Enhanced
- **HardwareInfo** struct now includes power profile and virtualization information
- **ThermalInfo** enhanced with prediction and optimization capabilities
- **BatteryInfo** now supports power estimation methods
- **Enhanced error handling** with new error types for specialized operations

### Changed
- **Feature flags** restructured to support optional monitoring capabilities
- **API methods** updated to include new power and virtualization data
- **Dependencies** updated with async runtime support for monitoring

## [0.1.0] - 2025-07-10

### Added

- Cross-platform hardware detection for Windows, Linux, and macOS
- Detailed CPU information detection with platform-specific implementations
- GPU detection with vendor-specific support (NVIDIA, AMD, Intel)
- Memory configuration and status detection
- Storage device enumeration and properties
- Network interface detection
- Battery status and health monitoring
- Thermal sensors and fan control information
- PCI and USB device enumeration
- Advanced AI capabilities analysis framework
- Hardware capability scoring system for workload placement
- Platform-specific implementations in separate modules
- JSON serialization/deserialization for all hardware information
- Comprehensive example applications
- Feature flags for optional GPU vendor-specific support

### Fixed

- Memory size calculation (KB vs MB issue)
- AICapabilities vs AdvancedAICapabilities reference in hardware_info.rs
- GPU detection implementation for AMD GPUs
- Platform-specific implementations consistency
