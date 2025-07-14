use hardware_query::HardwareInfo;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Query hardware information
    println!("Querying hardware information...");
    let hw_info = HardwareInfo::query()?;

    // Display CPU information
    println!("\n--- CPU Information ---");
    let cpu = hw_info.cpu();
    println!(
        "CPU: {} {} with {} cores, {} threads",
        cpu.vendor(),
        cpu.model_name(),
        cpu.physical_cores(),
        cpu.logical_cores()
    );
    println!("Base Frequency: {} MHz", cpu.base_frequency);
    println!("Max Frequency: {} MHz", cpu.max_frequency);

    if !cpu.features.is_empty() {
        println!("\nCPU Features:");
        for feature in cpu.features.iter().take(10) {
            println!("  - {feature}");
        }
        if cpu.features.len() > 10 {
            println!("  ... and {} more", cpu.features.len() - 10);
        }
    }

    // Display GPU information
    println!("\n--- GPU Information ---");
    if hw_info.gpus.is_empty() {
        println!("No GPUs detected");
    } else {
        for (i, gpu) in hw_info.gpus.iter().enumerate() {
            println!("\nGPU #{}: {} {}", i + 1, gpu.vendor(), gpu.model_name());
            println!("Memory: {} GB", gpu.memory_gb());
            if let Some(driver) = &gpu.driver_version {
                println!("Driver Version: {driver}");
            }

            // Display compute capabilities
            if gpu.compute_capabilities.cuda.is_some() {
                println!(
                    "CUDA Support: Yes (Compute Capability {})",
                    gpu.compute_capabilities
                        .cuda
                        .as_ref()
                        .unwrap_or(&"Unknown".to_string())
                );
            } else if gpu.compute_capabilities.opencl {
                println!("OpenCL Support: Yes");
            }

            if let Some(temp) = gpu.temperature {
                println!("Temperature: {temp}°C");
            }
        }
    }

    // Display Memory information
    println!("\n--- Memory Information ---");
    println!(
        "Total Memory: {:.2} GB",
        hw_info.memory.total_mb as f64 / 1024.0
    );
    println!(
        "Available Memory: {:.2} GB",
        hw_info.memory.available_mb as f64 / 1024.0
    );
    println!(
        "Used Memory: {:.2} GB",
        hw_info.memory.used_mb as f64 / 1024.0
    );
    println!("Memory Usage: {}%", hw_info.memory.usage_percent as u32);

    // Display Storage information
    println!("\n--- Storage Information ---");
    for (i, storage) in hw_info.storage_devices.iter().enumerate() {
        println!("\nStorage Device #{}: {}", i + 1, storage.model);
        println!("Type: {:?}", storage.storage_type);
        println!("Capacity: {:.2} GB", storage.capacity_gb);
        println!("Available: {:.2} GB", storage.available_gb);
        println!("Used: {:.2} GB", storage.used_gb);
        println!("Mount Point: {}", storage.mount_point);
    }

    // Display specialized hardware
    println!("\n--- Specialized Hardware ---");
    
    // NPUs
    let npus = hw_info.npus();
    if !npus.is_empty() {
        println!("NPUs found: {}", npus.len());
        for (i, npu) in npus.iter().enumerate() {
            println!("  {}. {} {} - {} TOPS", 
                i + 1,
                npu.vendor, 
                npu.model_name,
                npu.tops_performance.unwrap_or(0.0)
            );
        }
    } else {
        println!("No NPUs detected");
    }
    
    // TPUs
    let tpus = hw_info.tpus();
    if !tpus.is_empty() {
        println!("TPUs found: {}", tpus.len());
        for (i, tpu) in tpus.iter().enumerate() {
            println!("  {}. {} {:?} - {:.1} TOPS", 
                i + 1,
                tpu.vendor,
                tpu.architecture,
                tpu.tops_performance.unwrap_or(0.0)
            );
        }
    } else {
        println!("No TPUs detected");
    }
    
    // ARM hardware
    if let Some(arm) = hw_info.arm_hardware() {
        println!("ARM System: {}", arm.system_type);
    }
    
    // FPGAs
    let fpgas = hw_info.fpgas();
    if !fpgas.is_empty() {
        println!("FPGAs found: {}", fpgas.len());
        for (i, fpga) in fpgas.iter().enumerate() {
            println!("  {}. {} {} - {} logic elements",
                i + 1,
                fpga.vendor,
                fpga.family,
                fpga.logic_elements.unwrap_or(0)
            );
        }
    } else {
        println!("No FPGAs detected");
    }

    // Display hardware summary
    println!("\n--- Hardware Summary ---");
    println!("{}", hw_info.summary());

    Ok(())
}
