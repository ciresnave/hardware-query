use crate::Result;
use serde::{Deserialize, Serialize};
use std::time::Duration;

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
    /// Historical temperature readings (last 10 readings)
    pub temperature_history: Vec<TemperatureReading>,
}

/// Temperature reading with timestamp
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemperatureReading {
    /// Temperature in Celsius
    pub temperature: f32,
    /// Timestamp of the reading
    pub timestamp: std::time::SystemTime,
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
    /// Fan curve settings (if available)
    pub fan_curve: Option<FanCurve>,
}

/// Fan curve configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FanCurve {
    /// Temperature to fan speed mappings
    pub curve_points: Vec<CurvePoint>,
    /// Hysteresis in degrees Celsius
    pub hysteresis: f32,
    /// Minimum fan speed percentage
    pub min_speed_percent: f32,
    /// Maximum fan speed percentage  
    pub max_speed_percent: f32,
}

/// Single point on a fan curve
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurvePoint {
    /// Temperature in Celsius
    pub temperature: f32,
    /// Fan speed percentage (0-100)
    pub speed_percent: f32,
}

/// Thermal throttling prediction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThrottlingPrediction {
    /// Will throttling occur
    pub will_throttle: bool,
    /// Time until throttling occurs (if applicable)
    pub time_to_throttle: Option<Duration>,
    /// Predicted throttling severity
    pub severity: ThrottlingSeverity,
    /// Recommended actions to prevent throttling
    pub recommendations: Vec<String>,
    /// Confidence level of prediction (0.0 to 1.0)
    pub confidence: f64,
}

/// Severity of thermal throttling
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThrottlingSeverity {
    /// No throttling
    None,
    /// Light throttling (< 10% performance loss)
    Light,
    /// Moderate throttling (10-25% performance loss)
    Moderate,
    /// Heavy throttling (25-50% performance loss)
    Heavy,
    /// Severe throttling (> 50% performance loss)
    Severe,
}

impl std::fmt::Display for ThrottlingSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ThrottlingSeverity::None => write!(f, "None"),
            ThrottlingSeverity::Light => write!(f, "Light"),
            ThrottlingSeverity::Moderate => write!(f, "Moderate"),
            ThrottlingSeverity::Heavy => write!(f, "Heavy"),
            ThrottlingSeverity::Severe => write!(f, "Severe"),
        }
    }
}

/// Cooling optimization recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoolingRecommendation {
    /// Recommendation type
    pub recommendation_type: CoolingRecommendationType,
    /// Human-readable description
    pub description: String,
    /// Expected temperature reduction in Celsius
    pub expected_temp_reduction: Option<f32>,
    /// Implementation difficulty
    pub difficulty: ImplementationDifficulty,
    /// Estimated cost category
    pub cost_category: CostCategory,
}

/// Type of cooling recommendation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CoolingRecommendationType {
    /// Fan curve optimization
    FanCurveOptimization,
    /// Thermal paste replacement
    ThermalPasteReplacement,
    /// Additional case fans
    AdditionalFans,
    /// CPU cooler upgrade
    CPUCoolerUpgrade,
    /// GPU cooling solution
    GPUCooling,
    /// Case ventilation improvement
    CaseVentilation,
    /// Undervolting components
    Undervolting,
    /// Workload adjustment
    WorkloadAdjustment,
    /// Environmental changes
    EnvironmentalChanges,
}

/// Implementation difficulty level
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImplementationDifficulty {
    /// Easy to implement (software changes)
    Easy,
    /// Moderate difficulty (minor hardware changes)
    Moderate,
    /// Difficult (major hardware changes)
    Difficult,
    /// Expert level (professional installation recommended)
    Expert,
}

impl std::fmt::Display for ImplementationDifficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImplementationDifficulty::Easy => write!(f, "Easy"),
            ImplementationDifficulty::Moderate => write!(f, "Moderate"),
            ImplementationDifficulty::Difficult => write!(f, "Difficult"),
            ImplementationDifficulty::Expert => write!(f, "Expert"),
        }
    }
}

