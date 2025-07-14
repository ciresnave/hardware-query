/// Enhanced platform-specific hardware detection for macOS
use crate::{HardwareQueryError, Result};
use std::collections::HashMap;
use std::process::Command;

/// macOS-specific CPU information
#[derive(Debug, Clone)]
pub struct MacOSCPUInfo {
    pub brand_string: String,
    pub vendor: String,
    pub cpu_type: String,
    pub cpu_subtype: String,
    pub physical_cpu: u32,
    pub logical_cpu: u32,
    pub cpu_freq: u64,
    pub cpu_freq_max: u64,
    pub cpu_freq_min: u64,
    pub l1_icache_size: u64,
    pub l1_dcache_size: u64,
    pub l2_cache_size: u64,
    pub l3_cache_size: u64,
    pub cache_line_size: u64,
    pub features: Vec<String>,
}

impl MacOSCPUInfo {
    /// Query detailed CPU information from macOS sysctl
    pub fn query() -> Result<Self> {
        let mut cpu_info = MacOSCPUInfo {
            brand_string: String::new(),
            vendor: String::new(),
            cpu_type: String::new(),
            cpu_subtype: String::new(),
            physical_cpu: 0,
            logical_cpu: 0,
            cpu_freq: 0,
            cpu_freq_max: 0,
            cpu_freq_min: 0,
            l1_icache_size: 0,
            l1_dcache_size: 0,
            l2_cache_size: 0,
            l3_cache_size: 0,
            cache_line_size: 0,
            features: Vec::new(),
        };

        // Get CPU information using sysctl
        let sysctl_queries = [
            ("machdep.cpu.brand_string", "brand_string"),
            ("machdep.cpu.vendor", "vendor"),
            ("hw.cputype", "cpu_type"),
            ("hw.cpusubtype", "cpu_subtype"),
            ("hw.physicalcpu", "physical_cpu"),
            ("hw.logicalcpu", "logical_cpu"),
            ("hw.cpufrequency", "cpu_freq"),
            ("hw.cpufrequency_max", "cpu_freq_max"),
            ("hw.cpufrequency_min", "cpu_freq_min"),
            ("hw.l1icachesize", "l1_icache_size"),
            ("hw.l1dcachesize", "l1_dcache_size"),
            ("hw.l2cachesize", "l2_cache_size"),
            ("hw.l3cachesize", "l3_cache_size"),
            ("hw.cachelinesize", "cache_line_size"),
        ];

        for (sysctl_key, field_name) in &sysctl_queries {
            if let Ok(output) = Command::new("sysctl").args(["-n", sysctl_key]).output() {
                let value = String::from_utf8_lossy(&output.stdout).trim().to_string();

                match *field_name {
                    "brand_string" => cpu_info.brand_string = value,
                    "vendor" => cpu_info.vendor = value,
                    "cpu_type" => cpu_info.cpu_type = value,
                    "cpu_subtype" => cpu_info.cpu_subtype = value,
                    "physical_cpu" => cpu_info.physical_cpu = value.parse().unwrap_or(0),
                    "logical_cpu" => cpu_info.logical_cpu = value.parse().unwrap_or(0),
                    "cpu_freq" => cpu_info.cpu_freq = value.parse().unwrap_or(0),
                    "cpu_freq_max" => cpu_info.cpu_freq_max = value.parse().unwrap_or(0),
                    "cpu_freq_min" => cpu_info.cpu_freq_min = value.parse().unwrap_or(0),
                    "l1_icache_size" => cpu_info.l1_icache_size = value.parse().unwrap_or(0),
                    "l1_dcache_size" => cpu_info.l1_dcache_size = value.parse().unwrap_or(0),
                    "l2_cache_size" => cpu_info.l2_cache_size = value.parse().unwrap_or(0),
                    "l3_cache_size" => cpu_info.l3_cache_size = value.parse().unwrap_or(0),
                    "cache_line_size" => cpu_info.cache_line_size = value.parse().unwrap_or(0),
                    _ => {}
                }
            }
        }

        // Get CPU features
        cpu_info.features = Self::get_cpu_features()?;

        Ok(cpu_info)
    }

    /// Get CPU features from macOS sysctl
    fn get_cpu_features() -> Result<Vec<String>> {
        let mut features = Vec::new();

        let feature_queries = [
            "machdep.cpu.features",
            "machdep.cpu.leaf7_features",
            "machdep.cpu.extfeatures",
        ];

        for query in &feature_queries {
            if let Ok(output) = Command::new("sysctl").args(["-n", query]).output() {
                let feature_str = String::from_utf8_lossy(&output.stdout);
                let query_features: Vec<String> = feature_str
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .collect();
                features.extend(query_features);
            }
        }

        Ok(features)
    }

