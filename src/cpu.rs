use crate::{HardwareQueryError, Result};
use serde::{Deserialize, Serialize};
use std::fmt;
use sysinfo::System;

/// CPU vendor information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CPUVendor {
    Intel,
    AMD,
    ARM,
    Apple,
    Unknown(String),
}

impl fmt::Display for CPUVendor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CPUVendor::Intel => write!(f, "Intel"),
            CPUVendor::AMD => write!(f, "AMD"),
            CPUVendor::ARM => write!(f, "ARM"),
            CPUVendor::Apple => write!(f, "Apple"),
            CPUVendor::Unknown(name) => write!(f, "{name}"),
        }
    }
}

/// CPU feature flags
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CPUFeature {
    AVX,
    AVX2,
    AVX512,
    SSE,
    SSE2,
    SSE3,
    SSE41,
    SSE42,
    FMA,
    AES,
    SHA,
    BMI1,
    BMI2,
    RDRAND,
    RDSEED,
    POPCNT,
    LZCNT,
    MOVBE,
    PREFETCHWT1,
    CLFLUSHOPT,
    CLWB,
    XSAVE,
    XSAVEOPT,
    XSAVEC,
    XSAVES,
    FSGSBASE,
    RDTSCP,
    F16C,
    Unknown(String),
}

impl fmt::Display for CPUFeature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CPUFeature::AVX => write!(f, "AVX"),
            CPUFeature::AVX2 => write!(f, "AVX2"),
            CPUFeature::AVX512 => write!(f, "AVX512"),
            CPUFeature::SSE => write!(f, "SSE"),
            CPUFeature::SSE2 => write!(f, "SSE2"),
            CPUFeature::SSE3 => write!(f, "SSE3"),
            CPUFeature::SSE41 => write!(f, "SSE4.1"),
            CPUFeature::SSE42 => write!(f, "SSE4.2"),
            CPUFeature::FMA => write!(f, "FMA"),
            CPUFeature::AES => write!(f, "AES"),
            CPUFeature::SHA => write!(f, "SHA"),
            CPUFeature::BMI1 => write!(f, "BMI1"),
            CPUFeature::BMI2 => write!(f, "BMI2"),
            CPUFeature::RDRAND => write!(f, "RDRAND"),
            CPUFeature::RDSEED => write!(f, "RDSEED"),
            CPUFeature::POPCNT => write!(f, "POPCNT"),
            CPUFeature::LZCNT => write!(f, "LZCNT"),
            CPUFeature::MOVBE => write!(f, "MOVBE"),
            CPUFeature::PREFETCHWT1 => write!(f, "PREFETCHWT1"),
            CPUFeature::CLFLUSHOPT => write!(f, "CLFLUSHOPT"),
            CPUFeature::CLWB => write!(f, "CLWB"),
            CPUFeature::XSAVE => write!(f, "XSAVE"),
            CPUFeature::XSAVEOPT => write!(f, "XSAVEOPT"),
            CPUFeature::XSAVEC => write!(f, "XSAVEC"),
            CPUFeature::XSAVES => write!(f, "XSAVES"),
            CPUFeature::FSGSBASE => write!(f, "FSGSBASE"),
            CPUFeature::RDTSCP => write!(f, "RDTSCP"),
            CPUFeature::F16C => write!(f, "F16C"),
            CPUFeature::Unknown(name) => write!(f, "{name}"),
        }
    }
}

