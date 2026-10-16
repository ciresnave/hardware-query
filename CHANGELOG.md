# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.3] - 2026-10-16

### Fixed
- `cargo fmt --all` had never been run against current stable's formatting
  defaults (no `rustfmt.toml` was ever pinned); applied with no behavior
  change.
- All 19 `clippy --all-targets --all-features -- -D warnings` findings:
  unused import/variable, 3x unnecessary `mut`, 4x `map_or(false, ..)` ->
  `is_some_and`, missing `Default` impl for `HardwareQueryBuilder`,
  unnecessary cast, `async_trait`'s double-`must_use` false positive, and
  `large_enum_variant` on `MonitoringEvent` (left un-boxed: boxing would
  change public field types, a breaking change this release doesn't make).
- 2 genuinely dead private items: `HardwareQueryBuilder::include_capabilities`
  (no setter ever existed, `query()` never read it -- deleted, not an
  unfinished feature) and 4 GPU-classification helpers plus 1 NPU driver-
  version helper that are only ever called from Windows-only code paths
  (now `#[cfg(target_os = "windows")]`, matching their only callers, instead
  of `#[allow(dead_code)]`).
- `upper_case_acronyms` on `GPUPassthroughType`/`SecurityFeature`: allowed
  with a reason comment rather than renaming the public variants (SR-IOV,
  IOMMU, etc. are the industry names; renaming would be a breaking change).
- 6 of 7 doctests in `src/lib.rs` and `src/simple.rs` were broken, referencing
  a builder/monitoring API that no longer exists. Fixed per-doctest: 4 were
  simply wrong docs (builder chaining, `storage_devices()`/`drive_type()`,
  `MonitoringConfig`'s current struct-literal + async `on_event`/
  `start_monitoring` API, `dev_score` instead of a nonexistent
  `build_performance` field). The `SystemOverview::quick()` one-liner pitch
  ("formatted system summary") had no `Display` impl backing it at all --
  added `impl Display for SystemOverview` rather than rewrite the doc,
  since a working one-line summary is this crate's stated purpose.
- 2 unrelated dead-code/style lints clippy only surfaced once the lib itself
  compiled: an unused helper fn and a `field_reassign_with_default` in two
  examples.

## [0.2.2] - 2026-10-16

### Fixed
- The crate failed to compile on Linux (`#[cfg(target_os = "linux")]` paths):
  13 `E0433` in `cpu.rs` from `fs::read_to_string` calls with no `use std::fs`,
  and 3 `E0599` in `tpu.rs` from calls to `get_edge_tpu_driver_version` /
  `get_habana_driver_version`, neither of which was ever defined. This fix was
  already merged to `main` (#4, `fix: make it build`) but never released --
  0.2.1 on crates.io predates it. This release just ships that existing fix.

### Added
- An isolated `build-linux` CI job (`cargo build --all-features` on
  `ubuntu-latest`, no fmt/clippy steps ahead of it) so a Linux build
  regression shows up as its own red instead of being masked by the combined
  `test` job's earlier formatting step.

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