    /// Get CPU temperature using powermetrics (requires sudo)
    pub fn get_temperature() -> Result<Option<f32>> {
        // Try to get temperature using powermetrics
        if let Ok(output) = Command::new("powermetrics")
            .args([
                "--samplers",
                "smc",
                "-n",
                "1",
                "--hide-cpu-duty-cycle",
                "--show-process-coalition",
            ])
            .output()
        {
            let output_str = String::from_utf8_lossy(&output.stdout);
            for line in output_str.lines() {
                if line.contains("CPU die temperature") {
                    if let Some(temp_str) = line.split(':').nth(1) {
                        let temp_str = temp_str.trim().replace("C", "");
                        if let Ok(temp) = temp_str.parse::<f32>() {
                            return Ok(Some(temp));
                        }
                    }
                }
            }
        }

        // Alternative: try using iStat or other tools
        Ok(None)
    }

    /// Get detailed CPU architecture information
    pub fn get_architecture_info() -> Result<HashMap<String, String>> {
        let mut arch_info = HashMap::new();

        let arch_queries = [
            ("hw.targettype", "target_type"),
            ("hw.machine", "machine"),
            ("hw.model", "model"),
            ("machdep.cpu.family", "cpu_family"),
            ("machdep.cpu.model", "cpu_model"),
            ("machdep.cpu.stepping", "stepping"),
            ("machdep.cpu.microcode_version", "microcode_version"),
        ];

        for (sysctl_key, info_key) in &arch_queries {
            if let Ok(output) = Command::new("sysctl").args(["-n", sysctl_key]).output() {
                let value = String::from_utf8_lossy(&output.stdout).trim().to_string();
                arch_info.insert(info_key.to_string(), value);
            }
        }

        Ok(arch_info)
    }
}

/// macOS-specific GPU information
#[derive(Debug, Clone)]
pub struct MacOSGPUInfo {
    pub device_name: String,
    pub vendor_name: String,
    pub device_id: String,
    pub vendor_id: String,
    pub pci_class: String,
    pub metal_support: bool,
    pub memory_mb: u64,
}

impl MacOSGPUInfo {
    /// Query GPU information from macOS system_profiler
    pub fn query_all() -> Result<Vec<Self>> {
        let mut gpus = Vec::new();

        if let Ok(output) = Command::new("system_profiler")
            .args(["SPDisplaysDataType", "-xml"])
            .output()
        {
            let output_str = String::from_utf8_lossy(&output.stdout);
            // Parse system_profiler XML output
            // This is a simplified implementation - a full implementation would parse XML

            if output_str.contains("Graphics/Displays:") {
                // Extract GPU information from system_profiler output
                // This would require proper XML parsing
            }
        }

        // Alternative: use ioreg command
        if let Ok(output) = Command::new("ioreg")
            .args(["-r", "-d", "1", "-w", "0", "-c", "IOPCIDevice"])
            .output()
        {
            let output_str = String::from_utf8_lossy(&output.stdout);
            // Parse ioreg output for GPU devices
            // This is a simplified implementation
        }

        Ok(gpus)
    }

    /// Check Metal support and capabilities
    pub fn get_metal_info() -> Result<HashMap<String, String>> {
        let mut metal_info = HashMap::new();

        // Use Metal command-line tools if available
        if let Ok(output) = Command::new("xcrun").args(["metal", "-version"]).output() {
            let version_str = String::from_utf8_lossy(&output.stdout);
            metal_info.insert("metal_version".to_string(), version_str.trim().to_string());
        }

        Ok(metal_info)
    }
}

/// macOS-specific memory information
#[derive(Debug, Clone)]
pub struct MacOSMemoryInfo {
    pub physical_memory: u64,
    pub user_memory: u64,
    pub wired_memory: u64,
    pub compressed_memory: u64,
    pub memory_pressure: String,
    pub swap_usage: u64,
}

