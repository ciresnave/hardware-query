use hardware_query::{
    HardwareInfo, Result, 
};

#[cfg(feature = "monitoring")]
use hardware_query::{HardwareMonitor, MonitoringConfig, MonitoringEvent};

#[tokio::main]
async fn main() -> Result<()> {
    println!("=== Enhanced Hardware Query Demo ===\n");

    // Query basic hardware information
    let hw_info = HardwareInfo::query()?;
    
    println!("🖥️  System Overview:");
    println!("   CPU: {} {}", hw_info.cpu().vendor(), hw_info.cpu().model_name());
    println!("   Memory: {:.1} GB", hw_info.memory().total_gb());
    println!("   GPUs: {} detected", hw_info.gpus().len());
    
    // Virtualization detection
    println!("\n🏗️  Virtualization Environment:");
    let virt_info = hw_info.virtualization();
    println!("   Environment: {}", virt_info.environment_type);
    println!("   Is Virtualized: {}", virt_info.is_virtualized());
    println!("   Performance Impact: {:.1}%", (1.0 - virt_info.get_performance_factor()) * 100.0);
    
    if virt_info.is_containerized() {
        println!("   Running in container: {}", virt_info.environment_type);
        if let Some(runtime) = &virt_info.container_runtime {
            println!("   Container Runtime: {}", runtime.name);
        }
    }
    
    if virt_info.has_gpu_access() {
        println!("   GPU Access: Available");
    } else {
        println!("   GPU Access: Restricted");
    }

    // Power management
    println!("\n⚡ Power Management:");
    if let Some(power_profile) = hw_info.power_profile() {
        println!("   Power State: {}", power_profile.power_state);
        
        if let Some(total_power) = power_profile.total_power_draw {
            println!("   Total Power Draw: {:.1} W", total_power);
            
            // Battery life estimation
            if let Some(battery) = hw_info.battery() {
                if let Some(remaining_time) = power_profile.estimate_battery_life(battery) {
                    let hours = remaining_time.as_secs() / 3600;
                    let minutes = (remaining_time.as_secs() % 3600) / 60;
                    println!("   Estimated Battery Life: {}h {}m", hours, minutes);
                }
            }
        }
        
        println!("   Efficiency Score: {:.1}/10", power_profile.efficiency_score * 10.0);
        println!("   Thermal Throttling Risk: {}", power_profile.thermal_throttling_risk);
        
        // Power optimization recommendations
        let optimizations = power_profile.suggest_power_optimizations();
        if !optimizations.is_empty() {
            println!("   Power Optimization Recommendations:");
            for opt in optimizations.iter().take(3) {
                println!("     • {}", opt.recommendation);
                if let Some(savings) = opt.expected_savings_watts {
                    println!("       (Expected savings: {:.1}W)", savings);
                }
            }
        }
    } else {
        println!("   Power information not available");
    }

    // Enhanced thermal monitoring
    println!("\n🌡️  Thermal Management:");
    let thermal_info = hw_info.thermal();
    
    if let Some(max_temp) = thermal_info.max_temperature() {
        println!("   Maximum Temperature: {:.1}°C", max_temp);
        println!("   Thermal Status: {}", thermal_info.thermal_status());
        
        // Thermal throttling prediction
        let throttling_prediction = thermal_info.predict_thermal_throttling(1.0);
        println!("   Throttling Risk: {}", throttling_prediction.severity);
        
        if throttling_prediction.will_throttle {
            if let Some(time_to_throttle) = throttling_prediction.time_to_throttle {
                println!("   Time to Throttling: {}s", time_to_throttle.as_secs());
            }
            
            if !throttling_prediction.recommendations.is_empty() {
                println!("   Recommendations:");
                for rec in &throttling_prediction.recommendations {
                    println!("     • {}", rec);
                }
            }
        }
        
        // Cooling recommendations
        let cooling_recs = thermal_info.suggest_cooling_optimizations();
        if !cooling_recs.is_empty() {
            println!("   Cooling Optimizations:");
            for rec in cooling_recs.iter().take(2) {
                println!("     • {} ({})", rec.description, rec.cost_category);
                if let Some(temp_reduction) = rec.expected_temp_reduction {
                    println!("       Expected temp reduction: {:.1}°C", temp_reduction);
                }
            }
        }
        
        // Sustained performance capability
        let sustained_perf = thermal_info.calculate_sustained_performance();
        println!("   Sustained Performance: {:.1}%", sustained_perf * 100.0);
    }
    
    // CPU and GPU temperatures
    if let Some(cpu_temp) = thermal_info.cpu_temperature() {
        println!("   CPU Temperature: {:.1}°C", cpu_temp);
    }
    
    if let Some(gpu_temp) = thermal_info.gpu_temperature() {
        println!("   GPU Temperature: {:.1}°C", gpu_temp);
    }

    // AI/ML readiness assessment
    println!("\n🤖 AI/ML Readiness:");
    assess_ai_readiness(&hw_info);

    // Real-time monitoring demo (if feature is enabled)
    #[cfg(feature = "monitoring")]
    {
        println!("\n📊 Real-time Monitoring Demo:");
        demo_monitoring().await?;
    }

    #[cfg(not(feature = "monitoring"))]
    {
        println!("\n📊 Real-time monitoring is available with the 'monitoring' feature");
    }

    Ok(())
}

