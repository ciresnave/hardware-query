/// Platform-specific hardware detection modules
///
/// This module provides enhanced platform-specific implementations
/// for detailed hardware information gathering beyond what's available
/// through generic cross-platform libraries.
#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "macos")]
pub mod macos;

/// Re-export platform-specific modules for easier access
#[cfg(target_os = "windows")]
pub use windows::*;

#[cfg(target_os = "linux")]
pub use linux::*;

#[cfg(target_os = "macos")]
pub use macos::*;
