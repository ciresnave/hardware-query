use hardware_query::{
    CPUInfo, GPUInfo, HardwareInfo, MemoryInfo, StorageInfo,
};

#[test]
fn test_cpu_info_detection() {
    let cpu_info = CPUInfo::query().expect("Failed to query CPU info");

    // Basic validation
    assert!(
        cpu_info.physical_cores() > 0,
        "Physical cores should be > 0"
    );
    assert!(
        cpu_info.logical_cores() >= cpu_info.physical_cores(),
        "Logical cores should be >= physical cores"
    );

    // Test vendor detection
    let vendor = cpu_info.vendor();
    if let hardware_query::CPUVendor::Unknown(s) = vendor {
        assert!(!s.is_empty(), "Unknown CPU vendor should have a name")
    }

    // Test model name
    let model = cpu_info.model_name();
    assert!(!model.is_empty(), "CPU model name should not be empty");

    // Test feature detection
    assert!(
        !cpu_info.features.is_empty(),
        "CPU should have some features detected"
    );
}

#[test]
fn test_memory_info_detection() {
    let memory_info = MemoryInfo::query().expect("Failed to query memory info");

    // Basic validation
    assert!(memory_info.total_mb > 0, "Total memory should be > 0");
    assert!(
        memory_info.used_mb <= memory_info.total_mb,
        "Used memory should be <= total memory"
    );
    assert!(
        memory_info.available_mb <= memory_info.total_mb,
        "Available memory should be <= total memory"
    );

    // Test percentage calculation
    assert!(
        memory_info.usage_percent >= 0.0 && memory_info.usage_percent <= 100.0,
        "Memory usage percentage should be between 0 and 100"
    );
}

#[test]
fn test_gpu_detection() {
    let gpus = GPUInfo::query_all().expect("Failed to query GPU info");

    // We can't assume there's a GPU, but we can verify the result doesn't error
    // and that any found GPUs have valid properties
    for gpu in &gpus {
        assert!(
            !gpu.model_name.is_empty(),
            "GPU model name should not be empty"
        );
        assert!(gpu.memory_mb > 0, "GPU memory should be > 0");
    }
}

#[test]
fn test_storage_detection() {
    let storage_devices = StorageInfo::query_all().expect("Failed to query storage info");

    // Basic validation - should have at least one storage device
    assert!(
        !storage_devices.is_empty(),
        "Should detect at least one storage device"
    );

    for storage in &storage_devices {
        assert!(storage.capacity_gb > 0.0, "Storage capacity should be > 0");
        assert!(
            storage.used_gb <= storage.capacity_gb,
            "Used storage should be <= total capacity"
        );
        assert!(
            !storage.mount_point.is_empty(),
            "Mount point should not be empty"
        );
    }
}

#[test]
fn test_hardware_info_query() {
    // Test the main entry point
    let hw_info = HardwareInfo::query().expect("Failed to query hardware info");

    // Validate the structure
    assert!(hw_info.timestamp > 0, "Timestamp should be > 0");
    assert!(hw_info.cpu.physical_cores() > 0, "Should have CPU info");
}

#[test]
fn test_serialization() {
    let hw_info = HardwareInfo::query().expect("Failed to query hardware info");

    // Test serialization to JSON
    let json = hw_info.to_json().expect("Failed to serialize to JSON");
    assert!(!json.is_empty(), "JSON serialization should not be empty");

    // Test deserialization from JSON
    let deserialized = HardwareInfo::from_json(&json).expect("Failed to deserialize from JSON");
    assert_eq!(
        hw_info.cpu.physical_cores(),
        deserialized.cpu.physical_cores(),
        "Deserialized object should match original"
    );
}
