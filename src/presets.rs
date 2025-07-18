//! Hardware query presets for common use cases
//!
//! This module provides pre-configured hardware queries for common scenarios,
//! making it extremely easy for developers to get the information they need
//! without having to understand all the available hardware types.

use crate::{simple::SystemOverview, builder::HardwareQueryBuilder, Result};
use serde::{Serialize, Deserialize};

/// AI/ML hardware assessment result
#[derive(Debug, Serialize, Deserialize)]
pub struct AIHardwareAssessment {
    /// System overview
    pub overview: SystemOverview,
    /// AI readiness score (0-100)
    pub ai_score: u8,
    /// Recommended AI frameworks
    pub frameworks: Vec<AIFramework>,
    /// Memory requirements for different model sizes
    pub model_recommendations: ModelRecommendations,
    /// Performance expectations
    pub performance: AIPerformanceEstimate,
    /// Optimization suggestions
    pub optimizations: Vec<String>,
}

/// Gaming hardware assessment result
#[derive(Debug, Serialize, Deserialize)]
pub struct GamingHardwareAssessment {
    /// System overview
    pub overview: SystemOverview,
    /// Gaming performance score (0-100)
    pub gaming_score: u8,
    /// Recommended game settings
    pub recommended_settings: GameSettings,
    /// Performance bottlenecks
    pub bottlenecks: Vec<String>,
    /// Upgrade recommendations
    pub upgrade_recommendations: Vec<String>,
}

/// Developer hardware assessment result
#[derive(Debug, Serialize, Deserialize)]
pub struct DeveloperHardwareAssessment {
    /// System overview
    pub overview: SystemOverview,
    /// Development performance score (0-100)
    pub dev_score: u8,
    /// Recommended development environments
    pub environments: Vec<DevEnvironment>,
    /// Virtualization capabilities
    pub virtualization_support: VirtualizationSupport,
    /// Recommended tools and configurations
    pub tool_recommendations: Vec<String>,
}

