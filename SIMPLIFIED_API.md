# Hardware Query - Simplified API Guide

The Hardware Query crate now provides three levels of API complexity to match different user needs:

> 💡 **New to this crate?** Try running the examples: `cargo run --example 01_simplified_api`

## 🚀 Quick Start (Simplified API)

For users who just want basic system information quickly:

```rust
use hardware_query::SystemOverview;

// Get essential system info in one call
let overview = SystemOverview::quick()?;
println!("CPU: {} ({} cores)", overview.cpu.name, overview.cpu.cores);
println!("Memory: {:.1} GB", overview.memory_gb);
println!("Performance Score: {}/100", overview.performance_score);
println!("AI Ready: {}", overview.is_ai_ready());
```

## 🎯 Use Case Presets

For specific scenarios like AI, gaming, development, or server deployment:

```rust
use hardware_query::HardwarePresets;

// AI/ML Assessment
let ai_assessment = HardwarePresets::ai_assessment()?;
println!("AI Score: {}/100", ai_assessment.ai_score);
println!("Can run large models: {}", ai_assessment.ai_score > 70);

// Gaming Assessment  
let gaming = HardwarePresets::gaming_assessment()?;
println!("Recommended: {} at {:?}", 
    gaming.recommended_settings.resolution,
    gaming.recommended_settings.quality_preset);

// Development Assessment
let dev = HardwarePresets::developer_assessment()?;
println!("Max VMs: {}", dev.virtualization_support.max_recommended_vms);

// Quick compatibility checks
let can_run_model = HardwarePresets::check_ai_model_compatibility(
    "GPT-3.5", "175B", 8.0 // 8GB VRAM required
)?;
```

## 🔧 Custom Queries (Builder Pattern)

For users who need specific hardware information:

```rust
use hardware_query::HardwareQueryBuilder;

// Query only what you need
let hw = HardwareQueryBuilder::new()
    .with_cpu()
    .with_memory()
    .with_gpu()
    .query()?;

// Pre-configured queries for common scenarios
let ai_hw = HardwareQueryBuilder::new().with_ai_focused().query()?;
let gaming_hw = HardwareQueryBuilder::new().with_gaming_focused().query()?;
let server_hw = HardwareQueryBuilder::new().with_server_focused().query()?;

// Custom filtering
let high_end_gpus = HardwareQueryBuilder::new()
    .with_gpu()
    .filter_gpus(|gpu| gpu.memory_gb() >= 8.0)
    .query()?;
```

## 📊 Advanced API (Full Control)

For users who need complete hardware details:

```rust
use hardware_query::HardwareInfo;

// Full system query with all details
let hw_info = HardwareInfo::query()?;

// Access detailed information
let cpu = hw_info.cpu();
println!("CPU: {} {} with {} MB cache",
    cpu.vendor(), cpu.model_name(), cpu.cache_size_mb());

// Check specific features
if cpu.has_feature("avx512") {
    println!("CPU supports AVX-512");
}

// Get all GPUs with detailed specs
for gpu in hw_info.gpus() {
    println!("GPU: {} - {} GB VRAM, {} compute units",
        gpu.model_name(), gpu.memory_gb(), gpu.compute_units());
}
```

## 🔄 Migration Path

### From Complex to Simple

If you're using the full API but want simpler code:

```rust
// Before (complex)
let hw_info = HardwareInfo::query()?;
let cpu_name = hw_info.cpu().model_name();
let memory_gb = hw_info.memory().total_gb();
let gpu_count = hw_info.gpus().len();

// After (simple)
let overview = SystemOverview::quick()?;
let cpu_name = &overview.cpu.name;
let memory_gb = overview.memory_gb;
let has_gpu = overview.gpu.is_some();
```

### Adding Specific Queries

If you need more than the overview but less than everything:

```rust
// Get only CPU and memory info
let essentials = HardwareQueryBuilder::cpu_and_memory()?;

// Get gaming-relevant info
let gaming_info = HardwareQueryBuilder::new()
    .with_cpu()
    .with_gpu() 
    .with_memory()
    .with_thermal()
    .query()?;
```

## 📈 Performance Comparison

| API Level | Query Time | Memory Usage | Use Case |
|-----------|------------|--------------|----------|
| SystemOverview::quick() | ~50ms | Low | Quick checks, dashboards |
| HardwarePresets | ~100ms | Medium | Specific assessments |
| Custom Builder | ~75-150ms | Medium | Targeted queries |
| Full HardwareInfo | ~200ms | High | Complete analysis |

## 🛠 Common Patterns

### System Health Dashboard
```rust
let overview = SystemOverview::quick()?;
match overview.health.status {
    HealthStatus::Excellent => println!("✅ System running optimally"),
    HealthStatus::Good => println!("✅ System running well"),
    HealthStatus::Fair => println!("⚠️ Minor issues detected"),
    HealthStatus::Poor => println!("⚠️ Issues need attention"),
    HealthStatus::Critical => println!("🚨 Critical issues!"),
}
```

### AI Model Compatibility
```rust
fn can_run_model(model_vram_gb: f64) -> Result<bool> {
    let overview = SystemOverview::quick()?;
    Ok(overview.gpu.as_ref().map_or(false, |gpu| gpu.vram_gb >= model_vram_gb))
}
```

### Gaming Performance Estimation
```rust
fn estimate_gaming_performance(resolution: &str) -> Result<String> {
    let gaming = HardwarePresets::gaming_assessment()?;
    let fps = HardwarePresets::gaming_fps_estimate(resolution, "High")?;
    Ok(format!("Expected {} FPS at {} (Score: {}/100)", 
        fps, resolution, gaming.gaming_score))
}
```

## 🚫 What NOT to Do

```rust
// DON'T: Query everything when you only need basics
let hw = HardwareInfo::query()?; // Slow and memory-intensive
let cpu_name = hw.cpu().model_name();

// DO: Use the appropriate API level
let overview = SystemOverview::quick()?; // Fast and lightweight
let cpu_name = &overview.cpu.name;

// DON'T: Repeatedly query the same information
for i in 0..100 {
    let overview = SystemOverview::quick()?; // Expensive!
}

// DO: Cache the results
let overview = SystemOverview::quick()?;
for i in 0..100 {
    // Use cached overview
}
```

## 🎯 Choosing the Right API

| If you need... | Use... |
|----------------|--------|
| Basic system info | `SystemOverview::quick()` |
| AI/ML suitability | `HardwarePresets::ai_assessment()` |
| Gaming performance | `HardwarePresets::gaming_assessment()` |
| Development setup | `HardwarePresets::developer_assessment()` |
| Server planning | `HardwarePresets::server_assessment()` |
| Specific components | `HardwareQueryBuilder` |
| Everything | `HardwareInfo::query()` |
| Quick compatibility | `HardwarePresets::check_*()` |

The simplified API is designed to make hardware-query accessible to developers who don't need to become hardware experts. Start with the simplest API that meets your needs, and move to more complex APIs only when necessary.
