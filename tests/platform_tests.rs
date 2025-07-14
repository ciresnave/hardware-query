//! Platform-specific hardware detection tests

use hardware_query::{CPUInfo, CPUVendor};

// These tests only run on their respective platforms

#[cfg(target_os = "windows")]
#[test]
fn test_windows_platform_detection() {
    // Test basic Windows-specific hardware detection works
    let cpu_info = CPUInfo::query().expect("Failed to get Windows CPU info");
    assert!(cpu_info.physical_cores() > 0);
    assert!(cpu_info.logical_cores() > 0);
    
    // Test that CPU info is reasonable on Windows
    assert!(!cpu_info.model_name().is_empty());
}

#[cfg(target_os = "linux")]
#[test] 
fn test_linux_platform_detection() {
    // Test basic Linux-specific hardware detection works
    let cpu_info = CPUInfo::query().expect("Failed to get Linux CPU info");
    assert!(cpu_info.physical_cores() > 0);
    
    // Test /proc filesystem access if available
    if std::path::Path::new("/proc/cpuinfo").exists() {
        println!("Linux /proc/cpuinfo is accessible");
    }
}

#[cfg(target_os = "macos")]
#[test]
fn test_macos_platform_detection() {
    // Test basic macOS-specific hardware detection works
    let cpu_info = CPUInfo::query().expect("Failed to get macOS CPU info");
    assert!(cpu_info.physical_cores() > 0);
    
    // Test sysctl command access if available
    use std::process::Command;
    if let Ok(output) = Command::new("sysctl")
        .arg("-n")
        .arg("hw.physicalcpu")
        .output()
    {
        if output.status.success() {
            println!("macOS sysctl command works");
        }
    }
}

// Test CPU feature detection across platforms
#[test]
fn test_cross_platform_cpu_features() {
    let cpu_info = CPUInfo::query().expect("Failed to query CPU info");
    
    // Test that features are detected
    assert!(!cpu_info.features.is_empty(), "CPU should have some features");
    
    // Test common feature checks
    let common_features = ["sse", "sse2", "sse3", "avx"];
    for feature in common_features {
        // Should not panic, even if feature is not present
        let _has_feature = cpu_info.has_feature(feature);
    }
}

// Test vendor detection across platforms
#[test]
fn test_cross_platform_vendor_detection() {
    let cpu_info = CPUInfo::query().expect("Failed to query CPU info");
    
    match cpu_info.vendor() {
        CPUVendor::Intel => assert!(!cpu_info.model_name().is_empty()),
        CPUVendor::AMD => assert!(!cpu_info.model_name().is_empty()),
        CPUVendor::ARM => assert!(!cpu_info.model_name().is_empty()),
        CPUVendor::Apple => assert!(!cpu_info.model_name().is_empty()),
        CPUVendor::Unknown(name) => assert!(!name.is_empty()),
    }
}
