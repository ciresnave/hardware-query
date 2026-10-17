# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2026-10-16

### Breaking
- `rust-version` raised 1.85 -> 1.95 (see below) -- a breaking change for
  consumers on an older toolchain, hence the major bump rather than 0.2.4
  under the pre-1.0 "second number is major" convention.

### Removed
- `core-foundation`, `core-foundation-sys`, `system-configuration` (macOS)
  and `nix`, `libc` (Linux): 5 declared dependencies with no `use` of them
  anywhere in this crate's source (confirmed by grep, not inferred --
  `libc` found independently of the other 4, same reasoning). Dead
  weight, and it shrinks the surface a future macOS/Linux dependency
  bump can break. `libc` and `core-foundation-sys` may still appear in
  `Cargo.lock` as transitive dependencies of other crates (e.g. tokio,
  nvml-wrapper) -- only our own direct, unused declarations are removed.


### Changed
- All dependencies bumped to their latest published version (CireSnave's
  standing policy: all projects' deps at their most recent version at all
  times). Most were in-range patch/minor bumps via `cargo update` alone.
  8 required raising the `Cargo.toml` requirement itself because the
  latest published release crossed a pre-1.0 "major" boundary (second
  version component): `sysinfo` 0.36->0.39, `wmi` 0.17->0.18,
  `windows` 0.61->0.62, `nix` 0.30->0.31, `system-configuration` 0.6->0.8,
  `nvml-wrapper` 0.11->0.13, `mockall` (dev) 0.13->0.15, `criterion` (dev)
  0.6->0.8.
- `rust-version` raised 1.85 -> 1.95: `sysinfo` 0.39.6 requires Rust 1.95
  and `criterion` 0.8.2 requires 1.86 (permitted per CireSnave: "Feel free
  to raise the minimum Rust version on any project").

### Fixed
- `wmi` 0.18 removed `COMLibrary` from its public API entirely --
  `WMIConnection::new()` now takes no arguments and manages COM
  initialization internally. Updated all 16 call sites across
  `cpu.rs`/`gpu.rs`/`npu.rs`/`platform/windows.rs`. Verified by an actual
  native Windows build + the full test suite (including live WMI queries
  against this machine's real hardware), not just a compile check.
- The MSRV bump to 1.95 unlocked `if`-let-chains, which made clippy's
  `collapsible_if` fire on ~46 previously-unreachable nested
  `if let`/`if` sites (43 found on Linux, 3 more that only exist in
  Windows-only code paths and only showed up building natively on
  Windows). Applied via `cargo clippy --fix`, each one a mechanical,
  semantically-identical collapse -- spot-checked by diff, confirmed by
  the full test suite staying green on both platforms.
- 2 platform-conditional `unused_mut`/`unused_import` warnings in
  `cpu.rs`/`tpu.rs` that only clippy's Windows run could see (variables
  only mutated/imports only used inside `#[cfg(target_os = "linux")]`
  blocks) -- same pattern as the dead_code fixes in 0.2.3, gated rather
  than silenced.

### Known gaps
- `core-foundation`, `core-foundation-sys`, `system-configuration` (macOS)
  and `nix` (Linux) are declared dependencies with **no `use` of them
  anywhere in this crate's source** -- confirmed by grep, not inferred.
  Their version bumps are consequently zero-risk to our own code (nothing
  calls into them), but I have no way to verify macOS compiles at all
  (no Mac, no cross-toolchain here) -- the macOS CI job is the only real
  check for `system-configuration`'s 0.6->0.8 jump (two major bumps).
  Separate question from this PR: are these 4 crates dead weight that
  should just be removed?

## [0.2.3] - 2026-10-16

### Security
- `cargo audit` on #6's CI run found 3 advisories in transitive dependencies:
  `bytes` 1.10.1 (RUSTSEC-2026-0007, integer overflow in `BytesMut::reserve`),
  `crossbeam-epoch` 0.9.18 (RUSTSEC-2026-0204, invalid pointer dereference),
  `slab` 0.4.10 (RUSTSEC-2025-0047, out-of-bounds access, also yanked).
  `cargo update -p bytes -p crossbeam-epoch -p slab` resolves all three;
  `cargo audit` is clean after.

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