/// CPU information and specifications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CPUInfo {
    /// CPU vendor
    pub vendor: CPUVendor,
    /// CPU model name
    pub model_name: String,
    /// CPU brand string
    pub brand: String,
    /// Number of physical cores
    pub physical_cores: u32,
    /// Number of logical cores (threads)
    pub logical_cores: u32,
    /// Base frequency in MHz
    pub base_frequency: u32,
    /// Maximum frequency in MHz
    pub max_frequency: u32,
    /// L1 cache size in KB
    pub l1_cache_kb: u32,
    /// L2 cache size in KB
    pub l2_cache_kb: u32,
    /// L3 cache size in KB
    pub l3_cache_kb: u32,
    /// CPU features supported
    pub features: Vec<CPUFeature>,
    /// Architecture (x86_64, arm64, etc.)
    pub architecture: String,
    /// CPU usage percentage per core
    pub core_usage: Vec<f32>,
    /// CPU temperature in Celsius (if available)
    pub temperature: Option<f32>,
    /// CPU power consumption in watts (if available)
    pub power_consumption: Option<f32>,
    /// CPU stepping
    pub stepping: Option<u32>,
    /// CPU family
    pub family: Option<u32>,
    /// CPU model number
    pub model: Option<u32>,
    /// CPU microcode version
    pub microcode: Option<String>,
    /// CPU vulnerabilities (Spectre, Meltdown, etc.)
    pub vulnerabilities: Vec<String>,
}

impl CPUInfo {
    /// Query CPU information from the system
    pub fn query() -> Result<Self> {
        let mut system = System::new_all();
        system.refresh_all();

        let cpus = system.cpus();
        if cpus.is_empty() {
            return Err(HardwareQueryError::system_info_unavailable(
                "No CPU information available",
            ));
        }

        let cpu = &cpus[0];
        let brand = cpu.brand().to_string();
        let vendor = Self::parse_vendor(&brand);

        Ok(Self {
            vendor,
            model_name: Self::extract_model_name(&brand),
            brand,
            physical_cores: Self::detect_physical_cores()?,
            logical_cores: system.cpus().len() as u32,
            base_frequency: cpu.frequency() as u32,
            max_frequency: Self::detect_max_frequency()?,
            l1_cache_kb: Self::detect_l1_cache()?,
            l2_cache_kb: Self::detect_l2_cache()?,
            l3_cache_kb: Self::detect_l3_cache()?,
            features: Self::detect_features()?,
            architecture: Self::detect_architecture(),
            core_usage: cpus.iter().map(|cpu| cpu.cpu_usage()).collect(),
            temperature: Self::detect_temperature(),
            power_consumption: Self::detect_power_consumption(),
            stepping: Self::detect_stepping()?,
            family: Self::detect_family()?,
            model: Self::detect_model()?,
            microcode: Self::detect_microcode(),
            vulnerabilities: Self::detect_vulnerabilities()?,
        })
    }

    /// Get CPU vendor
    pub fn vendor(&self) -> &CPUVendor {
        &self.vendor
    }

    /// Get CPU model name
    pub fn model_name(&self) -> &str {
        &self.model_name
    }

    /// Get CPU brand string
    pub fn brand(&self) -> &str {
        &self.brand
    }

    /// Get number of physical cores
    pub fn physical_cores(&self) -> u32 {
        self.physical_cores
    }

    /// Get number of logical cores (threads)
    pub fn logical_cores(&self) -> u32 {
        self.logical_cores
    }

    /// Get base frequency in MHz
    pub fn base_frequency(&self) -> u32 {
        self.base_frequency
    }

    /// Get maximum frequency in MHz
    pub fn max_frequency(&self) -> u32 {
        self.max_frequency
    }

    /// Get L1 cache size in KB
    pub fn l1_cache_kb(&self) -> u32 {
        self.l1_cache_kb
    }

    /// Get L2 cache size in KB
    pub fn l2_cache_kb(&self) -> u32 {
        self.l2_cache_kb
    }

    /// Get L3 cache size in KB
    pub fn l3_cache_kb(&self) -> u32 {
        self.l3_cache_kb
    }

    /// Get supported CPU features
    pub fn features(&self) -> &[CPUFeature] {
        &self.features
    }

    /// Check if CPU supports a specific feature
    pub fn has_feature(&self, feature: &str) -> bool {
        self.features
            .iter()
            .any(|f| f.to_string().to_lowercase() == feature.to_lowercase())
    }

