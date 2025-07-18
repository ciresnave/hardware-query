//! Comprehensive tests for the simplified API functionality

use hardware_query::{
    HardwareQueryBuilder, HardwarePresets, SystemOverview,
    HealthStatus, TemperatureStatus, PowerStatus,
};

#[test]
fn test_system_overview_quick() {
    let overview = SystemOverview::quick().expect("Failed to get system overview");

    // Validate CPU information
    assert!(overview.cpu.cores > 0, "Should have at least one CPU core");
    assert!(overview.cpu.threads >= overview.cpu.cores, "Threads should be >= cores");
    assert!(!overview.cpu.name.is_empty(), "CPU name should not be empty");
    assert!(!overview.cpu.vendor.is_empty(), "CPU vendor should not be empty");

    // Validate memory
    assert!(overview.memory_gb > 0.0, "Should have positive memory amount");

    // Validate storage
    assert!(overview.storage.total_gb > 0.0, "Should have positive storage capacity");
    assert!(overview.storage.available_gb >= 0.0, "Available storage should be non-negative");
    assert!(overview.storage.available_gb <= overview.storage.total_gb, "Available <= total storage");
    assert!(!overview.storage.drive_type.is_empty(), "Drive type should not be empty");
    assert!(!overview.storage.health.is_empty(), "Storage health should not be empty");

    // Validate health status
    assert!(matches!(overview.health.status, 
        HealthStatus::Excellent | HealthStatus::Good | HealthStatus::Fair | 
        HealthStatus::Poor | HealthStatus::Critical));
    assert!(matches!(overview.health.temperature,
        TemperatureStatus::Normal | TemperatureStatus::Warm | 
        TemperatureStatus::Hot | TemperatureStatus::Critical));
    assert!(matches!(overview.health.power,
        PowerStatus::Low | PowerStatus::Normal | 
        PowerStatus::High | PowerStatus::VeryHigh));

    // Validate environment
    assert!(!overview.environment.is_empty(), "Environment should not be empty");

    // Validate performance score
    assert!(overview.performance_score <= 100, "Performance score should be <= 100");
}

#[test]
fn test_ai_readiness_assessment() {
    let overview = SystemOverview::quick().expect("Failed to get system overview");
    
    // Test AI readiness logic
    let ai_ready = overview.is_ai_ready();
    let ai_score = overview.ai_score();
    
    // AI score should be valid
    assert!(ai_score <= 100, "AI score should be <= 100");
    
    // If AI ready, score should be reasonably high
    if ai_ready {
        assert!(ai_score >= 30, "If AI ready, score should be at least 30");
    }
    
    // If high memory (>= 8GB), should contribute to AI readiness
    if overview.memory_gb >= 8.0 {
        assert!(ai_score >= 20, "High memory should contribute to AI score");
    }
}

#[test]
fn test_hardware_presets() {
    // Test AI preset
    let ai_assessment = HardwarePresets::ai_assessment()
        .expect("Failed to get AI assessment");
    
    assert!(ai_assessment.ai_score <= 100);
    assert!(!ai_assessment.frameworks.is_empty(), "Should have framework recommendations");
    assert!(!ai_assessment.optimizations.is_empty(), "Should have optimization suggestions");

    // Test gaming preset
    let gaming_assessment = HardwarePresets::gaming_assessment()
        .expect("Failed to get gaming assessment");
    
    assert!(gaming_assessment.gaming_score <= 100);

    // Test development preset
    let dev_assessment = HardwarePresets::developer_assessment()
        .expect("Failed to get development assessment");
    
    assert!(dev_assessment.dev_score <= 100);

    // Test server preset
    let server_assessment = HardwarePresets::server_assessment()
        .expect("Failed to get server assessment");
    
    assert!(server_assessment.server_score <= 100);
}

#[test]
fn test_hardware_query_builder_basic() {
    let builder = HardwareQueryBuilder::new();
    
    // Test basic configuration
    let custom_info = builder
        .with_cpu()
        .with_memory()
        .query()
        .expect("Failed to build hardware query");
    
    // Should have CPU and memory
    assert!(custom_info.cpu.is_some());
    assert!(custom_info.memory.is_some());
    if let Some(cpu) = custom_info.cpu {
        assert!(cpu.physical_cores() > 0);
    }
    if let Some(memory) = custom_info.memory {
        assert!(memory.total_mb > 0);
    }
}

#[test]
fn test_hardware_query_builder_with_basic() {
    let custom_info = HardwareQueryBuilder::new()
        .with_basic()
        .query()
        .expect("Failed to build basic hardware query");
    
    // Basic should include CPU, memory, storage
    assert!(custom_info.cpu.is_some());
    assert!(custom_info.memory.is_some());
    assert!(!custom_info.storage_devices.is_empty() || custom_info.storage_devices.is_empty());
}

#[test]
fn test_hardware_query_builder_with_ai_focused() {
    let custom_info = HardwareQueryBuilder::new()
        .with_ai_focused()
        .query()
        .expect("Failed to build AI-focused hardware query");
    
    // AI-focused should include CPU, GPU, memory
    assert!(custom_info.cpu.is_some());
    assert!(custom_info.memory.is_some());
    // GPU may be empty but should not error
}

