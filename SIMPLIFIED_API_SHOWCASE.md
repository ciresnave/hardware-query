# Hardware Query - README Update

## 🚀 Three Levels of API Complexity

The Hardware Query crate now provides three distinct API levels to accommodate different user needs:

### Level 1: Quick Overview (Simplest)
Perfect for dashboards, system monitors, and quick checks:

```rust
use hardware_query::SystemOverview;

// Get essential system info in one line
let overview = SystemOverview::quick()?;
println!("CPU: {} ({} cores)", overview.cpu.name, overview.cpu.cores);
println!("Memory: {:.1} GB", overview.memory_gb);
println!("Performance Score: {}/100", overview.performance_score);
println!("AI Ready: {}", overview.is_ai_ready());
```

### Level 2: Use Case Presets (Targeted)
Pre-configured assessments for specific scenarios:

```rust
use hardware_query::HardwarePresets;

// AI/ML Assessment
let ai_assessment = HardwarePresets::ai_assessment()?;
println!("AI Score: {}/100", ai_assessment.ai_score);

// Gaming Performance
let gaming = HardwarePresets::gaming_assessment()?;
println!("Recommended: {} at {:?}", 
    gaming.recommended_settings.resolution,
    gaming.recommended_settings.quality_preset);

// Quick compatibility checks
let can_run_gpt35 = HardwarePresets::check_ai_model_compatibility(
    "GPT-3.5", "175B", 8.0 // 8GB VRAM required
)?;
```

### Level 3: Custom Queries (Flexible)
Builder pattern for specific hardware information:

```rust
use hardware_query::HardwareQueryBuilder;

// Query only what you need
let hw = HardwareQueryBuilder::new()
    .with_cpu()
    .with_memory()
    .with_gpu()
    .query()?;

// Pre-configured for common use cases
let ai_focused = HardwareQueryBuilder::new().with_ai_focused().query()?;
let gaming_focused = HardwareQueryBuilder::new().with_gaming_focused().query()?;
```

### Level 4: Full Control (Complete)
Access to all hardware details (existing API):

```rust
use hardware_query::HardwareInfo;

// Full system query with all details
let hw_info = HardwareInfo::query()?;
// ... access all hardware components
```

## 📊 API Comparison

| Need | Recommended API | Query Time | Memory | Code Lines |
|------|----------------|-----------|---------|------------|
| System overview | `SystemOverview::quick()` | ~50ms | Low | 1-2 |
| AI suitability | `HardwarePresets::ai_assessment()` | ~100ms | Medium | 2-3 |
| Gaming performance | `HardwarePresets::gaming_assessment()` | ~100ms | Medium | 2-3 |
| Specific components | `HardwareQueryBuilder` | ~75ms | Medium | 3-5 |
| Everything | `HardwareInfo::query()` | ~200ms | High | 1+ |

## 🎯 Migration Guide

### Simplifying Existing Code

**Before (complex):**
```rust
let hw_info = HardwareInfo::query()?;
let cpu_name = hw_info.cpu().model_name();
let memory_gb = hw_info.memory().total_gb();
let has_gpu = !hw_info.gpus().is_empty();
let gpu_vram = hw_info.gpus().first().map(|g| g.memory_gb()).unwrap_or(0.0);
```

**After (simple):**
```rust
let overview = SystemOverview::quick()?;
let cpu_name = &overview.cpu.name;
let memory_gb = overview.memory_gb;
let has_gpu = overview.gpu.is_some();
let gpu_vram = overview.gpu.as_ref().map(|g| g.vram_gb).unwrap_or(0.0);
```

### Adding Targeted Analysis

**AI/ML Workload Assessment:**
```rust
let assessment = HardwarePresets::ai_assessment()?;
if assessment.ai_score > 70 {
    println!("Excellent for AI/ML workloads!");
    for framework in &assessment.frameworks {
        println!("- {}: {:?}", framework.name, framework.compatibility);
    }
}
```

**Gaming Performance:**
```rust
let gaming = HardwarePresets::gaming_assessment()?;
let fps_estimate = HardwarePresets::gaming_fps_estimate("1080p", "High")?;
println!("Expected performance: {} FPS at 1080p High", fps_estimate);
```

The simplified API maintains full backward compatibility while making hardware-query accessible to developers who don't need to become hardware experts.
