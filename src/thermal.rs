use crate::Result;
use serde::{Deserialize, Serialize};

/// Thermal sensor information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThermalSensor {
    /// Sensor name
    pub name: String,
    /// Current temperature in Celsius
    pub temperature: f32,
    /// Critical temperature threshold
    pub critical_temperature: Option<f32>,
    /// Maximum recorded temperature
    pub max_temperature: Option<f32>,
    /// Sensor type (CPU, GPU, System, etc.)
    pub sensor_type: String,
}

/// Fan information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FanInfo {
    /// Fan name
    pub name: String,
    /// Current fan speed in RPM
    pub speed_rpm: u32,
    /// Maximum fan speed in RPM
    pub max_speed_rpm: Option<u32>,
    /// Fan speed percentage (0-100)
    pub speed_percent: Option<f32>,
    /// Is fan controllable
    pub controllable: bool,
}

/// System thermal information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThermalInfo {
    /// Temperature sensors
    pub sensors: Vec<ThermalSensor>,
    /// System fans
    pub fans: Vec<FanInfo>,
    /// Overall system temperature status
    pub thermal_status: ThermalStatus,
}

/// System thermal status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThermalStatus {
    Normal,
    Warm,
    Hot,
    Critical,
    Unknown,
}

impl std::fmt::Display for ThermalStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ThermalStatus::Normal => write!(f, "Normal"),
            ThermalStatus::Warm => write!(f, "Warm"),
            ThermalStatus::Hot => write!(f, "Hot"),
            ThermalStatus::Critical => write!(f, "Critical"),
            ThermalStatus::Unknown => write!(f, "Unknown"),
        }
    }
}

impl ThermalInfo {
    /// Query thermal information
    pub fn query() -> Result<Self> {
        let sensors = Self::query_sensors()?;
        let fans = Self::query_fans()?;
        let thermal_status = Self::calculate_thermal_status(&sensors);

        Ok(Self {
            sensors,
            fans,
            thermal_status,
        })
    }

    /// Get temperature sensors
    pub fn sensors(&self) -> &[ThermalSensor] {
        &self.sensors
    }

    /// Get system fans
    pub fn fans(&self) -> &[FanInfo] {
        &self.fans
    }

    /// Get thermal status
    pub fn thermal_status(&self) -> &ThermalStatus {
        &self.thermal_status
    }

    /// Get maximum temperature across all sensors
    pub fn max_temperature(&self) -> Option<f32> {
        self.sensors
            .iter()
            .map(|sensor| sensor.temperature)
            .fold(None, |acc, temp| match acc {
                None => Some(temp),
                Some(max_temp) => Some(max_temp.max(temp)),
            })
    }

    /// Get average temperature across all sensors
    pub fn average_temperature(&self) -> Option<f32> {
        if self.sensors.is_empty() {
            None
        } else {
            let total: f32 = self.sensors.iter().map(|sensor| sensor.temperature).sum();
            Some(total / self.sensors.len() as f32)
        }
    }

    /// Check if any sensor is at critical temperature
    pub fn has_critical_temperature(&self) -> bool {
        self.sensors.iter().any(|sensor| {
            if let Some(critical) = sensor.critical_temperature {
                sensor.temperature >= critical
            } else {
                sensor.temperature >= 90.0 // Default critical threshold
            }
        })
    }

    /// Get CPU temperature (if available)
    pub fn cpu_temperature(&self) -> Option<f32> {
        self.sensors
            .iter()
            .find(|sensor| sensor.sensor_type.to_lowercase().contains("cpu"))
            .map(|sensor| sensor.temperature)
    }

    /// Get GPU temperature (if available)
    pub fn gpu_temperature(&self) -> Option<f32> {
        self.sensors
            .iter()
            .find(|sensor| sensor.sensor_type.to_lowercase().contains("gpu"))
            .map(|sensor| sensor.temperature)
    }

    fn query_sensors() -> Result<Vec<ThermalSensor>> {
        // Platform-specific implementation would go here
        // For now, return empty vector
        Ok(vec![])
    }

    fn query_fans() -> Result<Vec<FanInfo>> {
        // Platform-specific implementation would go here
        // For now, return empty vector
        Ok(vec![])
    }

    fn calculate_thermal_status(sensors: &[ThermalSensor]) -> ThermalStatus {
        if sensors.is_empty() {
            return ThermalStatus::Unknown;
        }

        let max_temp = sensors
            .iter()
            .map(|sensor| sensor.temperature)
            .fold(0.0f32, |acc, temp| acc.max(temp));

        if max_temp >= 90.0 {
            ThermalStatus::Critical
        } else if max_temp >= 80.0 {
            ThermalStatus::Hot
        } else if max_temp >= 70.0 {
            ThermalStatus::Warm
        } else {
            ThermalStatus::Normal
        }
    }
}
