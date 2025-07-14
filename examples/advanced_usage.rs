/*!
 * Advanced Hardware Query Example
 * 
 * This example demonstrates advanced usage patterns including:
 * - Error handling strategies
 * - Performance monitoring 
 * - Serialization and persistence
 * - Conditional compilation for different platforms
 * - Integration with async code
 */

use hardware_query::{
    HardwareQueryError,
    HardwareInfo,
};
use std::error::Error;
use std::fs;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("=== Advanced Hardware Query Demo ===\n");

    // Example 1: Performance monitoring
    demonstrate_performance_monitoring().await?;

    // Example 2: Error handling patterns
    demonstrate_error_handling().await?;

    // Example 3: Serialization and persistence
    demonstrate_serialization().await?;

    // Example 4: Platform-specific functionality
    demonstrate_platform_specific().await?;

    // Example 5: AI workload optimization
    demonstrate_ai_workload_optimization().await?;

    println!("=== Demo Complete ===");
    Ok(())
}

async fn demonstrate_performance_monitoring() -> Result<(), Box<dyn Error>> {
    println!("📊 Performance Monitoring Example");
    println!("=================================");

    let start = Instant::now();
    let hw_info = HardwareInfo::query()?;
    let query_duration = start.elapsed();

    println!("✅ Hardware query completed in: {query_duration:?}");
    println!("📈 System components detected:");
    println!("   - CPU cores: {}", hw_info.cpu().physical_cores());
    println!("   - GPUs: {}", hw_info.gpus().len());
    println!("   - Storage devices: {}", hw_info.storage_devices().len());
    println!("   - Network interfaces: {}", hw_info.network_interfaces().len());

    // Performance analysis
    if query_duration.as_millis() > 1000 {
        println!("⚠️  Hardware query took longer than expected. Consider caching results.");
    } else {
        println!("✅ Hardware query performance is optimal.");
    }

    println!();
    Ok(())
}

async fn demonstrate_error_handling() -> Result<(), Box<dyn Error>> {
    println!("🔧 Error Handling Patterns");
    println!("===========================");

    // Graceful degradation example
    match HardwareInfo::query() {
        Ok(hw_info) => {
            println!("✅ Full hardware information available");
            
            // Even with successful query, individual components might have limitations
            if hw_info.gpus().is_empty() {
                println!("⚠️  No GPUs detected - will fall back to CPU-only processing");
            }

            if hw_info.memory().total_gb() < 8.0 {
                println!("⚠️  Limited system memory - consider optimizing memory usage");
            }
        }
        Err(e) => {
            // Handle different error types appropriately
            match e {
                HardwareQueryError::PermissionDenied(msg) => {
                    println!("❌ Permission denied: {msg}");
                    println!("💡 Try running with elevated privileges");
                }
                HardwareQueryError::PlatformNotSupported(msg) => {
                    println!("❌ Platform not supported: {msg}");
                    println!("💡 This platform is not yet supported by the library");
                }
                _ => {
                    println!("❌ Unexpected error: {e}");
                    println!("💡 Consider filing a bug report");
                }
            }
        }
    }

    // Test invalid JSON handling
    let invalid_json = r#"{"invalid": "structure"}"#;
    match HardwareInfo::from_json(invalid_json) {
        Ok(_) => println!("❌ Should not have succeeded with invalid JSON"),
        Err(HardwareQueryError::SerializationError(_)) => {
            println!("✅ Correctly handled invalid JSON deserialization");
        }
        Err(e) => println!("❌ Unexpected error type: {e}"),
    }

    println!();
    Ok(())
}

