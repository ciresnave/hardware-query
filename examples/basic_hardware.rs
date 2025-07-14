use hardware_query::HardwareInfo;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hardware Query Demo");
    println!("==================");

    // Get complete system information
    let hw_info = HardwareInfo::query()?;

    // Display hardware summary
    println!("\n{}", hw_info.summary());

    // Access CPU information
    let cpu = hw_info.cpu();
    println!("\nCPU Details:");
    println!("  Vendor: {}", cpu.vendor());
    println!("  Model: {}", cpu.model_name());
    println!(
        "  Cores: {} physical, {} logical",
        cpu.physical_cores(),
        cpu.logical_cores()
    );
    println!("  Base Frequency: {} MHz", cpu.base_frequency());
    println!("  Architecture: {}", cpu.architecture());

    // Check for specific CPU features
    let features_to_check = ["avx", "avx2", "avx512", "sse", "sse2", "fma"];
    println!("  Features:");
    for feature in &features_to_check {
        if cpu.has_feature(feature) {
            println!("    ✓ {}", feature.to_uppercase());
        }
    }

    // Get GPU information
    println!("\nGPU Information:");
    if hw_info.gpus().is_empty() {
        println!("  No discrete GPUs detected");
    } else {
        for (i, gpu) in hw_info.gpus().iter().enumerate() {
            println!("  GPU {}: {} {}", i + 1, gpu.vendor(), gpu.model_name());
            println!("    VRAM: {:.1} GB", gpu.memory_gb());
            println!("    Type: {}", gpu.gpu_type());
            println!(
                "    CUDA: {}",
                if gpu.supports_cuda() { "Yes" } else { "No" }
            );
            println!(
                "    ROCm: {}",
                if gpu.supports_rocm() { "Yes" } else { "No" }
            );
            println!(
                "    DirectML: {}",
                if gpu.supports_directml() { "Yes" } else { "No" }
            );
            println!("    VRAM: {} GB", gpu.memory_gb());
        }
    }

    // Memory information
    let mem = hw_info.memory();
    println!("\nMemory Information:");
    println!("  Total: {:.1} GB", mem.total_gb());
    println!("  Available: {:.1} GB", mem.available_gb());
    println!(
        "  Used: {:.1} GB ({:.1}%)",
        mem.used_gb(),
        mem.usage_percent()
    );
    println!("  Speed: {} MHz", mem.speed_mhz());
    println!("  Channels: {}", mem.channels());
    println!(
        "  ECC Support: {}",
        if mem.ecc_support() { "Yes" } else { "No" }
    );

    // Storage information
    println!("\nStorage Devices:");
    for (i, disk) in hw_info.storage_devices().iter().enumerate() {
        println!("  Drive {}: {}", i + 1, disk.model());
        println!("    Type: {}", disk.drive_type());
        println!("    Capacity: {:.1} GB", disk.capacity_gb());
        println!(
            "    Available: {:.1} GB ({:.1}% used)",
            disk.available_gb(),
            disk.usage_percent()
        );
        println!("    Mount Point: {}", disk.mount_point);
    }

    // Network interfaces
    println!("\nNetwork Interfaces:");
    for interface in hw_info.network_interfaces() {
        if interface.is_active() {
            println!(
                "  {}: {} ({})",
                interface.name(),
                interface.network_type(),
                if interface.is_active() {
                    "Active"
                } else {
                    "Inactive"
                }
            );
        }
    }

    // Battery information (if available)
    if let Some(battery) = hw_info.battery() {
        println!("\nBattery Information:");
        println!("  Charge: {:.1}%", battery.percentage());
        println!("  Status: {}", battery.status());
        if let Some(time_remaining) = battery.time_remaining_hours() {
            println!("  Time Remaining: {time_remaining:.1} hours");
        }
        if let Some(health) = battery.health_percent() {
            println!("  Health: {health:.1}%");
        }
    }

    // Specialized hardware detection
    println!("\nSpecialized Hardware:");
    
    // Check for NPUs
    let npus = hw_info.npus();
    if !npus.is_empty() {
        println!("  NPUs:");
        for npu in npus {
            println!("    {} {} - {} TOPS", 
                npu.vendor, 
                npu.model_name,
                npu.tops_performance.unwrap_or(0.0)
            );
        }
    }
    
    // Check for TPUs
    let tpus = hw_info.tpus();
    if !tpus.is_empty() {
        println!("  TPUs:");
        for tpu in tpus {
            println!("    {} {:?} - {:.1} TOPS", 
                tpu.vendor,
                tpu.architecture,
                tpu.tops_performance.unwrap_or(0.0)
            );
        }
    }
    
    // Check for ARM-specific hardware
    if let Some(arm) = hw_info.arm_hardware() {
        println!("  ARM System: {}", arm.system_type);
        if let Some(power) = &arm.power_info {
            if let Some(consumption) = power.power_consumption {
                println!("    Power consumption: {consumption:.1}W");
            }
            if let Some(voltage) = power.voltage {
                println!("    Voltage: {voltage:.1}V");
            }
        }
    }
    
    // Check for FPGAs
    let fpgas = hw_info.fpgas();
    if !fpgas.is_empty() {
        println!("  FPGAs:");
        for fpga in fpgas {
            println!("    {} {} - {} logic elements",
                fpga.vendor,
                fpga.family,
                fpga.logic_elements.unwrap_or(0)
            );
        }
    }

    println!("\n{}", "=".repeat(50));
    println!("Hardware analysis complete!");

    Ok(())
}