/// Cost category for recommendations
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CostCategory {
    /// Free (software/settings changes)
    Free,
    /// Low cost (< $50)
    Low,
    /// Medium cost ($50-200)
    Medium,
    /// High cost (> $200)
    High,
}

impl std::fmt::Display for CostCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CostCategory::Free => write!(f, "Free"),
            CostCategory::Low => write!(f, "Low cost"),
            CostCategory::Medium => write!(f, "Medium cost"),
            CostCategory::High => write!(f, "High cost"),
        }
    }
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
    /// Ambient temperature (if available)
    pub ambient_temperature: Option<f32>,
    /// Thermal design power (TDP) information
    pub tdp_info: Option<TDPInfo>,
}

/// Thermal Design Power information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TDPInfo {
    /// CPU TDP in watts
    pub cpu_tdp: Option<f32>,
    /// GPU TDP in watts  
    pub gpu_tdp: Option<f32>,
    /// System TDP in watts
    pub system_tdp: Option<f32>,
    /// Current power consumption vs TDP ratio
    pub power_ratio: Option<f32>,
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
        let ambient_temperature = Self::query_ambient_temperature()?;
        let tdp_info = Self::query_tdp_info()?;

        Ok(Self {
            sensors,
            fans,
            thermal_status,
            ambient_temperature,
            tdp_info,
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

    /// Predict thermal throttling based on current conditions
    pub fn predict_thermal_throttling(&self, workload_intensity: f32) -> ThrottlingPrediction {
        let max_temp = self.max_temperature().unwrap_or(0.0);
        let cpu_temp = self.cpu_temperature().unwrap_or(0.0);
        let gpu_temp = self.gpu_temperature().unwrap_or(0.0);

        // Simple prediction algorithm - would be enhanced with ML models
        let critical_threshold = 90.0;
        let temp_trend = self.calculate_temperature_trend();
        
        // Factor in workload intensity
        let projected_temp_increase = workload_intensity * 10.0; // Simplified calculation
        let projected_max_temp = max_temp + projected_temp_increase + temp_trend;

        let will_throttle = projected_max_temp >= critical_threshold;
        let time_to_throttle = if will_throttle && temp_trend > 0.0 {
            let temp_diff = critical_threshold - max_temp;
            let time_seconds = (temp_diff / temp_trend) * 60.0; // Convert to seconds
            Some(Duration::from_secs(time_seconds.max(0.0) as u64))
        } else {
            None
        };

        let severity = if projected_max_temp >= 95.0 {
            ThrottlingSeverity::Severe
        } else if projected_max_temp >= 90.0 {
            ThrottlingSeverity::Heavy
        } else if projected_max_temp >= 85.0 {
            ThrottlingSeverity::Moderate
        } else if projected_max_temp >= 80.0 {
            ThrottlingSeverity::Light
        } else {
            ThrottlingSeverity::None
        };

        let mut recommendations = Vec::new();
        if will_throttle {
            recommendations.push("Reduce workload intensity".to_string());
            recommendations.push("Increase fan speeds if possible".to_string());
            if cpu_temp > gpu_temp {
                recommendations.push("Focus on CPU cooling optimization".to_string());
            } else if gpu_temp > cpu_temp {
                recommendations.push("Focus on GPU cooling optimization".to_string());
            }
        }

        let confidence = if temp_trend.abs() > 2.0 { 0.8 } else { 0.6 };

        ThrottlingPrediction {
            will_throttle,
            time_to_throttle,
            severity,
            recommendations,
            confidence,
        }
    }

    /// Get cooling optimization recommendations
    pub fn suggest_cooling_optimizations(&self) -> Vec<CoolingRecommendation> {
        let mut recommendations = Vec::new();
        let max_temp = self.max_temperature().unwrap_or(0.0);

        if max_temp > 85.0 {
            // High temperature recommendations
            recommendations.push(CoolingRecommendation {
                recommendation_type: CoolingRecommendationType::FanCurveOptimization,
                description: "Optimize fan curves for better cooling efficiency".to_string(),
                expected_temp_reduction: Some(3.0),
                difficulty: ImplementationDifficulty::Easy,
                cost_category: CostCategory::Free,
            });

            if max_temp > 90.0 {
                recommendations.push(CoolingRecommendation {
                    recommendation_type: CoolingRecommendationType::ThermalPasteReplacement,
                    description: "Consider replacing thermal paste on CPU/GPU".to_string(),
                    expected_temp_reduction: Some(5.0),
                    difficulty: ImplementationDifficulty::Moderate,
                    cost_category: CostCategory::Low,
                });
            }
        }

        // Check fan utilization
        let avg_fan_speed = self.fans
            .iter()
            .filter_map(|fan| fan.speed_percent)
            .fold(0.0, |acc, speed| acc + speed) / self.fans.len().max(1) as f32;

        if avg_fan_speed > 80.0 {
            recommendations.push(CoolingRecommendation {
                recommendation_type: CoolingRecommendationType::AdditionalFans,
                description: "Current fans are running at high speeds. Consider adding more case fans".to_string(),
                expected_temp_reduction: Some(4.0),
                difficulty: ImplementationDifficulty::Moderate,
                cost_category: CostCategory::Low,
            });
        }

        // CPU specific recommendations
        if let Some(cpu_temp) = self.cpu_temperature() {
            if cpu_temp > 85.0 {
                recommendations.push(CoolingRecommendation {
                    recommendation_type: CoolingRecommendationType::CPUCoolerUpgrade,
                    description: "CPU temperatures are high. Consider upgrading CPU cooler".to_string(),
                    expected_temp_reduction: Some(8.0),
                    difficulty: ImplementationDifficulty::Moderate,
                    cost_category: CostCategory::Medium,
                });
            }
        }

        // Environmental recommendations
        if let Some(ambient) = self.ambient_temperature {
            if ambient > 30.0 {
                recommendations.push(CoolingRecommendation {
                    recommendation_type: CoolingRecommendationType::EnvironmentalChanges,
                    description: "High ambient temperature detected. Improve room ventilation or use air conditioning".to_string(),
                    expected_temp_reduction: Some(ambient - 25.0),
                    difficulty: ImplementationDifficulty::Easy,
                    cost_category: CostCategory::Free,
                });
            }
        }

        recommendations
    }

    /// Calculate sustained performance capability considering thermal limits
    pub fn calculate_sustained_performance(&self) -> f64 {
        let max_temp = self.max_temperature().unwrap_or(0.0);
        let critical_temp = 90.0;
        
        if max_temp < 70.0 {
            1.0 // Full performance
        } else if max_temp < 80.0 {
            0.95 // Slight performance reduction
        } else if max_temp < critical_temp {
            0.85 // Moderate performance reduction
        } else {
            0.70 // Significant throttling expected
        }
    }

    /// Update temperature history for a sensor
    pub fn update_sensor_history(&mut self, sensor_name: &str, temperature: f32) {
        if let Some(sensor) = self.sensors.iter_mut().find(|s| s.name == sensor_name) {
            sensor.temperature_history.push(TemperatureReading {
                temperature,
                timestamp: std::time::SystemTime::now(),
            });

            // Keep only last 10 readings
            if sensor.temperature_history.len() > 10 {
                sensor.temperature_history.remove(0);
            }

            // Update current temperature
            sensor.temperature = temperature;
        }
    }

    fn calculate_temperature_trend(&self) -> f32 {
        // Calculate average temperature trend across all sensors
        let mut total_trend = 0.0;
        let mut sensor_count = 0;

        for sensor in &self.sensors {
            if sensor.temperature_history.len() >= 3 {
                let recent_temps: Vec<f32> = sensor.temperature_history
                    .iter()
                    .rev()
                    .take(3)
                    .map(|reading| reading.temperature)
                    .collect();

                // Simple linear trend calculation
                let trend = (recent_temps[0] - recent_temps[2]) / 2.0;
                total_trend += trend;
                sensor_count += 1;
            }
        }

        if sensor_count > 0 {
            total_trend / sensor_count as f32
        } else {
            0.0
        }
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

    fn query_ambient_temperature() -> Result<Option<f32>> {
        // Platform-specific implementation would go here
        Ok(None)
    }

    fn query_tdp_info() -> Result<Option<TDPInfo>> {
        // Platform-specific implementation would go here
        Ok(None)
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