impl MacOSMemoryInfo {
    /// Query memory information from macOS system tools
    pub fn query() -> Result<Self> {
        let mut mem_info = MacOSMemoryInfo {
            physical_memory: 0,
            user_memory: 0,
            wired_memory: 0,
            compressed_memory: 0,
            memory_pressure: String::new(),
            swap_usage: 0,
        };

        // Get physical memory
        if let Ok(output) = Command::new("sysctl").args(["-n", "hw.memsize"]).output() {
            let mem_str = String::from_utf8_lossy(&output.stdout);
            mem_info.physical_memory = mem_str.trim().parse().unwrap_or(0);
        }

        // Get memory usage details
        if let Ok(output) = Command::new("vm_stat").output() {
            let vm_stat_str = String::from_utf8_lossy(&output.stdout);

            for line in vm_stat_str.lines() {
                if let Some((key, value)) = line.split_once(':') {
                    let key = key.trim();
                    let value = value
                        .trim()
                        .replace(".", "")
                        .replace("pages", "")
                        .trim()
                        .to_string();

                    if let Ok(pages) = value.parse::<u64>() {
                        // Assuming 4KB page size
                        let bytes = pages * 4096;

                        match key {
                            "Pages free" => {
                                // Calculate available memory
                            }
                            "Pages wired down" => mem_info.wired_memory = bytes,
                            "Pages active" => mem_info.user_memory += bytes,
                            "Pages inactive" => mem_info.user_memory += bytes,
                            "Pages occupied by compressor" => mem_info.compressed_memory = bytes,
                            _ => {}
                        }
                    }
                }
            }
        }

        // Get memory pressure
        if let Ok(output) = Command::new("memory_pressure").output() {
            let pressure_str = String::from_utf8_lossy(&output.stdout);
            mem_info.memory_pressure = pressure_str.trim().to_string();
        }

        // Get swap usage
        if let Ok(output) = Command::new("sysctl").args(["-n", "vm.swapusage"]).output() {
            let swap_str = String::from_utf8_lossy(&output.stdout);
            // Parse swap usage string
            if let Some(used_start) = swap_str.find("used = ") {
                if let Some(used_end) = swap_str[used_start + 7..].find("M") {
                    let used_str = &swap_str[used_start + 7..used_start + 7 + used_end];
                    if let Ok(used_mb) = used_str.parse::<u64>() {
                        mem_info.swap_usage = used_mb * 1024 * 1024; // Convert to bytes
                    }
                }
            }
        }

        Ok(mem_info)
    }

    /// Get detailed memory module information using system_profiler
    pub fn get_memory_modules() -> Result<Vec<MacOSMemoryModule>> {
        let mut modules = Vec::new();

        if let Ok(output) = Command::new("system_profiler")
            .args(["SPMemoryDataType", "-xml"])
            .output()
        {
            let output_str = String::from_utf8_lossy(&output.stdout);
            // Parse system_profiler XML output for memory modules
            // This would require proper XML parsing
        }

        Ok(modules)
    }
}

#[derive(Debug, Clone)]
pub struct MacOSMemoryModule {
    pub size_gb: u64,
    pub speed_mhz: u32,
    pub memory_type: String,
    pub manufacturer: String,
    pub part_number: String,
    pub serial_number: String,
    pub slot: String,
}

/// macOS-specific system information
pub struct MacOSSystemInfo;

impl MacOSSystemInfo {
    /// Get system version and build information
    pub fn get_system_version() -> Result<HashMap<String, String>> {
        let mut version_info = HashMap::new();

        if let Ok(output) = Command::new("sw_vers").output() {
            let version_str = String::from_utf8_lossy(&output.stdout);

            for line in version_str.lines() {
                if let Some((key, value)) = line.split_once(':') {
                    let key = key.trim();
                    let value = value.trim();

                    match key {
                        "ProductName" => {
                            version_info.insert("product_name".to_string(), value.to_string())
                        }
                        "ProductVersion" => {
                            version_info.insert("product_version".to_string(), value.to_string())
                        }
                        "BuildVersion" => {
                            version_info.insert("build_version".to_string(), value.to_string())
                        }
                        _ => None,
                    };
                }
            }
        }

        Ok(version_info)
    }

    /// Get hardware model information
    pub fn get_hardware_model() -> Result<String> {
        if let Ok(output) = Command::new("sysctl").args(["-n", "hw.model"]).output() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            Err(HardwareQueryError::system_info_unavailable(
                "Cannot get hardware model",
            ))
        }
    }

    /// Get system uptime
    pub fn get_uptime() -> Result<u64> {
        if let Ok(output) = Command::new("sysctl")
            .args(["-n", "kern.boottime"])
            .output()
        {
            let boottime_str = String::from_utf8_lossy(&output.stdout);
            // Parse boottime and calculate uptime
            // This would require parsing the boottime format
            Ok(0) // Placeholder
        } else {
            Err(HardwareQueryError::system_info_unavailable(
                "Cannot get uptime",
            ))
        }
    }
}