fn assess_ai_readiness(hw_info: &HardwareInfo) {
    let mut score = 0;
    let mut recommendations = Vec::new();

    // GPU assessment
    if !hw_info.gpus().is_empty() {
        score += 30;
        let total_vram: f64 = hw_info.gpus().iter()
            .map(|gpu| gpu.memory_gb())
            .sum();
        
        println!("   GPU Acceleration: Available ({:.1} GB VRAM)", total_vram);
        
        if total_vram >= 8.0 {
            score += 20;
            println!("   VRAM Assessment: Excellent for large models");
        } else if total_vram >= 4.0 {
            score += 10;
            println!("   VRAM Assessment: Good for medium models");
            recommendations.push("Consider GPU upgrade for larger models");
        } else {
            println!("   VRAM Assessment: Limited to small models");
            recommendations.push("GPU upgrade recommended for better AI performance");
        }
    } else {
        println!("   GPU Acceleration: Not available");
        recommendations.push("Dedicated GPU highly recommended for AI workloads");
    }

    // CPU assessment
    let cpu_cores = hw_info.cpu().logical_cores();
    if cpu_cores >= 16 {
        score += 20;
        println!("   CPU Cores: Excellent ({} cores)", cpu_cores);
    } else if cpu_cores >= 8 {
        score += 15;
        println!("   CPU Cores: Good ({} cores)", cpu_cores);
    } else {
        score += 5;
        println!("   CPU Cores: Limited ({} cores)", cpu_cores);
        recommendations.push("More CPU cores would improve parallel processing");
    }

    // Memory assessment
    let total_memory = hw_info.memory().total_gb();
    if total_memory >= 32.0 {
        score += 20;
        println!("   System Memory: Excellent ({:.1} GB)", total_memory);
    } else if total_memory >= 16.0 {
        score += 15;
        println!("   System Memory: Good ({:.1} GB)", total_memory);
    } else {
        score += 5;
        println!("   System Memory: Limited ({:.1} GB)", total_memory);
        recommendations.push("More system memory recommended for large datasets");
    }

    // Specialized accelerators
    if !hw_info.npus().is_empty() {
        score += 10;
        println!("   NPU Acceleration: Available ({} units)", hw_info.npus().len());
    }
    
    if !hw_info.tpus().is_empty() {
        score += 15;
        println!("   TPU Acceleration: Available ({} units)", hw_info.tpus().len());
    }

    // Virtualization impact
    let virt_impact = hw_info.virtualization_performance_impact();
    if virt_impact < 0.9 {
        score -= 5;
        recommendations.push("Virtualization may impact AI performance");
    }

    // Thermal considerations
    if let Some(max_temp) = hw_info.thermal().max_temperature() {
        if max_temp > 80.0 {
            score -= 10;
            recommendations.push("High temperatures may cause throttling during AI workloads");
        }
    }

    println!("   Overall AI Readiness Score: {}/100", score);
    
    if score >= 80 {
        println!("   Assessment: Excellent for AI/ML workloads");
    } else if score >= 60 {
        println!("   Assessment: Good for most AI/ML tasks");
    } else if score >= 40 {
        println!("   Assessment: Suitable for basic AI tasks");
    } else {
        println!("   Assessment: Significant hardware upgrades recommended");
    }

    if !recommendations.is_empty() {
        println!("   Recommendations:");
        for rec in recommendations {
            println!("     • {}", rec);
        }
    }
}

#[cfg(feature = "monitoring")]
async fn demo_monitoring() -> Result<()> {
    use std::time::Duration;
    use tokio::time::timeout;

    println!("   Starting 10-second monitoring session...");
    
    let mut config = MonitoringConfig::default();
    config.update_interval = Duration::from_secs(2);
    config.thermal_threshold = 70.0; // Lower threshold for demo
    
    let monitor = HardwareMonitor::with_config(config);
    
    // Add some callbacks
    monitor.on_event(|event| {
        match event {
            MonitoringEvent::ThermalAlert { sensor_name, temperature, threshold, .. } => {
                println!("     🚨 Thermal Alert: {} at {:.1}°C (threshold: {:.1}°C)", 
                    sensor_name, temperature, threshold);
            }
            MonitoringEvent::PowerAlert { current_power, threshold, .. } => {
                println!("     ⚡ Power Alert: {:.1}W (threshold: {:.1}W)", 
                    current_power, threshold);
            }
            MonitoringEvent::MetricsUpdate { thermal_info, power_profile, .. } => {
                if let Some(thermal) = thermal_info {
                    if let Some(max_temp) = thermal.max_temperature() {
                        println!("     📊 Update: Max temp {:.1}°C, Status: {}", 
                            max_temp, thermal.thermal_status());
                    }
                }
                
                if let Some(power) = power_profile {
                    if let Some(total_power) = power.total_power_draw {
                        println!("     📊 Update: Power draw {:.1}W, Efficiency: {:.2}", 
                            total_power, power.efficiency_score);
                    }
                }
            }
            MonitoringEvent::MonitoringError { error, .. } => {
                println!("     ❌ Monitoring Error: {}", error);
            }
            _ => {}
        }
    }).await;

    // Start monitoring
    monitor.start_monitoring().await?;
    
    // Run for 10 seconds
    let _result = timeout(Duration::from_secs(10), async {
        tokio::time::sleep(Duration::from_secs(10)).await;
    }).await;

    // Stop monitoring
    monitor.stop_monitoring().await;
    
    // Show statistics
    let stats = monitor.get_stats().await;
    println!("   Monitoring completed:");
    println!("     Total events: {}", stats.total_events);
    println!("     Thermal alerts: {}", stats.thermal_alerts);
    println!("     Power alerts: {}", stats.power_alerts);
    println!("     Average update interval: {:?}", stats.average_update_interval);

    Ok(())
}
