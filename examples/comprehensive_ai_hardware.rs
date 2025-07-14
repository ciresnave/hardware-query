use hardware_query::{HardwareInfo, Result};

fn main() -> Result<()> {
    println!("🔍 Hardware Query - Comprehensive AI/ML Hardware Detection");
    println!("============================================================");
    
    // Query all hardware information
    let hw_info = HardwareInfo::query()?;
    
    // System Overview
    println!("\n🖥️  System Overview:");
    println!("   Timestamp: {}", hw_info.timestamp);
    
    // CPU Information
    let cpu = hw_info.cpu();
    println!("\n🧠 CPU Information:");
    println!("   Vendor: {}", cpu.vendor());
    println!("   Model: {}", cpu.model_name());
    println!("   Cores: {} physical, {} logical", 
             cpu.physical_cores(), cpu.logical_cores());
    println!("   Base Frequency: {:.2} GHz", cpu.base_frequency() as f64 / 1000.0);
    
    // Check for CPU AI features
    let cpu_ai_features = ["avx2", "avx512f", "fma", "f16c"];
    let available_features: Vec<_> = cpu_ai_features.iter()
        .filter(|&feature| cpu.has_feature(feature))
        .collect();
    if !available_features.is_empty() {
        println!("   AI Features: {}", available_features.iter().map(|s| s.to_string()).collect::<Vec<_>>().join(", "));
    }
    
    // GPU Information
    println!("\n🎮 GPU Information:");
    if hw_info.gpus().is_empty() {
        println!("   No dedicated GPUs detected");
    } else {
        for (i, gpu) in hw_info.gpus().iter().enumerate() {
            println!("   GPU {}: {} {} ({:.1} GB VRAM)",
                     i + 1,
                     gpu.vendor(),
                     gpu.model_name(),
                     gpu.memory_gb());
            
            // Display GPU type classification
            match gpu.gpu_type() {
                hardware_query::GPUType::Discrete => println!("     Type: Discrete"),
                hardware_query::GPUType::Integrated => println!("     Type: Integrated"),
                hardware_query::GPUType::Workstation => println!("     Type: Workstation"),
                hardware_query::GPUType::Datacenter => println!("     Type: Datacenter"),
                hardware_query::GPUType::Virtual => println!("     Type: Virtual"),
                hardware_query::GPUType::Unknown => println!("     Type: Unknown"),
            }
            
            // Check for AI acceleration support
            if gpu.supports_cuda() {
                println!("     ✅ CUDA Support");
            }
            if gpu.supports_opencl() {
                println!("     ✅ OpenCL Support");
            }
        }
    }
    
    // NPU Information (Neural Processing Units)
    println!("\n🧮 NPU Information:");
    if hw_info.npus().is_empty() {
        println!("   No NPUs detected");
    } else {
        for (i, npu) in hw_info.npus().iter().enumerate() {
            println!("   NPU {}: {} {} ({} TOPS)",
                     i + 1,
                     npu.vendor,
                     npu.model_name,
                     npu.tops_performance.unwrap_or(0.0));
            println!("     Architecture: {:?}", npu.architecture);
            println!("     Type: {:?}", npu.npu_type);
            
            if !npu.supported_frameworks.is_empty() {
                println!("     Frameworks: {}", npu.supported_frameworks.join(", "));
            }
        }
    }
    
    // TPU Information (Tensor Processing Units)
    println!("\n⚡ TPU Information:");
    if hw_info.tpus().is_empty() {
        println!("   No TPUs detected");
    } else {
        for (i, tpu) in hw_info.tpus().iter().enumerate() {
            println!("   TPU {}: {} {}",
                     i + 1,
                     tpu.vendor,
                     tpu.model_name);
            println!("     Architecture: {:?}", tpu.architecture);
            println!("     Connection: {:?}", tpu.connection_type);
            
            if let Some(tops) = tpu.tops_performance {
                println!("     Performance: {tops:.1} TOPS");
            }
            
            if !tpu.supported_frameworks.is_empty() {
                println!("     Frameworks: {}", tpu.supported_frameworks.join(", "));
            }
        }
    }
    
    // ARM Hardware Information
    println!("\n🤖 ARM Hardware Information:");
    if let Some(arm_hw) = hw_info.arm_hardware() {
        println!("   System Type: {}", arm_hw.system_type);
        println!("   Board Model: {}", arm_hw.board_model);
        println!("   CPU Architecture: {}", arm_hw.cpu_architecture);
        println!("   CPU Cores: {}", arm_hw.cpu_cores);
        
        if let Some(memory_mb) = arm_hw.memory_mb {
            println!("   Memory: {:.1} GB", memory_mb as f64 / 1024.0);
        }
        
        if let Some(gpu_info) = &arm_hw.gpu_info {
            println!("   GPU: {gpu_info}");
        }
        
        if !arm_hw.acceleration_features.is_empty() {
            println!("   Acceleration: {}", arm_hw.acceleration_features.join(", "));
        }
        
        if !arm_hw.ml_capabilities.is_empty() {
            println!("   ML Capabilities:");
            for (key, value) in &arm_hw.ml_capabilities {
                println!("     {key}: {value}");
            }
        }
    } else {
        println!("   Not running on ARM hardware");
    }
    
    // FPGA Information
    println!("\n🔧 FPGA Information:");
    if hw_info.fpgas().is_empty() {
        println!("   No FPGA accelerators detected");
    } else {
        for (i, fpga) in hw_info.fpgas().iter().enumerate() {
            println!("   FPGA {}: {} {} ({})",
                     i + 1,
                     fpga.vendor,
                     fpga.family,
                     fpga.model);
            println!("     Interface: {:?}", fpga.interface);
            
            if let Some(logic_elements) = fpga.logic_elements {
                println!("     Logic Elements: {logic_elements}");
            }
            
            if let Some(dsp_blocks) = fpga.dsp_blocks {
                println!("     DSP Blocks: {dsp_blocks}");
            }
            
            if let Some(freq) = fpga.max_frequency_mhz {
                println!("     Max Frequency: {freq} MHz");
            }
            
            if !fpga.development_tools.is_empty() {
                println!("     Tools: {}", fpga.development_tools.join(", "));
            }
            
            // Calculate AI performance metrics
            let ai_performance = fpga.calculate_ai_performance();
            if !ai_performance.is_empty() {
                println!("     AI Performance:");
                for (metric, value) in ai_performance {
                    println!("       {metric}: {value:.2e}");
                }
            }
            
            // Show framework support
            let frameworks = fpga.get_ai_framework_support();
            if !frameworks.is_empty() {
                println!("     AI Frameworks: {}", frameworks.join(", "));
            }
        }
    }
    
    // Memory Information
    println!("\n💾 Memory Information:");
    let memory = hw_info.memory();
    println!("   Total: {:.1} GB", memory.total_gb());
    println!("   Available: {:.1} GB", memory.available_gb());
    println!("   Usage: {:.1}%", (1.0 - memory.available_gb() / memory.total_gb()) * 100.0);
    
    // Accelerator Summary
    println!("\n🚀 Accelerator Summary:");
    let accelerator_summary = hw_info.accelerator_summary();
    if accelerator_summary.is_empty() {
        println!("   No specialized accelerators detected");
    } else {
        for (acc_type, count) in accelerator_summary {
            println!("   {acc_type}: {count}");
        }
        println!("   Total Accelerators: {}", hw_info.accelerator_count());
    }
    
    // System Summary
    println!("\n📊 System Summary:");
    let summary = hw_info.summary();
    println!("{summary}");
    
    println!("\n✨ Comprehensive hardware analysis complete!");
    
    Ok(())
}
