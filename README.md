# Hardware Query

A cross-platform Rust library for querying detailed system hardware information.

## Features

- ✅ Cross-platform hardware detection (Windows, Linux, macOS)
- ✅ Detailed CPU information (cores, threads, cache, features)
- ✅ GPU detection and capabilities (CUDA, ROCm, DirectML support)
- ✅ Memory configuration and status
- ✅ Storage device enumeration and properties
- ✅ Network interface detection and capabilities
- ✅ Hardware acceleration support detection
- ✅ Battery status and health monitoring
- ✅ Thermal sensors and fan control (where available)
- ✅ PCI/USB device enumeration
- ✅ Serializable hardware information

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
hardware-query = "0.1.0"
```

## Usage

```rust
use hardware_query::HardwareInfo;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get complete system information
    let hw_info = HardwareInfo::query()?;
    
    // Access CPU information
    let cpu = hw_info.cpu();
    println!("CPU: {} {} with {} cores, {} threads",
        cpu.vendor(),
        cpu.model_name(),
        cpu.physical_cores(),
        cpu.logical_cores()
    );
    
    // Check for specific CPU features
    if cpu.has_feature("avx2") {
        println!("CPU supports AVX2 instructions");
    }
    
    // Get GPU information
    for (i, gpu) in hw_info.gpus().iter().enumerate() {
        println!("GPU {}: {} {} with {} GB VRAM",
            i + 1,
            gpu.vendor(),
            gpu.model_name(),
            gpu.memory_gb()
        );
        
        // Check GPU capabilities
        println!("  CUDA support: {}", gpu.supports_cuda());
        println!("  ROCm support: {}", gpu.supports_rocm());
        println!("  DirectML support: {}", gpu.supports_directml());
    }
    
    // Memory information
    let mem = hw_info.memory();
    println!("System memory: {:.1} GB total, {:.1} GB available",
        mem.total_gb(),
        mem.available_gb()
    );
    
    // Storage information
    for (i, disk) in hw_info.storage_devices().iter().enumerate() {
        println!("Disk {}: {} {:.1} GB ({:?} type)",
            i + 1,
            disk.model,
            disk.capacity_gb,
            disk.storage_type
        );
    }
    
    // Network interfaces
    for interface in hw_info.network_interfaces() {
        println!("Network: {} - {}", 
            interface.name,
            interface.mac_address
        );
    }
    
    // Check specialized hardware
    for npu in hw_info.npus() {
        println!("NPU detected: {} {}", npu.vendor(), npu.model_name());
    }
    
    for tpu in hw_info.tpus() {
        println!("TPU detected: {} {}", tpu.vendor(), tpu.model_name());
    }
    
    // Check ARM-specific hardware (Raspberry Pi, Jetson, etc.)
    if let Some(arm) = hw_info.arm_hardware() {
        println!("ARM System: {}", arm.system_type);
    }
    
    // Check FPGA hardware
    for fpga in hw_info.fpgas() {
        println!("FPGA: {} {} with {} logic elements",
            fpga.vendor,
            fpga.family,
            fpga.logic_elements.unwrap_or(0)
        );
    }
    
    // Serialize hardware information to JSON
    let hw_json = hw_info.to_json()?;
    println!("Hardware JSON: {}", hw_json);
    
    Ok(())
}
```

## Specialized Hardware Support

The library provides comprehensive detection for AI/ML-oriented hardware:

- **NPUs**: Intel Movidius, GNA, XDNA; Apple Neural Engine; Qualcomm Hexagon
- **TPUs**: Google Cloud TPU and Edge TPU; Intel Habana
- **ARM Systems**: Raspberry Pi, NVIDIA Jetson, Apple Silicon with power management
- **FPGAs**: Intel/Altera and Xilinx families with AI optimization scoring

```rust
use hardware_query::HardwareInfo;

let hw_info = HardwareInfo::query()?;

// Check for specialized AI hardware
for npu in hw_info.npus() {
    println!("NPU: {} {}", 
        npu.vendor(), 
        npu.model_name()
    );
}

for tpu in hw_info.tpus() {
    println!("TPU: {} {}", 
        tpu.vendor(),
        tpu.model_name()
    );
}
```

## Platform Support

| Platform | CPU | GPU | Memory | Storage | Network | Battery | Thermal | PCI | USB |
|----------|-----|-----|--------|---------|---------|---------|---------|-----|-----|
| Windows  | ✅  | ✅  | ✅     | ✅      | ✅      | ✅      | ✅      | ✅  | ✅  |
| Linux    | ✅  | ✅  | ✅     | ✅      | ✅      | ✅      | ✅      | ✅  | ✅  |
| macOS    | ✅  | ✅  | ✅     | ✅      | ✅      | ✅      | ✅      | ✅  | ✅  |

## Optional Features

Enable additional GPU support with feature flags:

```toml
[dependencies]
hardware-query = { version = "0.1.0", features = ["nvidia", "amd", "intel"] }
```

- `nvidia`: NVIDIA GPU support via NVML
- `amd`: AMD GPU support via ROCm
- `intel`: Intel GPU support

## Examples

See the [examples](examples/) directory for complete usage examples:

- [Basic Hardware Detection](examples/basic_hardware.rs)
- [Comprehensive Hardware](examples/comprehensive_hardware.rs)
- [Enhanced Hardware](examples/enhanced_hardware.rs)
- [Advanced Usage](examples/advanced_usage.rs)
- [Comprehensive AI Hardware](examples/comprehensive_ai_hardware.rs)

## Building

```bash
# Build with default features
cargo build

# Build with GPU support
cargo build --features="nvidia,amd,intel"

# Run tests
cargo test

# Run examples
cargo run --example basic_hardware
```

## Dependencies

- `sysinfo` - Cross-platform system information
- `serde` - Serialization framework
- `thiserror` - Error handling
- `num_cpus` - CPU core detection

### Optional Dependencies

- `nvml-wrapper` - NVIDIA GPU support (feature: nvidia)
- `wmi` - Windows Management Instrumentation (Windows only)
- `libc` - Linux system calls (Linux only)
- `core-foundation` - macOS system APIs (macOS only)

## Performance

The library is designed for performance with:

- Lazy evaluation of hardware information
- Minimal system calls
- Efficient data structures
- Optional caching for repeated queries

Typical query times:

- Complete hardware scan: 10-50ms
- CPU information: 1-5ms
- GPU information: 5-20ms
- Memory information: 1-3ms

## Error Handling

The library uses comprehensive error handling:

```rust
use hardware_query::{HardwareInfo, HardwareQueryError};

match HardwareInfo::query() {
    Ok(hw_info) => {
        // Use hardware information
    }
    Err(HardwareQueryError::SystemInfoUnavailable(msg)) => {
        eprintln!("System info unavailable: {}", msg);
    }
    Err(HardwareQueryError::PermissionDenied(msg)) => {
        eprintln!("Permission denied: {}", msg);
    }
    Err(e) => {
        eprintln!("Hardware query error: {}", e);
    }
}
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT OR Apache-2.0 License - see the [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE) files for details.

## Acknowledgments

- Built on top of the excellent [`sysinfo`](https://github.com/GuillaumeGomez/sysinfo) crate
- Inspired by the need for better hardware detection in AI workload placement
- Thanks to all contributors who helped improve cross-platform compatibility