    /// Get CPU architecture
    pub fn architecture(&self) -> &str {
        &self.architecture
    }

    /// Get current CPU usage per core
    pub fn core_usage(&self) -> &[f32] {
        &self.core_usage
    }

    /// Get CPU temperature (if available)
    pub fn temperature(&self) -> Option<f32> {
        self.temperature
    }

    /// Get CPU power consumption (if available)
    pub fn power_consumption(&self) -> Option<f32> {
        self.power_consumption
    }

    fn parse_vendor(brand: &str) -> CPUVendor {
        let brand_lower = brand.to_lowercase();
        if brand_lower.contains("intel") {
            CPUVendor::Intel
        } else if brand_lower.contains("amd") {
            CPUVendor::AMD
        } else if brand_lower.contains("arm") {
            CPUVendor::ARM
        } else if brand_lower.contains("apple") {
            CPUVendor::Apple
        } else {
            CPUVendor::Unknown(brand.to_string())
        }
    }

    fn extract_model_name(brand: &str) -> String {
        // Extract model name from brand string
        brand.split('@').next().unwrap_or(brand).trim().to_string()
    }

    fn detect_physical_cores() -> Result<u32> {
        #[cfg(target_os = "windows")]
        {
            Self::detect_physical_cores_windows()
        }
        #[cfg(target_os = "linux")]
        {
            Self::detect_physical_cores_linux()
        }
        #[cfg(target_os = "macos")]
        {
            Self::detect_physical_cores_macos()
        }
        #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
        {
            // Fallback for other platforms
            Ok((num_cpus::get() / 2) as u32)
        }
    }

    fn detect_max_frequency() -> Result<u32> {
        #[cfg(target_os = "windows")]
        {
            Self::detect_max_frequency_windows()
        }
        #[cfg(target_os = "linux")]
        {
            Self::detect_max_frequency_linux()
        }
        #[cfg(target_os = "macos")]
        {
            Self::detect_max_frequency_macos()
        }
        #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
        {
            Ok(3000) // Default fallback
        }
    }

    fn detect_l1_cache() -> Result<u32> {
        #[cfg(target_os = "windows")]
        {
            Self::detect_l1_cache_windows()
        }
        #[cfg(target_os = "linux")]
        {
            Self::detect_l1_cache_linux()
        }
        #[cfg(target_os = "macos")]
        {
            Self::detect_l1_cache_macos()
        }
        #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
        {
            Ok(32) // Default fallback
        }
    }

    fn detect_l2_cache() -> Result<u32> {
        #[cfg(target_os = "windows")]
        {
            Self::detect_l2_cache_windows()
        }
        #[cfg(target_os = "linux")]
        {
            Self::detect_l2_cache_linux()
        }
        #[cfg(target_os = "macos")]
        {
            Self::detect_l2_cache_macos()
        }
        #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
        {
            Ok(256) // Default fallback
        }
    }

    fn detect_l3_cache() -> Result<u32> {
        #[cfg(target_os = "windows")]
        {
            Self::detect_l3_cache_windows()
        }
        #[cfg(target_os = "linux")]
        {
            Self::detect_l3_cache_linux()
        }
        #[cfg(target_os = "macos")]
        {
            Self::detect_l3_cache_macos()
        }
        #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
        {
            Ok(8192) // Default fallback
        }
    }

    fn detect_features() -> Result<Vec<CPUFeature>> {
        #[cfg(target_os = "windows")]
        {
            Self::detect_features_windows()
        }
        #[cfg(target_os = "linux")]
        {
            Self::detect_features_linux()
        }
        #[cfg(target_os = "macos")]
        {
            Self::detect_features_macos()
        }
        #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
        {
            Ok(vec![
                CPUFeature::SSE,
                CPUFeature::SSE2,
                CPUFeature::SSE3,
                CPUFeature::SSE41,
                CPUFeature::SSE42,
                CPUFeature::AVX,
                CPUFeature::AVX2,
            ])
        }
    }

