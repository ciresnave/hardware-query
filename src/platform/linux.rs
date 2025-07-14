/// Enhanced platform-specific hardware detection for Linux
use crate::{HardwareQueryError, Result};
use std::collections::HashMap;
use std::fs;
use std::process::Command;

/// Linux-specific CPU information
#[derive(Debug, Clone)]
pub struct LinuxCPUInfo {
    pub vendor_id: String,
    pub model_name: String,
    pub cpu_family: Option<u32>,
    pub model: Option<u32>,
    pub stepping: Option<u32>,
    pub microcode: Option<String>,
    pub cpu_cores: u32,
    pub siblings: u32,
    pub core_id: Vec<u32>,
    pub apicid: Vec<u32>,
    pub initial_apicid: Vec<u32>,
    pub cpu_mhz: f32,
    pub cache_size: u32,
    pub physical_id: Vec<u32>,
    pub flags: Vec<String>,
    pub bogomips: f32,
    pub clflush_size: Option<u32>,
    pub cache_alignment: Option<u32>,
    pub address_sizes: Option<String>,
    pub power_management: Option<String>,
    pub vulnerabilities: Vec<String>,
}

impl LinuxCPUInfo {
    /// Query detailed CPU information from Linux /proc and /sys
    pub fn query() -> Result<Self> {
        let cpuinfo_content = fs::read_to_string("/proc/cpuinfo").map_err(|e| {
            HardwareQueryError::system_info_unavailable(format!("Cannot read /proc/cpuinfo: {}", e))
        })?;

        let mut cpu_info = LinuxCPUInfo {
            vendor_id: String::new(),
            model_name: String::new(),
            cpu_family: None,
            model: None,
            stepping: None,
            microcode: None,
            cpu_cores: 0,
            siblings: 0,
            core_id: Vec::new(),
            apicid: Vec::new(),
            initial_apicid: Vec::new(),
            cpu_mhz: 0.0,
            cache_size: 0,
            physical_id: Vec::new(),
            flags: Vec::new(),
            bogomips: 0.0,
            clflush_size: None,
            cache_alignment: None,
            address_sizes: None,
            power_management: None,
            vulnerabilities: Vec::new(),
        };

        // Parse /proc/cpuinfo
        for line in cpuinfo_content.lines() {
            if let Some((key, value)) = line.split_once(':') {
                let key = key.trim();
                let value = value.trim();

                match key {
                    "vendor_id" => cpu_info.vendor_id = value.to_string(),
                    "model name" => cpu_info.model_name = value.to_string(),
                    "cpu family" => cpu_info.cpu_family = value.parse().ok(),
                    "model" => cpu_info.model = value.parse().ok(),
                    "stepping" => cpu_info.stepping = value.parse().ok(),
                    "microcode" => cpu_info.microcode = Some(value.to_string()),
                    "cpu cores" => cpu_info.cpu_cores = value.parse().unwrap_or(0),
                    "siblings" => cpu_info.siblings = value.parse().unwrap_or(0),
                    "core id" => cpu_info.core_id.push(value.parse().unwrap_or(0)),
                    "apicid" => cpu_info.apicid.push(value.parse().unwrap_or(0)),
                    "initial apicid" => cpu_info.initial_apicid.push(value.parse().unwrap_or(0)),
                    "cpu MHz" => cpu_info.cpu_mhz = value.parse().unwrap_or(0.0),
                    "cache size" => {
                        // Parse cache size like "8192 KB"
                        if let Some(size_str) = value.split_whitespace().next() {
                            cpu_info.cache_size = size_str.parse().unwrap_or(0);
                        }
                    }
                    "physical id" => cpu_info.physical_id.push(value.parse().unwrap_or(0)),
                    "flags" => {
                        cpu_info.flags = value.split_whitespace().map(|s| s.to_string()).collect();
                    }
                    "bogomips" => cpu_info.bogomips = value.parse().unwrap_or(0.0),
                    "clflush size" => cpu_info.clflush_size = value.parse().ok(),
                    "cache_alignment" => cpu_info.cache_alignment = value.parse().ok(),
                    "address sizes" => cpu_info.address_sizes = Some(value.to_string()),
                    "power management" => cpu_info.power_management = Some(value.to_string()),
                    _ => {}
                }
            }
        }

        // Get vulnerabilities
        cpu_info.vulnerabilities = Self::get_vulnerabilities()?;

        Ok(cpu_info)
    }

