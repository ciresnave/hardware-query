//! Virtualization and container detection module
//!
//! This module provides detection and analysis of virtualized environments,
//! including containers, virtual machines, and their impact on hardware performance.

use crate::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Virtualization environment information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualizationInfo {
    /// Type of virtualization environment
    pub environment_type: VirtualizationType,
    /// Hypervisor name and version (if applicable)
    pub hypervisor: Option<String>,
    /// Container runtime information (if applicable)
    pub container_runtime: Option<ContainerRuntime>,
    /// Resource limits imposed by the virtualization layer
    pub resource_limits: ResourceLimits,
    /// GPU passthrough capabilities
    pub gpu_passthrough: GPUPassthroughInfo,
    /// Performance impact factor (0.0 to 1.0, where 1.0 is native performance)
    pub performance_impact: f64,
    /// Nested virtualization support
    pub nested_virtualization: bool,
    /// Security features enabled
    pub security_features: Vec<SecurityFeature>,
    /// Platform-specific virtualization details
    pub platform_specific: HashMap<String, String>,
}

/// Type of virtualization environment
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VirtualizationType {
    /// Running on bare metal
    Native,
    /// Docker container
    Docker,
    /// Kubernetes pod
    Kubernetes,
    /// LXC/LXD container
    LXC,
    /// Podman container
    Podman,
    /// Full virtual machine
    VirtualMachine,
    /// Windows Subsystem for Linux
    WSL,
    /// Windows Subsystem for Linux 2
    WSL2,
    /// Wine compatibility layer
    Wine,
    /// macOS virtualization
    MacOSVirtualization,
    /// Unknown virtualization
    Unknown,
}

/// Container runtime information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerRuntime {
    /// Runtime name (docker, containerd, cri-o, etc.)
    pub name: String,
    /// Runtime version
    pub version: Option<String>,
    /// Container ID
    pub container_id: Option<String>,
    /// Container image
    pub image: Option<String>,
    /// Container orchestrator (if any)
    pub orchestrator: Option<String>,
    /// Security context
    pub security_context: SecurityContext,
}

/// Security context for containers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityContext {
    /// Running as privileged container
    pub privileged: bool,
    /// User namespace mapping
    pub user_namespace: bool,
    /// AppArmor/SELinux profile
    pub security_profile: Option<String>,
    /// Capabilities granted
    pub capabilities: Vec<String>,
    /// Resource isolation level
    pub isolation_level: IsolationLevel,
}

/// Container isolation level
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IsolationLevel {
    /// Full isolation
    Full,
    /// Partial isolation
    Partial,
    /// Minimal isolation
    Minimal,
    /// No isolation (dangerous)
    None,
}

/// Resource limits imposed by virtualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    /// CPU limits
    pub cpu_limits: CPULimits,
    /// Memory limits
    pub memory_limits: MemoryLimits,
    /// I/O limits
    pub io_limits: IOLimits,
    /// Network limits
    pub network_limits: NetworkLimits,
    /// GPU access restrictions
    pub gpu_limits: GPULimits,
}

/// CPU resource limits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CPULimits {
    /// Maximum CPU cores available
    pub max_cores: Option<u32>,
    /// CPU quota percentage
    pub quota_percent: Option<f32>,
    /// CPU shares/weight
    pub shares: Option<u32>,
    /// CPU affinity mask
    pub affinity_mask: Option<String>,
    /// Specific CPU features disabled
    pub disabled_features: Vec<String>,
}

/// Memory resource limits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryLimits {
    /// Maximum memory in bytes
    pub max_memory_bytes: Option<u64>,
    /// Maximum swap in bytes
    pub max_swap_bytes: Option<u64>,
    /// Memory reservation in bytes
    pub reservation_bytes: Option<u64>,
    /// OOM killer disabled
    pub oom_kill_disabled: bool,
    /// NUMA policy restrictions
    pub numa_policy: Option<String>,
}

/// I/O resource limits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IOLimits {
    /// Maximum read IOPS
    pub max_read_iops: Option<u64>,
    /// Maximum write IOPS
    pub max_write_iops: Option<u64>,
    /// Maximum read bandwidth (bytes/sec)
    pub max_read_bps: Option<u64>,
    /// Maximum write bandwidth (bytes/sec)
    pub max_write_bps: Option<u64>,
    /// Block device weights
    pub device_weights: HashMap<String, u32>,
}

/// Network resource limits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkLimits {
    /// Maximum bandwidth (bits/sec)
    pub max_bandwidth_bps: Option<u64>,
    /// Network namespace isolation
    pub network_namespace: bool,
    /// Port mapping restrictions
    pub port_restrictions: Vec<PortRestriction>,
    /// Network policies
    pub network_policies: Vec<String>,
}

/// Port access restriction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortRestriction {
    /// Port number or range
    pub port: String,
    /// Protocol (TCP/UDP)
    pub protocol: String,
    /// Access type (allow/deny)
    pub access: AccessType,
}