    fn detect_stepping() -> Result<Option<u32>> {
        #[cfg(target_os = "linux")]
        {
            if let Ok(content) = fs::read_to_string("/proc/cpuinfo") {
                for line in content.lines() {
                    if line.starts_with("stepping") {
                        if let Some(value) = line.split(':').nth(1) {
                            return Ok(value.trim().parse().ok());
                        }
                    }
                }
            }
        }
        Ok(None)
    }

    fn detect_family() -> Result<Option<u32>> {
        #[cfg(target_os = "linux")]
        {
            if let Ok(content) = fs::read_to_string("/proc/cpuinfo") {
                for line in content.lines() {
                    if line.starts_with("cpu family") {
                        if let Some(value) = line.split(':').nth(1) {
                            return Ok(value.trim().parse().ok());
                        }
                    }
                }
            }
        }
        Ok(None)
    }

    fn detect_model() -> Result<Option<u32>> {
        #[cfg(target_os = "linux")]
        {
            if let Ok(content) = fs::read_to_string("/proc/cpuinfo") {
                for line in content.lines() {
                    if line.starts_with("model") && !line.starts_with("model name") {
                        if let Some(value) = line.split(':').nth(1) {
                            return Ok(value.trim().parse().ok());
                        }
                    }
                }
            }
        }
        Ok(None)
    }

    fn detect_microcode() -> Option<String> {
        #[cfg(target_os = "linux")]
        {
            if let Ok(content) = fs::read_to_string("/proc/cpuinfo") {
                for line in content.lines() {
                    if line.starts_with("microcode") {
                        if let Some(value) = line.split(':').nth(1) {
                            return Some(value.trim().to_string());
                        }
                    }
                }
            }
        }
        None
    }

    fn detect_vulnerabilities() -> Result<Vec<String>> {
        let vulnerabilities = Vec::new();

        #[cfg(target_os = "linux")]
        {
            if let Ok(entries) = fs::read_dir("/sys/devices/system/cpu/vulnerabilities") {
                for entry in entries.flatten() {
                    if let Ok(name) = entry.file_name().into_string() {
                        if let Ok(status) = fs::read_to_string(entry.path()) {
                            let status = status.trim();
                            if !status.starts_with("Not affected")
                                && !status.starts_with("Mitigation")
                            {
                                vulnerabilities.push(format!("{}: {}", name, status));
                            }
                        }
                    }
                }
            }
        }

        Ok(vulnerabilities)
    }

    // Windows-specific implementations
    #[cfg(target_os = "windows")]
    fn detect_physical_cores_windows() -> Result<u32> {
        match wmi::WMIConnection::new(wmi::COMLibrary::new()?) {
            Ok(wmi_con) => {
                let results: Vec<std::collections::HashMap<String, wmi::Variant>> = wmi_con
                    .raw_query("SELECT NumberOfCores FROM Win32_Processor")
                    .map_err(|e| {
                        HardwareQueryError::system_info_unavailable(format!(
                            "WMI query failed: {e}"
                        ))
                    })?;

                if let Some(result) = results.first() {
                    if let Some(wmi::Variant::UI4(cores)) = result.get("NumberOfCores") {
                        return Ok(*cores);
                    }
                }
                Err(HardwareQueryError::system_info_unavailable(
                    "Could not get core count from WMI",
                ))
            }
            Err(_) => Ok((num_cpus::get() / 2) as u32), // Fallback
        }
    }

    #[cfg(target_os = "windows")]
    fn detect_max_frequency_windows() -> Result<u32> {
        match wmi::WMIConnection::new(wmi::COMLibrary::new()?) {
            Ok(wmi_con) => {
                let results: Vec<std::collections::HashMap<String, wmi::Variant>> = wmi_con
                    .raw_query("SELECT MaxClockSpeed FROM Win32_Processor")
                    .map_err(|e| {
                        HardwareQueryError::system_info_unavailable(format!(
                            "WMI query failed: {e}"
                        ))
                    })?;

                if let Some(result) = results.first() {
                    if let Some(wmi::Variant::UI4(freq)) = result.get("MaxClockSpeed") {
                        return Ok(*freq);
                    }
                }
                Ok(3000) // Default fallback
            }
            Err(_) => Ok(3000), // Fallback
        }
    }