async fn demonstrate_serialization() -> Result<(), Box<dyn Error>> {
    println!("💾 Serialization and Persistence");
    println!("=================================");

    let hw_info = HardwareInfo::query()?;

    // Serialize to JSON
    let json_data = hw_info.to_json()?;
    println!("✅ Serialized hardware info to JSON ({} bytes)", json_data.len());

    // Save to file
    let temp_file = "hardware_info.json";
    fs::write(temp_file, &json_data)?;
    println!("✅ Saved hardware info to '{temp_file}'");

    // Load from file and deserialize
    let loaded_json = fs::read_to_string(temp_file)?;
    let loaded_hw_info = HardwareInfo::from_json(&loaded_json)?;
    println!("✅ Loaded and deserialized hardware info from file");

    // Verify data integrity
    assert_eq!(hw_info.cpu().physical_cores(), loaded_hw_info.cpu().physical_cores());
    assert_eq!(hw_info.memory().total_mb, loaded_hw_info.memory().total_mb);
    println!("✅ Data integrity verified after roundtrip");

    // Cleanup
    fs::remove_file(temp_file).ok();

    // Pretty-printed JSON example
    let pretty_json = serde_json::to_string_pretty(&hw_info)?;
    println!("📄 Sample pretty-printed JSON (first 200 chars):");
    println!("{}", &pretty_json[..pretty_json.len().min(200)]);
    if pretty_json.len() > 200 {
        println!("... (truncated)");
    }

    println!();
    Ok(())
}

async fn demonstrate_platform_specific() -> Result<(), Box<dyn Error>> {
    println!("🖥️  Platform-Specific Functionality");
    println!("=====================================");

    let hw_info = HardwareInfo::query()?;

    // Platform-specific features
    #[cfg(target_os = "windows")]
    {
        println!("🪟 Windows-specific features:");
        println!("   - WMI-based hardware detection enabled");
        println!("   - DirectML acceleration available");
        
        // Check for Windows-specific GPU features
        for gpu in hw_info.gpus() {
            if gpu.supports_directml() {
                println!("   - DirectML support detected on {}", gpu.model_name());
            }
        }
    }

    #[cfg(target_os = "linux")]
    {
        println!("🐧 Linux-specific features:");
        println!("   - /proc and /sys filesystem detection enabled");
        println!("   - ROCm support available (if installed)");
        
        // Check for Linux-specific features
        for gpu in hw_info.gpus() {
            if gpu.supports_rocm() {
                println!("   - ROCm support detected on {}", gpu.model_name());
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        println!("🍎 macOS-specific features:");
        println!("   - Core Foundation and IOKit detection enabled");
        println!("   - Metal acceleration available");
        
        // Check for macOS-specific features
        for gpu in hw_info.gpus() {
            if gpu.supports_metal() {
                println!("   - Metal support detected on {}", gpu.model_name());
            }
        }
    }

    println!();
    Ok(())
}

async fn demonstrate_ai_workload_optimization() -> Result<(), Box<dyn Error>> {
    println!("🤖 AI Workload Optimization");
    println!("============================");

    let hw_info = HardwareInfo::query()?;

    println!("🔧 Specialized Hardware Analysis:");
    
    // NPUs
    let npus = hw_info.npus();
    if !npus.is_empty() {
        println!("   🧠 Neural Processing Units: {}", npus.len());
        for npu in npus {
            println!("      {} {} - {} TOPS", 
                npu.vendor, 
                npu.model_name,
                npu.tops_performance.unwrap_or(0.0)
            );
        }
    }
    
    // TPUs
    let tpus = hw_info.tpus();
    if !tpus.is_empty() {
        println!("   ⚡ Tensor Processing Units: {}", tpus.len());
        for tpu in tpus {
            println!("      {} {:?} - {:.1} TOPS", 
                tpu.vendor,
                tpu.architecture,
                tpu.tops_performance.unwrap_or(0.0)
            );
        }
    }
    
    // ARM hardware
    if let Some(arm) = hw_info.arm_hardware() {
        println!("   📱 ARM System: {}", arm.system_type);
    }
    
    // FPGAs
    let fpgas = hw_info.fpgas();
    if !fpgas.is_empty() {
        println!("   🔌 FPGAs: {}", fpgas.len());
        for fpga in fpgas {
            println!("      {} {} - {} logic elements",
                fpga.vendor,
                fpga.family,
                fpga.logic_elements.unwrap_or(0)
            );
        }
    }

    println!();
    Ok(())
}