/// Access control type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccessType {
    Allow,
    Deny,
}

/// GPU access limitations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GPULimits {
    /// GPU access allowed
    pub gpu_access: bool,
    /// Specific GPU devices accessible
    pub accessible_devices: Vec<String>,
    /// GPU memory limits
    pub memory_limits: HashMap<String, u64>,
    /// Compute capability restrictions
    pub capability_restrictions: Vec<String>,
}

/// GPU passthrough information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GPUPassthroughInfo {
    /// GPU passthrough available
    pub available: bool,
    /// Type of GPU passthrough
    pub passthrough_type: GPUPassthroughType,
    /// Passed-through devices
    pub devices: Vec<PassthroughDevice>,
    /// Performance overhead
    pub performance_overhead: f64,
}

/// Type of GPU passthrough
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GPUPassthroughType {
    /// Full GPU passthrough
    Full,
    /// SR-IOV virtual functions
    SRIOV,
    /// GPU sharing/virtualization
    Shared,
    /// Software-only GPU emulation
    Emulated,
    /// No GPU access
    None,
}

/// Passthrough device information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassthroughDevice {
    /// Device ID
    pub device_id: String,
    /// Device name
    pub device_name: String,
    /// PCI address
    pub pci_address: Option<String>,
    /// Driver used
    pub driver: Option<String>,
}

/// Security features enabled in virtualization
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecurityFeature {
    /// Secure Boot enabled
    SecureBoot,
    /// Intel TXT/AMD SVM
    TrustedExecution,
    /// Intel VT-d/AMD-Vi
    IOMMU,
    /// Kernel Guard Technology
    KGT,
    /// Control Flow Integrity
    CFI,
    /// Address Space Layout Randomization
    ASLR,
    /// Data Execution Prevention
    DEP,
    /// Hypervisor-protected Code Integrity
    HVCI,
    /// Memory Protection Keys
    MPK,
    /// Intel CET
    CET,
}

impl VirtualizationInfo {
    /// Detect current virtualization environment
    pub fn detect() -> Result<Self> {
        let environment_type = Self::detect_environment_type()?;
        let hypervisor = Self::detect_hypervisor()?;
        let container_runtime = Self::detect_container_runtime()?;
        let resource_limits = Self::detect_resource_limits()?;
        let gpu_passthrough = Self::detect_gpu_passthrough()?;
        let performance_impact = Self::calculate_performance_impact(&environment_type);
        let nested_virtualization = Self::detect_nested_virtualization()?;
        let security_features = Self::detect_security_features()?;
        let platform_specific = Self::gather_platform_specific_info()?;

        Ok(Self {
            environment_type,
            hypervisor,
            container_runtime,
            resource_limits,
            gpu_passthrough,
            performance_impact,
            nested_virtualization,
            security_features,
            platform_specific,
        })
    }

    /// Check if running in any virtualized environment
    pub fn is_virtualized(&self) -> bool {
        self.environment_type != VirtualizationType::Native
    }

    /// Check if running in a container
    pub fn is_containerized(&self) -> bool {
        matches!(
            self.environment_type,
            VirtualizationType::Docker
                | VirtualizationType::Kubernetes
                | VirtualizationType::LXC
                | VirtualizationType::Podman
        )
    }

    /// Check if running in a virtual machine
    pub fn is_virtual_machine(&self) -> bool {
        matches!(
            self.environment_type,
            VirtualizationType::VirtualMachine | VirtualizationType::WSL | VirtualizationType::WSL2
        )
    }

    /// Get estimated performance compared to bare metal
    pub fn get_performance_factor(&self) -> f64 {
        self.performance_impact
    }

    /// Check if GPU acceleration is available
    pub fn has_gpu_access(&self) -> bool {
        self.gpu_passthrough.available || self.resource_limits.gpu_limits.gpu_access
    }

    /// Get security recommendations for the current environment
    pub fn get_security_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();

        if self.is_containerized() {
            if let Some(runtime) = &self.container_runtime {
                if runtime.security_context.privileged {
                    recommendations.push(
                        "Consider running container in non-privileged mode for better security"
                            .to_string(),
                    );
                }

                if !runtime.security_context.user_namespace {
                    recommendations.push(
                        "Enable user namespace mapping for better isolation".to_string(),
                    );
                }

                if runtime.security_context.security_profile.is_none() {
                    recommendations.push(
                        "Apply AppArmor/SELinux security profile".to_string(),
                    );
                }
            }
        }

        if self.is_virtual_machine() && !self.security_features.contains(&SecurityFeature::SecureBoot) {
            recommendations.push("Enable Secure Boot for enhanced security".to_string());
        }

        if self.gpu_passthrough.available
            && matches!(
                self.gpu_passthrough.passthrough_type,
                GPUPassthroughType::Full
            )
        {
            recommendations.push(
                "GPU passthrough detected - ensure IOMMU isolation is properly configured"
                    .to_string(),
            );
        }