    #[cfg(target_os = "windows")]
    fn detect_l1_cache_windows() -> Result<u32> {
        match wmi::WMIConnection::new(wmi::COMLibrary::new()?) {
            Ok(wmi_con) => {
                let results: Vec<std::collections::HashMap<String, wmi::Variant>> = wmi_con
                    .raw_query("SELECT MaxCacheSize FROM Win32_CacheMemory WHERE Level = 3")
                    .map_err(|e| {
                        HardwareQueryError::system_info_unavailable(format!(
                            "WMI query failed: {e}"
                        ))
                    })?;

                if let Some(result) = results.first() {
                    if let Some(wmi::Variant::UI4(size)) = result.get("MaxCacheSize") {
                        return Ok(*size);
                    }
                }
                Ok(32) // Default fallback
            }
            Err(_) => Ok(32), // Fallback
        }
    }

    #[cfg(target_os = "windows")]
    fn detect_l2_cache_windows() -> Result<u32> {
        Ok(256) // Placeholder - would need more complex WMI query
    }

    #[cfg(target_os = "windows")]
    fn detect_l3_cache_windows() -> Result<u32> {
        Ok(8192) // Placeholder - would need more complex WMI query
    }

    #[cfg(target_os = "windows")]
    fn detect_features_windows() -> Result<Vec<CPUFeature>> {
        // Windows feature detection would use CPUID instructions
        // This is a simplified implementation
        Ok(vec![
            CPUFeature::SSE,
            CPUFeature::SSE2,
            CPUFeature::SSE3,
            CPUFeature::SSE41,
            CPUFeature::SSE42,
            CPUFeature::AVX,
            CPUFeature::AVX2,
        ])
    }

    // Linux-specific implementations
    #[cfg(target_os = "linux")]
    fn detect_physical_cores_linux() -> Result<u32> {
        if let Ok(content) = fs::read_to_string("/proc/cpuinfo") {
            let mut core_ids = std::collections::HashSet::new();
            for line in content.lines() {
                if line.starts_with("core id") {
                    if let Some(value) = line.split(':').nth(1) {
                        if let Ok(id) = value.trim().parse::<u32>() {
                            core_ids.insert(id);
                        }
                    }
                }
            }
            if !core_ids.is_empty() {
                return Ok(core_ids.len() as u32);
            }
        }
        Ok((num_cpus::get() / 2) as u32) // Fallback
    }

    #[cfg(target_os = "linux")]
    fn detect_max_frequency_linux() -> Result<u32> {
        if let Ok(content) = fs::read_to_string("/proc/cpuinfo") {
            for line in content.lines() {
                if line.starts_with("cpu MHz") {
                    if let Some(value) = line.split(':').nth(1) {
                        if let Ok(freq) = value.trim().parse::<f32>() {
                            return Ok(freq as u32);
                        }
                    }
                }
            }
        }

        // Try reading from scaling_max_freq
        if let Ok(content) =
            fs::read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/scaling_max_freq")
        {
            if let Ok(freq) = content.trim().parse::<u32>() {
                return Ok(freq / 1000); // Convert from kHz to MHz
            }
        }

        Ok(3000) // Default fallback
    }

    #[cfg(target_os = "linux")]
    fn detect_l1_cache_linux() -> Result<u32> {
        if let Ok(content) = fs::read_to_string("/sys/devices/system/cpu/cpu0/cache/index0/size") {
            if let Ok(size_str) = content.trim().parse::<String>() {
                if let Ok(size) = size_str.trim_end_matches('K').parse::<u32>() {
                    return Ok(size);
                }
            }
        }
        Ok(32) // Default fallback
    }

