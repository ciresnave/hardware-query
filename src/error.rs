/// Result type for hardware query operations
pub type Result<T> = std::result::Result<T, HardwareQueryError>;

/// Error types that can occur during hardware querying
#[derive(Debug, thiserror::Error)]
pub enum HardwareQueryError {
    /// System information is not available
    #[error("System information not available: {0}")]
    SystemInfoUnavailable(String),

    /// Hardware device not found
    #[error("Hardware device not found: {0}")]
    DeviceNotFound(String),

    /// Platform not supported
    #[error("Platform not supported: {0}")]
    PlatformNotSupported(String),

    /// Permission denied accessing hardware information
    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    /// I/O error occurred
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// Serialization error
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    /// WMI error (Windows only)
    #[cfg(target_os = "windows")]
    #[error("WMI error: {0}")]
    WMIError(#[from] wmi::WMIError),

    /// GPU driver error
    #[error("GPU driver error: {0}")]
    GPUDriverError(String),

    /// Invalid hardware configuration
    #[error("Invalid hardware configuration: {0}")]
    InvalidConfiguration(String),

    /// Monitoring error
    #[error("Monitoring error: {0}")]
    MonitoringError(String),

    /// Power management error
    #[error("Power management error: {0}")]
    PowerManagementError(String),

    /// Virtualization detection error
    #[error("Virtualization detection error: {0}")]
    VirtualizationError(String),

    /// Thermal management error
    #[error("Thermal management error: {0}")]
    ThermalError(String),

    /// Unknown error
    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl HardwareQueryError {
    pub fn system_info_unavailable(msg: impl Into<String>) -> Self {
        Self::SystemInfoUnavailable(msg.into())
    }

    pub fn device_not_found(msg: impl Into<String>) -> Self {
        Self::DeviceNotFound(msg.into())
    }

    pub fn platform_not_supported(msg: impl Into<String>) -> Self {
        Self::PlatformNotSupported(msg.into())
    }

    pub fn permission_denied(msg: impl Into<String>) -> Self {
        Self::PermissionDenied(msg.into())
    }

    pub fn gpu_driver_error(msg: impl Into<String>) -> Self {
        Self::GPUDriverError(msg.into())
    }

    pub fn invalid_configuration(msg: impl Into<String>) -> Self {
        Self::InvalidConfiguration(msg.into())
    }

    pub fn monitoring_error(msg: impl Into<String>) -> Self {
        Self::MonitoringError(msg.into())
    }

    pub fn power_management_error(msg: impl Into<String>) -> Self {
        Self::PowerManagementError(msg.into())
    }

    pub fn virtualization_error(msg: impl Into<String>) -> Self {
        Self::VirtualizationError(msg.into())
    }

    pub fn thermal_error(msg: impl Into<String>) -> Self {
        Self::ThermalError(msg.into())
    }

    pub fn unknown(msg: impl Into<String>) -> Self {
        Self::Unknown(msg.into())
    }
}