        recommendations
    }

    fn detect_environment_type() -> Result<VirtualizationType> {
        // Platform-specific detection logic would go here
        // For now, implement basic detection

        // Check for common container indicators
        if Self::check_docker_container()? {
            return Ok(VirtualizationType::Docker);
        }

        if Self::check_kubernetes_pod()? {
            return Ok(VirtualizationType::Kubernetes);
        }

        if Self::check_wsl()? {
            return Ok(VirtualizationType::WSL);
        }

        // Check for VM indicators
        if Self::check_virtual_machine()? {
            return Ok(VirtualizationType::VirtualMachine);
        }

        Ok(VirtualizationType::Native)
    }

    fn check_docker_container() -> Result<bool> {
        // Check for /.dockerenv file
        Ok(std::path::Path::new("/.dockerenv").exists())
    }

    fn check_kubernetes_pod() -> Result<bool> {
        // Check for Kubernetes environment variables
        Ok(std::env::var("KUBERNETES_SERVICE_HOST").is_ok())
    }

    fn check_wsl() -> Result<bool> {
        // Check for WSL indicators
        #[cfg(target_os = "linux")]
        {
            if let Ok(version) = std::fs::read_to_string("/proc/version") {
                return Ok(version.to_lowercase().contains("microsoft"));
            }
        }
        Ok(false)
    }

    fn check_virtual_machine() -> Result<bool> {
        // Basic VM detection - would be enhanced with platform-specific checks
        Ok(false)
    }

    fn detect_hypervisor() -> Result<Option<String>> {
        // Platform-specific hypervisor detection
        Ok(None)
    }

    fn detect_container_runtime() -> Result<Option<ContainerRuntime>> {
        // Container runtime detection
        Ok(None)
    }

    fn detect_resource_limits() -> Result<ResourceLimits> {
        // Resource limits detection
        Ok(ResourceLimits {
            cpu_limits: CPULimits {
                max_cores: None,
                quota_percent: None,
                shares: None,
                affinity_mask: None,
                disabled_features: Vec::new(),
            },
            memory_limits: MemoryLimits {
                max_memory_bytes: None,
                max_swap_bytes: None,
                reservation_bytes: None,
                oom_kill_disabled: false,
                numa_policy: None,
            },
            io_limits: IOLimits {
                max_read_iops: None,
                max_write_iops: None,
                max_read_bps: None,
                max_write_bps: None,
                device_weights: HashMap::new(),
            },
            network_limits: NetworkLimits {
                max_bandwidth_bps: None,
                network_namespace: false,
                port_restrictions: Vec::new(),
                network_policies: Vec::new(),
            },
            gpu_limits: GPULimits {
                gpu_access: true,
                accessible_devices: Vec::new(),
                memory_limits: HashMap::new(),
                capability_restrictions: Vec::new(),
            },
        })
    }

    fn detect_gpu_passthrough() -> Result<GPUPassthroughInfo> {
        // GPU passthrough detection
        Ok(GPUPassthroughInfo {
            available: false,
            passthrough_type: GPUPassthroughType::None,
            devices: Vec::new(),
            performance_overhead: 0.0,
        })
    }

    fn calculate_performance_impact(env_type: &VirtualizationType) -> f64 {
        match env_type {
            VirtualizationType::Native => 1.0,
            VirtualizationType::Docker | VirtualizationType::LXC | VirtualizationType::Podman => 0.95,
            VirtualizationType::Kubernetes => 0.92,
            VirtualizationType::WSL2 => 0.90,
            VirtualizationType::VirtualMachine => 0.85,
            VirtualizationType::WSL => 0.80,
            VirtualizationType::Wine => 0.75,
            _ => 0.70,
        }
    }

    fn detect_nested_virtualization() -> Result<bool> {
        // Nested virtualization detection
        Ok(false)
    }

    fn detect_security_features() -> Result<Vec<SecurityFeature>> {
        // Security features detection
        Ok(Vec::new())
    }

    fn gather_platform_specific_info() -> Result<HashMap<String, String>> {
        // Platform-specific information gathering
        Ok(HashMap::new())
    }
}

impl std::fmt::Display for VirtualizationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VirtualizationType::Native => write!(f, "Native"),
            VirtualizationType::Docker => write!(f, "Docker"),
            VirtualizationType::Kubernetes => write!(f, "Kubernetes"),
            VirtualizationType::LXC => write!(f, "LXC"),
            VirtualizationType::Podman => write!(f, "Podman"),
            VirtualizationType::VirtualMachine => write!(f, "Virtual Machine"),
            VirtualizationType::WSL => write!(f, "WSL"),
            VirtualizationType::WSL2 => write!(f, "WSL2"),
            VirtualizationType::Wine => write!(f, "Wine"),
            VirtualizationType::MacOSVirtualization => write!(f, "macOS Virtualization"),
            VirtualizationType::Unknown => write!(f, "Unknown"),
        }
    }
}
