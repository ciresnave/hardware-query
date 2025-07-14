# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
