//! Additional comprehensive tests for edge cases and error conditions

use hardware_query::{
    HardwareQueryError,
    CPUInfo, GPUInfo, HardwareInfo, MemoryInfo, StorageInfo,
};

#[test]
fn test_serialization_roundtrip() {
    let hw_info = HardwareInfo::query().expect("Failed to query hardware info");

    // Test JSON serialization roundtrip
    let json = hw_info.to_json().expect("Failed to serialize");
    let deserialized = HardwareInfo::from_json(&json).expect("Failed to deserialize");

    // Compare key fields
    assert_eq!(hw_info.cpu.physical_cores(), deserialized.cpu.physical_cores());
    assert_eq!(hw_info.memory.total_mb, deserialized.memory.total_mb);
    assert_eq!(hw_info.gpus.len(), deserialized.gpus.len());
    assert_eq!(hw_info.storage_devices.len(), deserialized.storage_devices.len());
}

#[test]
fn test_invalid_json_deserialization() {
    let invalid_json = r#"{"invalid": "json", "structure": true}"#;
    let result = HardwareInfo::from_json(invalid_json);
    assert!(result.is_err());
    
    if let Err(e) = result {
        assert!(matches!(e, HardwareQueryError::SerializationError(_)));
    }
}

#[test]
fn test_cpu_feature_detection() {
    let cpu_info = CPUInfo::query().expect("Failed to query CPU info");
    
    // Test feature detection methods
    assert!(!cpu_info.features.is_empty());
    
    // Test has_feature method with various features
    let common_features = ["sse", "sse2", "sse3", "sse4.1", "sse4.2", "avx", "avx2"];
    for feature in common_features {
        // Should not panic, even if feature is not present
        let _has_feature = cpu_info.has_feature(feature);
    }
}

#[test]
fn test_gpu_capability_methods() {
    let gpus = GPUInfo::query_all().expect("Failed to query GPU info");
    
    for gpu in gpus {
        // Test GPU capability methods don't panic
        let _supports_cuda = gpu.supports_cuda();
        let _supports_rocm = gpu.supports_rocm();
        let _supports_directml = gpu.supports_directml();
        let _supports_opencl = gpu.supports_opencl();
        let _supports_vulkan = gpu.supports_vulkan();
        
        // Memory should be reasonable
        assert!(gpu.memory_mb() > 0);
        assert!(gpu.memory_gb() > 0.0);
        
        // Vendor and model should not be empty
        assert!(!gpu.model_name().is_empty());
    }
}

#[test]
fn test_memory_calculations() {
    let memory_info = MemoryInfo::query().expect("Failed to query memory info");
    
    // Test memory calculations are reasonably consistent (allow for system overhead)
    let total_calculated = memory_info.used_mb + memory_info.available_mb;
    let difference = (memory_info.total_mb as i64 - total_calculated as i64).abs();
    assert!(difference <= 10, "Memory calculation difference too large: {difference} MB"); // Allow up to 10MB difference for system overhead
    
    // Test GB conversions
    let total_gb = memory_info.total_gb();
    let used_gb = memory_info.used_gb();
    let available_gb = memory_info.available_gb();
    
    assert!((total_gb - (used_gb + available_gb)).abs() < 1.0); // Allow larger rounding errors for GB calculations
    
    // Test percentage calculation
    let expected_percentage = (memory_info.used_mb as f32 / memory_info.total_mb as f32) * 100.0;
    assert!((memory_info.usage_percent - expected_percentage).abs() < 1.0); // Allow 1% tolerance
}

#[test]
fn test_storage_consistency() {
    let storage_devices = StorageInfo::query_all().expect("Failed to query storage info");
    
    for storage in storage_devices {
        // Available + used should not exceed capacity (with small tolerance for filesystem overhead)
        assert!(storage.used_gb + storage.available_gb <= storage.capacity_gb + 1.0);
        
        // Usage percentage should be consistent
        if storage.capacity_gb > 0.0 {
            let expected_usage = (storage.used_gb / storage.capacity_gb) * 100.0;
            assert!((storage.usage_percent() - expected_usage).abs() < 1.0);
        }
        
        // Mount point should not be empty
        assert!(!storage.mount_point.is_empty());
    }
}

#[test]
fn test_hardware_info_timestamp() {
    let hw_info = HardwareInfo::query().expect("Failed to query hardware info");
    
    // Timestamp should be reasonable (within the last hour and not in the future)
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    assert!(hw_info.timestamp <= now);
    assert!(hw_info.timestamp > now - 3600); // Within the last hour
}
