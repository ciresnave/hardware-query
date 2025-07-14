<!-- Use this file to provide workspace-specific custom instructions to Copilot. For more details, visit https://code.visualstudio.com/docs/copilot/copilot-customization#_use-a-githubcopilotinstructionsmd-file -->

# Hardware Query Crate - Copilot Instructions

This is a Rust library project for cross-platform hardware information querying. When working on this codebase:

## Project Overview
- **Purpose**: Cross-platform hardware detection and capability analysis
- **Primary use case**: AI workload placement and hardware capability assessment
- **Target platforms**: Windows, Linux, macOS
- **Architecture**: Modular design with platform-specific implementations

## Code Style Guidelines
- Use Rust idioms and best practices
- Prefer `Result<T>` return types for fallible operations
- Use `serde` for serialization/deserialization
- Follow the existing error handling patterns using `thiserror`
- Use platform-conditional compilation (`#[cfg(target_os = "...")]`) for OS-specific code

## Module Structure
- `error.rs` - Custom error types and Result aliases
- `hardware_info.rs` - Main API entry point and data aggregation
- `cpu.rs` - CPU information and feature detection
- `gpu.rs` - GPU information and compute capabilities
- `memory.rs` - System memory and modules information
- `storage.rs` - Storage devices and filesystem information
- `network.rs` - Network interfaces and connectivity
- `battery.rs` - Battery status and health monitoring
- `thermal.rs` - Temperature sensors and fan control
- `pci.rs` - PCI device enumeration
- `usb.rs` - USB device enumeration
- `capabilities.rs` - AI acceleration capability analysis

## Key Design Principles
1. **Cross-platform compatibility**: Use conditional compilation and platform-specific implementations
2. **Performance**: Minimize system calls and cache results when appropriate
3. **Accuracy**: Prefer accurate hardware detection over speed
4. **Extensibility**: Design APIs to easily add new hardware types and capabilities
5. **Safety**: Use safe Rust patterns and handle all error cases gracefully

## Platform-Specific Implementation Notes
- **Windows**: Use WMI and Windows APIs for detailed hardware information
- **Linux**: Use `/proc`, `/sys` filesystems and library APIs
- **macOS**: Use Core Foundation and IOKit for hardware detection

## AI Capabilities Framework
The capabilities module is central to the library's value proposition:
- Analyze hardware for AI workload suitability
- Score hardware performance for different AI tasks
- Recommend optimal configurations for specific models
- Support multiple acceleration frameworks (CUDA, ROCm, DirectML, etc.)

## Testing Guidelines
- Write unit tests for each module
- Include integration tests for cross-platform compatibility
- Mock platform-specific calls in tests when possible
- Test error handling paths thoroughly

## Documentation Standards
- Include comprehensive rustdoc comments
- Provide usage examples in module-level documentation
- Document platform-specific behavior and limitations
- Include performance characteristics where relevant

## Dependencies Management
- Keep dependencies minimal and well-justified
- Use feature flags for optional functionality (GPU drivers, etc.)
- Prefer mature, well-maintained crates
- Consider binary size impact for embedded use cases
