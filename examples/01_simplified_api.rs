//! Simple Hardware Query Example
//!
//! This example demonstrates the simplified API that makes hardware-query
//! easy to use for common scenarios.

use hardware_query::{
    SystemOverview, HardwarePresets, HardwareQueryBuilder,
    Result, HardwareQueryError
};

fn main() -> Result<()> {
    println!("=== Hardware Query - Simplified API Examples ===\n");

    // 1. Quick system overview - fastest and simplest
    println!("1. Quick System Overview:");
    let overview = SystemOverview::quick()?;
    println!("   CPU: {} ({} cores)", overview.cpu.name, overview.cpu.cores);
    println!("   Memory: {:.1} GB", overview.memory_gb);
    if let Some(gpu) = &overview.gpu {
        println!("   GPU: {} ({:.1} GB VRAM)", gpu.name, gpu.vram_gb);
    } else {
        println!("   GPU: None (integrated graphics)");
    }
    println!("   Storage: {:.0} GB {} (Health: {})", 
        overview.storage.total_gb, overview.storage.drive_type, overview.storage.health);
    println!("   Environment: {}", overview.environment);
    println!("   Performance Score: {}/100", overview.performance_score);
    println!("   AI Ready: {}", if overview.is_ai_ready() { "Yes" } else { "No" });
    println!();

    // 2. AI/ML Assessment
    println!("2. AI/ML Hardware Assessment:");
    let ai_assessment = HardwarePresets::ai_assessment()?;
    println!("   AI Score: {}/100", ai_assessment.ai_score);
    println!("   Training Capability: {:?}", ai_assessment.performance.training_capability);
    println!("   Inference Capability: {:?}", ai_assessment.performance.inference_capability);
    
    println!("   Recommended Frameworks:");
    for framework in &ai_assessment.frameworks {
        println!("     - {}: {:?} (Performance: {:?})",
            framework.name, framework.compatibility, framework.performance_estimate);
    }
    
    if !ai_assessment.optimizations.is_empty() {
        println!("   Optimization Suggestions:");
        for optimization in &ai_assessment.optimizations {
            println!("     - {}", optimization);
        }
    }
    println!();

    // 3. Gaming Assessment
    println!("3. Gaming Hardware Assessment:");
    let gaming_assessment = HardwarePresets::gaming_assessment()?;
    println!("   Gaming Score: {}/100", gaming_assessment.gaming_score);
    println!("   Recommended Settings:");
    println!("     Resolution: {}", gaming_assessment.recommended_settings.resolution);
    println!("     Quality: {:?}", gaming_assessment.recommended_settings.quality_preset);
    println!("     Target FPS: {}", gaming_assessment.recommended_settings.target_fps);
    println!("     Ray Tracing: {}", 
        if gaming_assessment.recommended_settings.raytracing_support { "Supported" } else { "Not Supported" });
    
    if !gaming_assessment.bottlenecks.is_empty() {
        println!("   Performance Bottlenecks:");
        for bottleneck in &gaming_assessment.bottlenecks {
            println!("     - {}", bottleneck);
        }
    }
    println!();

    // 4. Developer Assessment
    println!("4. Developer Workstation Assessment:");
    let dev_assessment = HardwarePresets::developer_assessment()?;
    println!("   Development Score: {}/100", dev_assessment.dev_score);
    println!("   Virtualization Support:");
    println!("     Hardware Acceleration: {}", 
        if dev_assessment.virtualization_support.hardware_acceleration { "Yes" } else { "No" });
    println!("     Max Recommended VMs: {}", dev_assessment.virtualization_support.max_recommended_vms);
    println!("     Docker Performance: {:?}", dev_assessment.virtualization_support.docker_performance);
    
    println!("   Recommended Development Environments:");
    for env in &dev_assessment.environments {
        println!("     - {}: {:?} ({})", env.name, env.suitability, env.recommended_config);
    }
    println!();

    // 5. Custom Query Examples
    println!("5. Custom Hardware Queries:");
    
    // CPU and Memory only
    let cpu_memory = HardwareQueryBuilder::cpu_and_memory()?;
    println!("   Query Summary: {}", cpu_memory.query_summary());
    
    // AI-focused query
    let ai_focused = HardwareQueryBuilder::new()
        .with_ai_focused()
        .query()?;
    println!("   AI-focused query: {}", ai_focused.query_summary());
    
    // Gaming-focused query with GPU filtering
    let gaming_focused = HardwareQueryBuilder::new()
        .with_gaming_focused()
        .filter_gpus(|gpu| gpu.memory_gb() >= 4.0) // Only GPUs with 4GB+ VRAM
        .query()?;
    println!("   Gaming query (4GB+ VRAM): {} GPUs found", gaming_focused.gpus.len());
    println!();

    // 6. Quick Compatibility Checks
    println!("6. Quick Compatibility Checks:");
    
    // Check specific AI model compatibility
    let can_run_gpt35 = HardwarePresets::check_ai_model_compatibility(
        "GPT-3.5", "175B parameters", 8.0
    )?;
    println!("   Can run GPT-3.5 (8GB VRAM): {}", if can_run_gpt35 { "Yes" } else { "No" });
    
    // Gaming performance estimate
    let fps_1080p_high = HardwarePresets::gaming_fps_estimate("1080p", "High")?;
    println!("   Estimated FPS (1080p High): {} FPS", fps_1080p_high);
    
    let fps_4k_ultra = HardwarePresets::gaming_fps_estimate("4K", "Ultra")?;
    println!("   Estimated FPS (4K Ultra): {} FPS", fps_4k_ultra);
    println!();

    // 7. System Health and Recommendations
    println!("7. System Health & Recommendations:");
    println!("   Overall Health: {}", overview.health.status);
    println!("   Temperature: {}", overview.health.temperature);
    println!("   Power Consumption: {}", overview.health.power);
    
    let recommendations = overview.get_recommendations();
    if !recommendations.is_empty() {
        println!("   Recommendations:");
        for rec in &recommendations {
            println!("     - {}", rec);
        }
    } else {
        println!("   No specific recommendations - system is performing well!");
    }
    println!();

    // 8. Specialized Hardware Detection
    println!("8. Specialized Hardware:");
    let full_query = HardwareQueryBuilder::new().with_all().query()?;
    
    if full_query.has_component("NPU") || full_query.has_component("TPU") {
        println!("   AI Accelerators detected!");
    }
    
    if let Some(virt) = &full_query.virtualization {
        if virt.is_containerized() {
            println!("   Running in container: {:?}", virt.container_runtime);
        }
        if virt.is_virtual_machine() {
            println!("   Running in virtual machine: {:?}", virt.environment_type);
        }
    }

    println!("\n=== Summary ===");
    println!("The simplified API provides three main entry points:");
    println!("1. SystemOverview::quick() - Fastest overview of your system");
    println!("2. HardwarePresets::*_assessment() - Detailed analysis for specific use cases");
    println!("3. HardwareQueryBuilder - Custom queries for advanced users");
    println!("\nAll APIs are designed to be intuitive and require minimal hardware knowledge!");

    Ok(())
}

// Helper function to demonstrate error handling
fn demonstrate_error_handling() {
    match SystemOverview::quick() {
        Ok(overview) => {
            println!("System detected successfully!");
            println!("Performance score: {}", overview.performance_score);
        }
        Err(HardwareQueryError::PlatformNotSupported(_)) => {
            eprintln!("Error: Your platform is not supported yet");
        }
        Err(HardwareQueryError::PermissionDenied(_)) => {
            eprintln!("Error: Insufficient permissions to query hardware");
        }
        Err(e) => {
            eprintln!("Hardware query failed: {}", e);
        }
    }
}