    #[cfg(target_os = "linux")]
    fn detect_l2_cache_linux() -> Result<u32> {
        if let Ok(content) = fs::read_to_string("/sys/devices/system/cpu/cpu0/cache/index1/size") {
            if let Ok(size_str) = content.trim().parse::<String>() {
                if let Ok(size) = size_str.trim_end_matches('K').parse::<u32>() {
                    return Ok(size);
                }
            }
        }
        Ok(256) // Default fallback
    }

    #[cfg(target_os = "linux")]
    fn detect_l3_cache_linux() -> Result<u32> {
        if let Ok(content) = fs::read_to_string("/sys/devices/system/cpu/cpu0/cache/index2/size") {
            if let Ok(size_str) = content.trim().parse::<String>() {
                if let Ok(size) = size_str.trim_end_matches('K').parse::<u32>() {
                    return Ok(size);
                }
            }
        }
        Ok(8192) // Default fallback
    }

    #[cfg(target_os = "linux")]
    fn detect_features_linux() -> Result<Vec<CPUFeature>> {
        let mut features = Vec::new();

        if let Ok(content) = fs::read_to_string("/proc/cpuinfo") {
            for line in content.lines() {
                if line.starts_with("flags") {
                    if let Some(flags_str) = line.split(':').nth(1) {
                        let flags: Vec<&str> = flags_str.split_whitespace().collect();

                        for flag in flags {
                            match flag {
                                "sse" => features.push(CPUFeature::SSE),
                                "sse2" => features.push(CPUFeature::SSE2),
                                "sse3" => features.push(CPUFeature::SSE3),
                                "sse4_1" => features.push(CPUFeature::SSE41),
                                "sse4_2" => features.push(CPUFeature::SSE42),
                                "avx" => features.push(CPUFeature::AVX),
                                "avx2" => features.push(CPUFeature::AVX2),
                                "avx512f" => features.push(CPUFeature::AVX512),
                                "fma" => features.push(CPUFeature::FMA),
                                "aes" => features.push(CPUFeature::AES),
                                "sha_ni" => features.push(CPUFeature::SHA),
                                "bmi1" => features.push(CPUFeature::BMI1),
                                "bmi2" => features.push(CPUFeature::BMI2),
                                "rdrand" => features.push(CPUFeature::RDRAND),
                                "rdseed" => features.push(CPUFeature::RDSEED),
                                "popcnt" => features.push(CPUFeature::POPCNT),
                                "lzcnt" => features.push(CPUFeature::LZCNT),
                                _ => {}
                            }
                        }
                        break;
                    }
                }
            }
        }

        Ok(features)
    }

    // macOS-specific implementations
    #[cfg(target_os = "macos")]
    fn detect_physical_cores_macos() -> Result<u32> {
        use std::process::Command;

        let output = Command::new("sysctl")
            .arg("-n")
            .arg("hw.physicalcpu")
            .output()
            .map_err(|e| {
                HardwareQueryError::system_info_unavailable(format!("sysctl failed: {}", e))
            })?;

        if output.status.success() {
            let cores_str = String::from_utf8_lossy(&output.stdout);
            if let Ok(cores) = cores_str.trim().parse::<u32>() {
                return Ok(cores);
            }
        }

        Ok((num_cpus::get() / 2) as u32) // Fallback
    }

    #[cfg(target_os = "macos")]
    fn detect_max_frequency_macos() -> Result<u32> {
        use std::process::Command;

        let output = Command::new("sysctl")
            .arg("-n")
            .arg("hw.cpufrequency_max")
            .output()
            .map_err(|e| {
                HardwareQueryError::system_info_unavailable(format!("sysctl failed: {}", e))
            })?;

        if output.status.success() {
            let freq_str = String::from_utf8_lossy(&output.stdout);
            if let Ok(freq) = freq_str.trim().parse::<u64>() {
                return Ok((freq / 1_000_000) as u32); // Convert Hz to MHz
            }
        }

        Ok(3000) // Default fallback
    }