    /// Get CPU vulnerabilities from /sys/devices/system/cpu/vulnerabilities/
    pub fn get_vulnerabilities() -> Result<Vec<String>> {
        let mut vulnerabilities = Vec::new();

        if let Ok(entries) = fs::read_dir("/sys/devices/system/cpu/vulnerabilities") {
            for entry in entries.flatten() {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_file() {
                        if let Some(vuln_name) = entry.file_name().to_str() {
                            if let Ok(status) = fs::read_to_string(entry.path()) {
                                let status = status.trim();
                                if !status.starts_with("Not affected")
                                    && !status.starts_with("Mitigation")
                                {
                                    vulnerabilities.push(format!("{}: {}", vuln_name, status));
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(vulnerabilities)
    }

    /// Get cache information from /sys/devices/system/cpu/cpu0/cache/
    pub fn get_cache_info() -> Result<HashMap<String, u32>> {
        let mut cache_info = HashMap::new();

        for level in 1..=3 {
            for cache_type in &["data", "instruction", "unified"] {
                let cache_path = format!("/sys/devices/system/cpu/cpu0/cache/index{}/", level);

                // Check if this cache level exists
                if let Ok(type_content) = fs::read_to_string(format!("{}type", cache_path)) {
                    if type_content.trim() == *cache_type || type_content.trim() == "Unified" {
                        if let Ok(size_content) = fs::read_to_string(format!("{}size", cache_path))
                        {
                            let size_str = size_content.trim().replace("K", "");
                            if let Ok(size) = size_str.parse::<u32>() {
                                let key = format!("L{} {}", level, cache_type);
                                cache_info.insert(key, size);
                            }
                        }
                    }
                }
            }
        }

        Ok(cache_info)
    }

    /// Get CPU frequency scaling information
    pub fn get_frequency_info() -> Result<HashMap<String, u32>> {
        let mut freq_info = HashMap::new();

        // Try to get scaling frequencies
        if let Ok(min_freq) =
            fs::read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/cpuinfo_min_freq")
        {
            if let Ok(freq) = min_freq.trim().parse::<u32>() {
                freq_info.insert("min_frequency_khz".to_string(), freq);
            }
        }

        if let Ok(max_freq) =
            fs::read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/cpuinfo_max_freq")
        {
            if let Ok(freq) = max_freq.trim().parse::<u32>() {
                freq_info.insert("max_frequency_khz".to_string(), freq);
            }
        }

        if let Ok(cur_freq) =
            fs::read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/scaling_cur_freq")
        {
            if let Ok(freq) = cur_freq.trim().parse::<u32>() {
                freq_info.insert("current_frequency_khz".to_string(), freq);
            }
        }

        Ok(freq_info)
    }

    /// Get CPU temperature from sensors
    pub fn get_temperature() -> Result<Option<f32>> {
        // Try different thermal sensor locations
        let thermal_paths = [
            "/sys/class/thermal/thermal_zone0/temp",
            "/sys/class/hwmon/hwmon0/temp1_input",
            "/sys/class/hwmon/hwmon1/temp1_input",
        ];

        for path in &thermal_paths {
            if let Ok(temp_str) = fs::read_to_string(path) {
                if let Ok(temp) = temp_str.trim().parse::<i32>() {
                    // Convert from millidegrees to degrees
                    return Ok(Some(temp as f32 / 1000.0));
                }
            }
        }

        // Try using lm-sensors
        if let Ok(output) = Command::new("sensors").arg("-A").arg("-u").output() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            for line in output_str.lines() {
                if line.contains("temp1_input") || line.contains("Core 0") {
                    if let Some(temp_str) = line.split(':').nth(1) {
                        if let Ok(temp) = temp_str.trim().parse::<f32>() {
                            return Ok(Some(temp));
                        }
                    }
                }
            }
        }

        Ok(None)
    }
}

/// Linux-specific GPU information
#[derive(Debug, Clone)]
pub struct LinuxGPUInfo {
    pub device_name: String,
    pub vendor_name: String,
    pub driver: String,
    pub pci_id: String,
    pub memory_info: Option<String>,
    pub driver_version: Option<String>,
}

impl LinuxGPUInfo {
    /// Query GPU information from Linux /sys and lspci
    pub fn query_all() -> Result<Vec<Self>> {
        let mut gpus = Vec::new();

        // Use lspci to get GPU information
        if let Ok(output) = Command::new("lspci")
            .args(["-v", "-s", "$(lspci | grep VGA | cut -d' ' -f1)"])
            .output()
        {
            let output_str = String::from_utf8_lossy(&output.stdout);
            // Parse lspci output - this is a simplified implementation
            // A full implementation would parse the complete lspci output
        }

        // Try to detect NVIDIA GPUs using nvidia-smi
        if let Ok(nvidia_gpus) = Self::query_nvidia_gpus() {
            gpus.extend(nvidia_gpus);
        }

        // Try to detect AMD GPUs using rocm-smi
        if let Ok(amd_gpus) = Self::query_amd_gpus() {
            gpus.extend(amd_gpus);
        }

        Ok(gpus)
    }

    /// Query NVIDIA GPU information using nvidia-smi
    fn query_nvidia_gpus() -> Result<Vec<Self>> {
        let mut gpus = Vec::new();

        if let Ok(output) = Command::new("nvidia-smi")
            .args([
                "--query-gpu=name,driver_version,memory.total,pci.bus_id",
                "--format=csv,noheader,nounits",
            ])
            .output()
        {
            let output_str = String::from_utf8_lossy(&output.stdout);
            for line in output_str.lines() {
                let parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
                if parts.len() >= 4 {
                    gpus.push(Self {
                        device_name: parts[0].to_string(),
                        vendor_name: "NVIDIA".to_string(),
                        driver: "nvidia".to_string(),
                        pci_id: parts[3].to_string(),
                        memory_info: Some(format!("{} MB", parts[2])),
                        driver_version: Some(parts[1].to_string()),
                    });
                }
            }
        }

        Ok(gpus)
    }

    /// Query AMD GPU information using rocm-smi
    fn query_amd_gpus() -> Result<Vec<Self>> {
        let mut gpus = Vec::new();

        if let Ok(output) = Command::new("rocm-smi")
            .args(["--showproductname", "--showdriverversion"])
            .output()
        {
            let output_str = String::from_utf8_lossy(&output.stdout);
            // Parse rocm-smi output - simplified implementation
            for line in output_str.lines() {
                if line.contains("Card series") {
                    // Extract GPU information from rocm-smi output
                    // This would need more sophisticated parsing
                }
            }
        }

        Ok(gpus)
    }
}

/// Linux-specific memory information
#[derive(Debug, Clone)]
pub struct LinuxMemoryInfo {
    pub mem_total_kb: u64,
    pub mem_free_kb: u64,
    pub mem_available_kb: u64,
    pub buffers_kb: u64,
    pub cached_kb: u64,
    pub swap_total_kb: u64,
    pub swap_free_kb: u64,
    pub modules: Vec<LinuxMemoryModule>,
}

#[derive(Debug, Clone)]
pub struct LinuxMemoryModule {
    pub size_mb: u64,
    pub speed_mhz: Option<u32>,
    pub memory_type: String,
    pub locator: String,
}

impl LinuxMemoryInfo {
    /// Query memory information from Linux /proc/meminfo and dmidecode
    pub fn query() -> Result<Self> {
        let meminfo_content = fs::read_to_string("/proc/meminfo").map_err(|e| {
            HardwareQueryError::system_info_unavailable(format!("Cannot read /proc/meminfo: {}", e))
        })?;

        let mut mem_info = LinuxMemoryInfo {
            mem_total_kb: 0,
            mem_free_kb: 0,
            mem_available_kb: 0,
            buffers_kb: 0,
            cached_kb: 0,
            swap_total_kb: 0,
            swap_free_kb: 0,
            modules: Vec::new(),
        };

        // Parse /proc/meminfo
        for line in meminfo_content.lines() {
            if let Some((key, value)) = line.split_once(':') {
                let key = key.trim();
                let value = value.trim().replace(" kB", "");

                if let Ok(val) = value.parse::<u64>() {
                    match key {
                        "MemTotal" => mem_info.mem_total_kb = val,
                        "MemFree" => mem_info.mem_free_kb = val,
                        "MemAvailable" => mem_info.mem_available_kb = val,
                        "Buffers" => mem_info.buffers_kb = val,
                        "Cached" => mem_info.cached_kb = val,
                        "SwapTotal" => mem_info.swap_total_kb = val,
                        "SwapFree" => mem_info.swap_free_kb = val,
                        _ => {}
                    }
                }
            }
        }

        // Get memory module information using dmidecode
        mem_info.modules = Self::get_memory_modules()?;

        Ok(mem_info)
    }

    /// Get memory module information using dmidecode
    fn get_memory_modules() -> Result<Vec<LinuxMemoryModule>> {
        let mut modules = Vec::new();

        if let Ok(output) = Command::new("dmidecode").args(["-t", "memory"]).output() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            let mut current_module: Option<LinuxMemoryModule> = None;

            for line in output_str.lines() {
                let line = line.trim();

                if line.starts_with("Memory Device") {
                    if let Some(module) = current_module.take() {
                        modules.push(module);
                    }
                    current_module = Some(LinuxMemoryModule {
                        size_mb: 0,
                        speed_mhz: None,
                        memory_type: String::new(),
                        locator: String::new(),
                    });
                } else if let Some(ref mut module) = current_module {
                    if let Some((key, value)) = line.split_once(':') {
                        let key = key.trim();
                        let value = value.trim();

                        match key {
                            "Size" => {
                                if value.contains("MB") {
                                    let size_str = value.replace(" MB", "");
                                    module.size_mb = size_str.parse().unwrap_or(0);
                                } else if value.contains("GB") {
                                    let size_str = value.replace(" GB", "");
                                    if let Ok(size_gb) = size_str.parse::<u64>() {
                                        module.size_mb = size_gb * 1024;
                                    }
                                }
                            }
                            "Speed" => {
                                let speed_str = value.replace(" MHz", "");
                                module.speed_mhz = speed_str.parse().ok();
                            }
                            "Type" => module.memory_type = value.to_string(),
                            "Locator" => module.locator = value.to_string(),
                            _ => {}
                        }
                    }
                }
            }

            if let Some(module) = current_module {
                modules.push(module);
            }
        }

        // Filter out empty modules
        modules.retain(|m| m.size_mb > 0);

        Ok(modules)
    }
}
