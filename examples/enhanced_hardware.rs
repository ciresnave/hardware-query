// Enhanced hardware query example demonstrating advanced AI capabilities
use hardware_query::HardwareInfo;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Enhanced Hardware Query Demo");
    println!("===========================");

    // Query hardware information
    let hw_info = HardwareInfo::query()?;

    // Hardware Summary
    println!("Hardware Summary:");
    println!(
        "  CPU: {} {} ({} cores, {} threads)",
        hw_info.cpu().vendor(),
        hw_info.cpu().model_name(),
        hw_info.cpu().physical_cores(),
        hw_info.cpu().logical_cores()
    );

    println!("  Memory: {:.1} GB", hw_info.memory().total_gb());

    if !hw_info.gpus().is_empty() {
        let gpu = &hw_info.gpus()[0];
        println!(
            "  Primary GPU: {} {} ({:.1} GB)",
            gpu.vendor(),
            gpu.model_name(),
            gpu.memory_gb()
        );
    }

    if !hw_info.storage_devices().is_empty() {
        let total_storage: f64 = hw_info
            .storage_devices()
            .iter()
            .map(|d| d.capacity_gb())
            .sum();
        println!("  Total Storage: {total_storage:.1} GB");
    }

    // Specialized Hardware Analysis
    println!("\n🔧 Specialized Hardware Analysis");
    println!("===============================");

    // NPUs
    let npus = hw_info.npus();
    if !npus.is_empty() {
        println!("🧠 Neural Processing Units:");
        for npu in npus {
            println!("  {} {} - {} TOPS", 
                npu.vendor, 
                npu.model_name,
                npu.tops_performance.unwrap_or(0.0)
            );
        }
    }
    
    // TPUs
    let tpus = hw_info.tpus();
    if !tpus.is_empty() {
        println!("⚡ Tensor Processing Units:");
        for tpu in tpus {
            println!("  {} {:?} - {:.1} TOPS", 
                tpu.vendor,
                tpu.architecture,
                tpu.tops_performance.unwrap_or(0.0)
            );
        }
    }
    
    // ARM hardware
    if let Some(arm) = hw_info.arm_hardware() {
        println!("📱 ARM System: {}", arm.system_type);
    }
    
    // FPGAs
    let fpgas = hw_info.fpgas();
    if !fpgas.is_empty() {
        println!("🔌 Field-Programmable Gate Arrays:");
        for fpga in fpgas {
            println!("  {} {} - {} logic elements",
                fpga.vendor,
                fpga.family,
                fpga.logic_elements.unwrap_or(0)
            );
        }
    }

    // Hardware summary
    println!("\n📋 System Summary:");
    let summary = hw_info.summary();
    println!("{summary}");

    println!("\n==================================================");
    println!("Enhanced hardware analysis complete!");

    Ok(())
}