    #[cfg(target_os = "macos")]
    fn detect_l1_cache_macos() -> Result<u32> {
        use std::process::Command;

        let output = Command::new("sysctl")
            .arg("-n")
            .arg("hw.l1dcachesize")
            .output()
            .map_err(|e| {
                HardwareQueryError::system_info_unavailable(format!("sysctl failed: {}", e))
            })?;

        if output.status.success() {
            let cache_str = String::from_utf8_lossy(&output.stdout);
            if let Ok(cache) = cache_str.trim().parse::<u32>() {
                return Ok(cache / 1024); // Convert bytes to KB
            }
        }

        Ok(32) // Default fallback
    }

    #[cfg(target_os = "macos")]
    fn detect_l2_cache_macos() -> Result<u32> {
        use std::process::Command;

        let output = Command::new("sysctl")
            .arg("-n")
            .arg("hw.l2cachesize")
            .output()
            .map_err(|e| {
                HardwareQueryError::system_info_unavailable(format!("sysctl failed: {}", e))
            })?;

        if output.status.success() {
            let cache_str = String::from_utf8_lossy(&output.stdout);
            if let Ok(cache) = cache_str.trim().parse::<u32>() {
                return Ok(cache / 1024); // Convert bytes to KB
            }
        }

        Ok(256) // Default fallback
    }

    #[cfg(target_os = "macos")]
    fn detect_l3_cache_macos() -> Result<u32> {
        use std::process::Command;

        let output = Command::new("sysctl")
            .arg("-n")
            .arg("hw.l3cachesize")
            .output()
            .map_err(|e| {
                HardwareQueryError::system_info_unavailable(format!("sysctl failed: {}", e))
            })?;

        if output.status.success() {
            let cache_str = String::from_utf8_lossy(&output.stdout);
            if let Ok(cache) = cache_str.trim().parse::<u32>() {
                return Ok(cache / 1024); // Convert bytes to KB
            }
        }

        Ok(8192) // Default fallback
    }

    #[cfg(target_os = "macos")]
    fn detect_features_macos() -> Result<Vec<CPUFeature>> {
        use std::process::Command;

        let mut features = Vec::new();

        // Check for various CPU features using sysctl
        let feature_checks = vec![
            ("hw.optional.sse", CPUFeature::SSE),
            ("hw.optional.sse2", CPUFeature::SSE2),
            ("hw.optional.sse3", CPUFeature::SSE3),
            ("hw.optional.sse4_1", CPUFeature::SSE41),
            ("hw.optional.sse4_2", CPUFeature::SSE42),
            ("hw.optional.avx1_0", CPUFeature::AVX),
            ("hw.optional.avx2_0", CPUFeature::AVX2),
            ("hw.optional.aes", CPUFeature::AES),
        ];

        for (sysctl_name, feature) in feature_checks {
            let output = Command::new("sysctl").arg("-n").arg(sysctl_name).output();

            if let Ok(output) = output {
                if output.status.success() {
                    let value_str = String::from_utf8_lossy(&output.stdout);
                    if value_str.trim() == "1" {
                        features.push(feature);
                    }
                }
            }
        }

        Ok(features)
    }

    fn detect_architecture() -> String {
        if cfg!(target_arch = "x86_64") {
            "x86_64".to_string()
        } else if cfg!(target_arch = "aarch64") {
            "aarch64".to_string()
        } else if cfg!(target_arch = "arm") {
            "arm".to_string()
        } else {
            std::env::consts::ARCH.to_string()
        }
    }

    fn detect_temperature() -> Option<f32> {
        // Platform-specific implementation would go here
        None
    }

    fn detect_power_consumption() -> Option<f32> {
        // Platform-specific implementation would go here
        None
    }
}