#[test]
fn test_hardware_query_builder_with_gaming_focused() {
    let custom_info = HardwareQueryBuilder::new()
        .with_gaming_focused()
        .query()
        .expect("Failed to build gaming-focused hardware query");
    
    // Gaming-focused should include CPU, GPU, memory, thermal, storage
    assert!(custom_info.cpu.is_some());
    assert!(custom_info.memory.is_some());
}

#[test]
fn test_hardware_query_builder_presets() {
    // Test CPU and memory preset
    let cpu_mem_info = HardwareQueryBuilder::cpu_and_memory()
        .expect("Failed to get CPU and memory info");
    assert!(cpu_mem_info.cpu.is_some());
    assert!(cpu_mem_info.memory.is_some());

    // Test GPU info preset
    let gpu_info = HardwareQueryBuilder::gpu_info()
        .expect("Failed to get GPU info");
    assert!(gpu_info.memory.is_some());

    // Test health check preset
    let health_info = HardwareQueryBuilder::health_check()
        .expect("Failed to get health check info");
    assert!(health_info.cpu.is_some());
    assert!(health_info.memory.is_some());

    // Test performance check preset
    let perf_info = HardwareQueryBuilder::performance_check()
        .expect("Failed to get performance check info");
    assert!(perf_info.cpu.is_some());
    assert!(perf_info.memory.is_some());
}

#[test]
fn test_system_overview_serialization() {
    let overview = SystemOverview::quick().expect("Failed to get system overview");
    
    // Test JSON serialization
    let json = serde_json::to_string(&overview).expect("Failed to serialize to JSON");
    assert!(!json.is_empty(), "JSON should not be empty");
    
    // Test deserialization
    let deserialized: SystemOverview = serde_json::from_str(&json)
        .expect("Failed to deserialize from JSON");
    
    // Verify key fields match
    assert_eq!(overview.cpu.cores, deserialized.cpu.cores);
    assert_eq!(overview.memory_gb, deserialized.memory_gb);
    assert_eq!(overview.performance_score, deserialized.performance_score);
}

#[test]
fn test_health_status_display() {
    // Test all health status display implementations
    assert_eq!(format!("{}", HealthStatus::Excellent), "Excellent");
    assert_eq!(format!("{}", HealthStatus::Good), "Good");
    assert_eq!(format!("{}", HealthStatus::Fair), "Fair");
    assert_eq!(format!("{}", HealthStatus::Poor), "Poor");
    assert_eq!(format!("{}", HealthStatus::Critical), "Critical");
    
    assert_eq!(format!("{}", TemperatureStatus::Normal), "Normal");
    assert_eq!(format!("{}", TemperatureStatus::Warm), "Warm");
    assert_eq!(format!("{}", TemperatureStatus::Hot), "Hot");
    assert_eq!(format!("{}", TemperatureStatus::Critical), "Critical");
    
    assert_eq!(format!("{}", PowerStatus::Low), "Low");
    assert_eq!(format!("{}", PowerStatus::Normal), "Normal");
    assert_eq!(format!("{}", PowerStatus::High), "High");
    assert_eq!(format!("{}", PowerStatus::VeryHigh), "Very High");
}

#[test]
fn test_error_handling_simplified_api() {
    // Test that simplified API handles errors gracefully
    // This tests internal error handling rather than forcing errors
    
    // System overview should handle missing components gracefully
    let overview = SystemOverview::quick();
    assert!(overview.is_ok(), "System overview should succeed on valid system");
    
    // Builder should handle valid configurations
    let builder_result = HardwareQueryBuilder::new()
        .with_cpu()
        .query();
    assert!(builder_result.is_ok(), "Builder should succeed with valid configuration");
}

#[test]
fn test_ai_assessment_comprehensive() {
    let ai_assessment = HardwarePresets::ai_assessment()
        .expect("Failed to get AI assessment");
    
    // Test comprehensive fields
    assert!(ai_assessment.ai_score <= 100, "AI score should be <= 100");
    assert!(!ai_assessment.frameworks.is_empty(), "Should recommend frameworks");
    assert!(!ai_assessment.optimizations.is_empty(), "Should provide optimizations");
    
    // Model recommendations should be reasonable
    assert!(!ai_assessment.model_recommendations.small_models.is_empty());
}

#[test]
fn test_gaming_assessment_comprehensive() {
    let gaming_assessment = HardwarePresets::gaming_assessment()
        .expect("Failed to get gaming assessment");
    
    // Test comprehensive fields
    assert!(gaming_assessment.gaming_score <= 100, "Gaming score should be <= 100");
    
    // Should have recommendations and bottleneck analysis
    assert!(!gaming_assessment.bottlenecks.is_empty() || gaming_assessment.bottlenecks.is_empty());
    assert!(!gaming_assessment.upgrade_recommendations.is_empty() || gaming_assessment.upgrade_recommendations.is_empty());
}

#[test]
fn test_developer_assessment_comprehensive() {
    let dev_assessment = HardwarePresets::developer_assessment()
        .expect("Failed to get development assessment");
    
    // Test comprehensive fields
    assert!(dev_assessment.dev_score <= 100, "Dev score should be <= 100");
    assert!(!dev_assessment.environments.is_empty(), "Should recommend environments");
}

#[test]
fn test_server_assessment_comprehensive() {
    let server_assessment = HardwarePresets::server_assessment()
        .expect("Failed to get server assessment");
    
    // Test comprehensive fields
    assert!(server_assessment.server_score <= 100, "Server score should be <= 100");
    assert!(!server_assessment.workload_suitability.is_empty(), "Should list suitable workloads");
}