/// Server hardware assessment result
#[derive(Debug, Serialize, Deserialize)]
pub struct ServerHardwareAssessment {
    /// System overview
    pub overview: SystemOverview,
    /// Server performance score (0-100)
    pub server_score: u8,
    /// Recommended server workloads
    pub workload_suitability: Vec<WorkloadSuitability>,
    /// Resource allocation recommendations
    pub resource_allocation: ResourceAllocation,
    /// Reliability assessment
    pub reliability: ReliabilityAssessment,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AIFramework {
    pub name: String,
    pub compatibility: CompatibilityLevel,
    pub performance_estimate: PerformanceLevel,
    pub requirements_met: bool,
    pub notes: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelRecommendations {
    pub small_models: Vec<ModelRecommendation>,
    pub medium_models: Vec<ModelRecommendation>,
    pub large_models: Vec<ModelRecommendation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelRecommendation {
    pub name: String,
    pub parameter_count: String,
    pub memory_required_gb: f64,
    pub feasible: bool,
    pub performance_estimate: PerformanceLevel,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AIPerformanceEstimate {
    pub training_capability: PerformanceLevel,
    pub inference_capability: PerformanceLevel,
    pub batch_processing: PerformanceLevel,
    pub real_time_processing: PerformanceLevel,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameSettings {
    pub resolution: String,
    pub quality_preset: QualityLevel,
    pub raytracing_support: bool,
    pub target_fps: u32,
    pub vram_usage_percent: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DevEnvironment {
    pub name: String,
    pub suitability: CompatibilityLevel,
    pub container_support: bool,
    pub vm_support: bool,
    pub recommended_config: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualizationSupport {
    pub hardware_acceleration: bool,
    pub nested_virtualization: bool,
    pub max_recommended_vms: u32,
    pub docker_performance: PerformanceLevel,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkloadSuitability {
    pub workload_type: String,
    pub suitability_score: u8,
    pub max_concurrent_users: Option<u32>,
    pub notes: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceAllocation {
    pub recommended_vm_count: u32,
    pub memory_per_vm_gb: f64,
    pub cpu_cores_per_vm: u32,
    pub storage_allocation_gb: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReliabilityAssessment {
    pub uptime_estimate: f64,
    pub thermal_stability: QualityLevel,
    pub power_stability: QualityLevel,
    pub maintenance_requirements: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompatibilityLevel {
    Excellent,
    Good,
    Fair,
    Poor,
    Incompatible,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PerformanceLevel {
    Excellent,
    Good,
    Fair,
    Poor,
    Inadequate,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum QualityLevel {
    Ultra,
    High,
    Medium,
    Low,
    Minimum,
}

/// Hardware query presets for common use cases
pub struct HardwarePresets;

impl HardwarePresets {
    /// Quick system overview - fastest query with most important information
    pub fn quick_overview() -> Result<SystemOverview> {
        SystemOverview::quick()
    }

    /// Comprehensive AI/ML hardware assessment
    pub fn ai_assessment() -> Result<AIHardwareAssessment> {
        let overview = SystemOverview::quick()?;
        let ai_score = overview.ai_score();
        
        let frameworks = Self::assess_ai_frameworks(&overview);
        let model_recommendations = Self::get_model_recommendations(&overview);
        let performance = Self::estimate_ai_performance(&overview);
        let optimizations = Self::get_ai_optimizations(&overview);

        Ok(AIHardwareAssessment {
            overview,
            ai_score,
            frameworks,
            model_recommendations,
            performance,
            optimizations,
        })
    }

    /// Gaming hardware assessment and recommendations
    pub fn gaming_assessment() -> Result<GamingHardwareAssessment> {
        let _hw_info = HardwareQueryBuilder::new()
            .with_gaming_focused()
            .query()?;
        
        let overview = SystemOverview::quick()?;
        let gaming_score = Self::calculate_gaming_score(&overview);
        let recommended_settings = Self::get_game_settings(&overview);
        let bottlenecks = Self::identify_gaming_bottlenecks(&overview);
        let upgrade_recommendations = Self::get_gaming_upgrades(&overview);

        Ok(GamingHardwareAssessment {
            overview,
            gaming_score,
            recommended_settings,
            bottlenecks,
            upgrade_recommendations,
        })
    }

    /// Developer workstation assessment
    pub fn developer_assessment() -> Result<DeveloperHardwareAssessment> {
        let overview = SystemOverview::quick()?;
        let dev_score = Self::calculate_dev_score(&overview);
        let environments = Self::assess_dev_environments(&overview);
        let virtualization_support = Self::assess_virtualization(&overview);
        let tool_recommendations = Self::get_dev_tool_recommendations(&overview);

        Ok(DeveloperHardwareAssessment {
            overview,
            dev_score,
            environments,
            virtualization_support,
            tool_recommendations,
        })
    }

    /// Server hardware assessment
    pub fn server_assessment() -> Result<ServerHardwareAssessment> {
        let _hw_info = HardwareQueryBuilder::new()
            .with_server_focused()
            .query()?;
        
        let overview = SystemOverview::quick()?;
        let server_score = Self::calculate_server_score(&overview);
        let workload_suitability = Self::assess_server_workloads(&overview);
        let resource_allocation = Self::recommend_resource_allocation(&overview);
        let reliability = Self::assess_reliability(&overview);

        Ok(ServerHardwareAssessment {
            overview,
            server_score,
            workload_suitability,
            resource_allocation,
            reliability,
        })
    }

    /// Check if system is ready for a specific AI model
    pub fn check_ai_model_compatibility(_model_name: &str, _params: &str, memory_gb: f64) -> Result<bool> {
        let overview = SystemOverview::quick()?;
        
        // Simple compatibility check
        let available_memory = if let Some(gpu) = &overview.gpu {
            gpu.vram_gb
        } else {
            overview.memory_gb * 0.7 // Assume 70% of system RAM is available
        };

        Ok(available_memory >= memory_gb && overview.is_ai_ready())
    }

    /// Get quick gaming performance estimate
    pub fn gaming_fps_estimate(resolution: &str, quality: &str) -> Result<u32> {
        let overview = SystemOverview::quick()?;
        
        // Simple FPS estimation based on hardware
        let base_fps = if let Some(gpu) = &overview.gpu {
            if gpu.vram_gb >= 8.0 {
                120
            } else if gpu.vram_gb >= 4.0 {
                80
            } else {
                30
            }
        } else {
            15
        };

        // Apply resolution and quality modifiers
        let resolution_modifier = match resolution {
            "1080p" => 1.0,
            "1440p" => 0.7,
            "4K" => 0.4,
            _ => 1.0,
        };

        let quality_modifier = match quality {
            "Low" => 1.2,
            "Medium" => 1.0,
            "High" => 0.8,
            "Ultra" => 0.6,
            _ => 1.0,
        };

        let estimated_fps = (base_fps as f64 * resolution_modifier * quality_modifier) as u32;
        Ok(estimated_fps.max(15)) // Minimum 15 FPS
    }

    // Private implementation methods
    fn assess_ai_frameworks(overview: &SystemOverview) -> Vec<AIFramework> {
        let mut frameworks = Vec::new();

        // PyTorch
        frameworks.push(AIFramework {
            name: "PyTorch".to_string(),
            compatibility: if overview.gpu.is_some() { 
                CompatibilityLevel::Excellent 
            } else { 
                CompatibilityLevel::Good 
            },
            performance_estimate: if overview.gpu.as_ref().map_or(false, |g| g.ai_capable) {
                PerformanceLevel::Excellent
            } else {
                PerformanceLevel::Fair
            },
            requirements_met: overview.memory_gb >= 4.0,
            notes: "Popular deep learning framework with excellent GPU support".to_string(),
        });

        // TensorFlow
        frameworks.push(AIFramework {
            name: "TensorFlow".to_string(),
            compatibility: if overview.gpu.is_some() { 
                CompatibilityLevel::Excellent 
            } else { 
                CompatibilityLevel::Good 
            },
            performance_estimate: if overview.gpu.as_ref().map_or(false, |g| g.ai_capable) {
                PerformanceLevel::Excellent
            } else {
                PerformanceLevel::Fair
            },
            requirements_met: overview.memory_gb >= 4.0,
            notes: "Google's ML framework with strong production support".to_string(),
        });

        // ONNX Runtime
        frameworks.push(AIFramework {
            name: "ONNX Runtime".to_string(),
            compatibility: CompatibilityLevel::Excellent,
            performance_estimate: PerformanceLevel::Good,
            requirements_met: true,
            notes: "Cross-platform inference with broad hardware support".to_string(),
        });

        frameworks
    }

    fn get_model_recommendations(overview: &SystemOverview) -> ModelRecommendations {
        let available_vram = overview.gpu.as_ref().map_or(0.0, |g| g.vram_gb);
        let available_ram = overview.memory_gb;

        ModelRecommendations {
            small_models: vec![
                ModelRecommendation {
                    name: "BERT-base".to_string(),
                    parameter_count: "110M".to_string(),
                    memory_required_gb: 1.0,
                    feasible: available_vram >= 1.0 || available_ram >= 2.0,
                    performance_estimate: PerformanceLevel::Excellent,
                },
                ModelRecommendation {
                    name: "DistilBERT".to_string(),
                    parameter_count: "66M".to_string(),
                    memory_required_gb: 0.5,
                    feasible: true,
                    performance_estimate: PerformanceLevel::Excellent,
                },
            ],
            medium_models: vec![
                ModelRecommendation {
                    name: "GPT-3.5".to_string(),
                    parameter_count: "175B".to_string(),
                    memory_required_gb: 8.0,
                    feasible: available_vram >= 8.0,
                    performance_estimate: if available_vram >= 8.0 { 
                        PerformanceLevel::Good 
                    } else { 
                        PerformanceLevel::Poor 
                    },
                },
            ],
            large_models: vec![
                ModelRecommendation {
                    name: "GPT-4".to_string(),
                    parameter_count: "1.7T".to_string(),
                    memory_required_gb: 80.0,
                    feasible: available_vram >= 80.0,
                    performance_estimate: if available_vram >= 80.0 { 
                        PerformanceLevel::Fair 
                    } else { 
                        PerformanceLevel::Inadequate 
                    },
                },
            ],
        }
    }

    fn estimate_ai_performance(overview: &SystemOverview) -> AIPerformanceEstimate {
        let has_gpu = overview.gpu.is_some();
        let gpu_ai_capable = overview.gpu.as_ref().map_or(false, |g| g.ai_capable);
        let sufficient_memory = overview.memory_gb >= 16.0;

        AIPerformanceEstimate {
            training_capability: if gpu_ai_capable && sufficient_memory {
                PerformanceLevel::Good
            } else if has_gpu {
                PerformanceLevel::Fair
            } else {
                PerformanceLevel::Poor
            },
            inference_capability: if gpu_ai_capable {
                PerformanceLevel::Excellent
            } else if has_gpu {
                PerformanceLevel::Good
            } else {
                PerformanceLevel::Fair
            },
            batch_processing: if gpu_ai_capable && sufficient_memory {
                PerformanceLevel::Excellent
            } else {
                PerformanceLevel::Fair
            },
            real_time_processing: if gpu_ai_capable {
                PerformanceLevel::Good
            } else {
                PerformanceLevel::Fair
            },
        }
    }

    fn get_ai_optimizations(overview: &SystemOverview) -> Vec<String> {
        let mut optimizations = Vec::new();

        if overview.gpu.is_none() {
            optimizations.push("Consider adding a dedicated GPU for AI acceleration".to_string());
        }

        if overview.memory_gb < 16.0 {
            optimizations.push("Increase system RAM to 16GB+ for better model performance".to_string());
        }

        if let Some(gpu) = &overview.gpu {
            if gpu.vram_gb < 8.0 {
                optimizations.push("Consider GPU with more VRAM for larger models".to_string());
            }
        }

        optimizations.extend(overview.get_recommendations());
        optimizations
    }

    fn calculate_gaming_score(overview: &SystemOverview) -> u8 {
        let mut score = 0;

        // GPU is most important for gaming (60 points)
        if let Some(gpu) = &overview.gpu {
            if gpu.vram_gb >= 12.0 {
                score += 60;
            } else if gpu.vram_gb >= 8.0 {
                score += 50;
            } else if gpu.vram_gb >= 6.0 {
                score += 40;
            } else if gpu.vram_gb >= 4.0 {
                score += 30;
            } else {
                score += 15;
            }
        }

        // CPU (25 points)
        if overview.cpu.cores >= 8 {
            score += 25;
        } else if overview.cpu.cores >= 6 {
            score += 20;
        } else if overview.cpu.cores >= 4 {
            score += 15;
        } else {
            score += 5;
        }

        // Memory (15 points)
        if overview.memory_gb >= 32.0 {
            score += 15;
        } else if overview.memory_gb >= 16.0 {
            score += 12;
        } else if overview.memory_gb >= 8.0 {
            score += 8;
        } else {
            score += 3;
        }

        score.min(100)
    }

    fn get_game_settings(overview: &SystemOverview) -> GameSettings {
        let vram = overview.gpu.as_ref().map_or(0.0, |g| g.vram_gb);
        
        let (resolution, quality, target_fps) = if vram >= 12.0 {
            ("4K", QualityLevel::Ultra, 60)
        } else if vram >= 8.0 {
            ("1440p", QualityLevel::High, 75)
        } else if vram >= 6.0 {
            ("1080p", QualityLevel::High, 60)
        } else if vram >= 4.0 {
            ("1080p", QualityLevel::Medium, 60)
        } else {
            ("1080p", QualityLevel::Low, 30)
        };

        GameSettings {
            resolution: resolution.to_string(),
            quality_preset: quality,
            raytracing_support: vram >= 8.0,
            target_fps,
            vram_usage_percent: 85,
        }
    }

    fn identify_gaming_bottlenecks(overview: &SystemOverview) -> Vec<String> {
        let mut bottlenecks = Vec::new();

        if overview.gpu.is_none() {
            bottlenecks.push("No dedicated GPU - severely limits gaming performance".to_string());
        } else if let Some(gpu) = &overview.gpu {
            if gpu.vram_gb < 4.0 {
                bottlenecks.push("Low GPU VRAM limits texture quality and resolution".to_string());
            }
        }

        if overview.cpu.cores < 4 {
            bottlenecks.push("Low CPU core count may limit performance in modern games".to_string());
        }

        if overview.memory_gb < 16.0 {
            bottlenecks.push("Low system RAM may cause stuttering in memory-intensive games".to_string());
        }

        if overview.storage.drive_type.to_lowercase().contains("hdd") {
            bottlenecks.push("HDD storage may cause slow loading times".to_string());
        }

        bottlenecks
    }

    fn get_gaming_upgrades(overview: &SystemOverview) -> Vec<String> {
        let mut upgrades = Vec::new();

        if let Some(gpu) = &overview.gpu {
            if gpu.vram_gb < 8.0 {
                upgrades.push("Upgrade to GPU with 8GB+ VRAM for modern games".to_string());
            }
        } else {
            upgrades.push("Add dedicated gaming GPU".to_string());
        }

        if overview.memory_gb < 16.0 {
            upgrades.push("Upgrade to 16GB+ RAM".to_string());
        }

        if overview.storage.drive_type.to_lowercase().contains("hdd") {
            upgrades.push("Upgrade to NVMe SSD for faster loading".to_string());
        }

        upgrades
    }

    fn calculate_dev_score(overview: &SystemOverview) -> u8 {
        let mut score = 0;

        // CPU is crucial for development (40 points)
        if overview.cpu.cores >= 16 {
            score += 40;
        } else if overview.cpu.cores >= 8 {
            score += 35;
        } else if overview.cpu.cores >= 6 {
            score += 25;
        } else {
            score += 15;
        }

        // Memory for IDEs and build tools (35 points)
        if overview.memory_gb >= 32.0 {
            score += 35;
        } else if overview.memory_gb >= 16.0 {
            score += 25;
        } else if overview.memory_gb >= 8.0 {
            score += 15;
        } else {
            score += 5;
        }

        // Storage for fast builds (25 points)
        if overview.storage.drive_type.to_lowercase().contains("nvme") {
            score += 25;
        } else if overview.storage.drive_type.to_lowercase().contains("ssd") {
            score += 20;
        } else {
            score += 10;
        }

        score.min(100)
    }

    fn assess_dev_environments(overview: &SystemOverview) -> Vec<DevEnvironment> {
        vec![
            DevEnvironment {
                name: "Visual Studio Code".to_string(),
                suitability: CompatibilityLevel::Excellent,
                container_support: true,
                vm_support: false,
                recommended_config: "Lightweight, excellent for most development tasks".to_string(),
            },
            DevEnvironment {
                name: "Docker Desktop".to_string(),
                suitability: if overview.memory_gb >= 8.0 { 
                    CompatibilityLevel::Excellent 
                } else { 
                    CompatibilityLevel::Fair 
                },
                container_support: true,
                vm_support: true,
                recommended_config: "Requires 8GB+ RAM for optimal performance".to_string(),
            },
            DevEnvironment {
                name: "IntelliJ IDEA".to_string(),
                suitability: if overview.memory_gb >= 16.0 { 
                    CompatibilityLevel::Excellent 
                } else { 
                    CompatibilityLevel::Good 
                },
                container_support: true,
                vm_support: false,
                recommended_config: "Heavy IDE, benefits from 16GB+ RAM".to_string(),
            },
        ]
    }

    fn assess_virtualization(overview: &SystemOverview) -> VirtualizationSupport {
        let hardware_acceleration = overview.cpu.cores >= 4;
        let nested_virtualization = overview.cpu.cores >= 8;
        let max_vms = if overview.memory_gb >= 32.0 { 4 } else if overview.memory_gb >= 16.0 { 2 } else { 1 };
        let docker_performance = if overview.memory_gb >= 16.0 { 
            PerformanceLevel::Excellent 
        } else { 
            PerformanceLevel::Good 
        };

        VirtualizationSupport {
            hardware_acceleration,
            nested_virtualization,
            max_recommended_vms: max_vms,
            docker_performance,
        }
    }

    fn get_dev_tool_recommendations(overview: &SystemOverview) -> Vec<String> {
        let mut recommendations = Vec::new();

        recommendations.push("Git for version control".to_string());
        
        if overview.memory_gb >= 16.0 {
            recommendations.push("Docker for containerized development".to_string());
        }
        
        if overview.cpu.cores >= 8 {
            recommendations.push("Parallel build tools (ninja, etc.)".to_string());
        }
        
        recommendations.push("Terminal with good performance (Windows Terminal, iTerm2)".to_string());
        
        recommendations
    }

    fn calculate_server_score(overview: &SystemOverview) -> u8 {
        // Server scoring focuses on stability, multiple cores, and adequate memory
        let mut score = 0;

        // CPU cores for concurrent handling (40 points)
        if overview.cpu.cores >= 32 {
            score += 40;
        } else if overview.cpu.cores >= 16 {
            score += 35;
        } else if overview.cpu.cores >= 8 {
            score += 25;
        } else {
            score += 10;
        }

        // Memory for server applications (35 points)
        if overview.memory_gb >= 64.0 {
            score += 35;
        } else if overview.memory_gb >= 32.0 {
            score += 25;
        } else if overview.memory_gb >= 16.0 {
            score += 15;
        } else {
            score += 5;
        }

        // Storage reliability and speed (25 points)
        if overview.storage.drive_type.to_lowercase().contains("nvme") {
            score += 25;
        } else if overview.storage.drive_type.to_lowercase().contains("ssd") {
            score += 20;
        } else {
            score += 10;
        }

        score.min(100)
    }

    fn assess_server_workloads(overview: &SystemOverview) -> Vec<WorkloadSuitability> {
        vec![
            WorkloadSuitability {
                workload_type: "Web Server".to_string(),
                suitability_score: if overview.cpu.cores >= 8 { 90 } else { 70 },
                max_concurrent_users: Some(overview.cpu.cores * 100),
                notes: "Good for serving web applications".to_string(),
            },
            WorkloadSuitability {
                workload_type: "Database Server".to_string(),
                suitability_score: if overview.memory_gb >= 32.0 { 85 } else { 60 },
                max_concurrent_users: Some((overview.memory_gb as u32) * 10),
                notes: "Memory-intensive workload".to_string(),
            },
            WorkloadSuitability {
                workload_type: "Container Orchestration".to_string(),
                suitability_score: if overview.cpu.cores >= 16 && overview.memory_gb >= 32.0 { 95 } else { 70 },
                max_concurrent_users: None,
                notes: "Requires high CPU and memory for container management".to_string(),
            },
        ]
    }

    fn recommend_resource_allocation(overview: &SystemOverview) -> ResourceAllocation {
        let vm_count = if overview.memory_gb >= 64.0 { 8 } else if overview.memory_gb >= 32.0 { 4 } else { 2 };
        let memory_per_vm = (overview.memory_gb * 0.8) / (vm_count as f64);
        let cores_per_vm = overview.cpu.cores / vm_count;
        let storage_per_vm = overview.storage.total_gb * 0.7 / (vm_count as f64);

        ResourceAllocation {
            recommended_vm_count: vm_count,
            memory_per_vm_gb: memory_per_vm,
            cpu_cores_per_vm: cores_per_vm,
            storage_allocation_gb: storage_per_vm,
        }
    }

    fn assess_reliability(overview: &SystemOverview) -> ReliabilityAssessment {
        let thermal_stability = match overview.health.temperature {
            crate::simple::TemperatureStatus::Normal => QualityLevel::High,
            crate::simple::TemperatureStatus::Warm => QualityLevel::Medium,
            crate::simple::TemperatureStatus::Hot => QualityLevel::Low,
            crate::simple::TemperatureStatus::Critical => QualityLevel::Minimum,
        };

        let power_stability = match overview.health.power {
            crate::simple::PowerStatus::Low | crate::simple::PowerStatus::Normal => QualityLevel::High,
            crate::simple::PowerStatus::High => QualityLevel::Medium,
            crate::simple::PowerStatus::VeryHigh => QualityLevel::Low,
        };

        let uptime_estimate = match overview.health.status {
            crate::simple::HealthStatus::Excellent => 99.9,
            crate::simple::HealthStatus::Good => 99.5,
            crate::simple::HealthStatus::Fair => 99.0,
            crate::simple::HealthStatus::Poor => 98.0,
            crate::simple::HealthStatus::Critical => 95.0,
        };

        ReliabilityAssessment {
            uptime_estimate,
            thermal_stability,
            power_stability,
            maintenance_requirements: overview.health.warnings.clone(),
        }
    }
}
