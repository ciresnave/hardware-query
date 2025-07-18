use crate::{HardwareQueryError, Result};
use serde::{Deserialize, Serialize};

/// Battery status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BatteryStatus {
    Charging,
    Discharging,
    Full,
    NotCharging,
    Unknown,
}

impl std::fmt::Display for BatteryStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BatteryStatus::Charging => write!(f, "Charging"),
            BatteryStatus::Discharging => write!(f, "Discharging"),
            BatteryStatus::Full => write!(f, "Full"),
            BatteryStatus::NotCharging => write!(f, "Not Charging"),
            BatteryStatus::Unknown => write!(f, "Unknown"),
        }
    }
}

/// Battery information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatteryInfo {
    /// Current battery percentage (0-100)
    pub percentage: f32,
    /// Battery status
    pub status: BatteryStatus,
    /// Time remaining in minutes (if available)
    pub time_remaining_minutes: Option<u32>,
    /// Battery health percentage (0-100)
    pub health_percent: Option<f32>,
    /// Design capacity in Wh
    pub design_capacity_wh: Option<f32>,
    /// Current capacity in Wh
    pub current_capacity_wh: Option<f32>,
    /// Cycle count
    pub cycle_count: Option<u32>,
    /// Battery temperature in Celsius
    pub temperature: Option<f32>,
    /// Voltage in Volts
    pub voltage: Option<f32>,
    /// Current in Amperes
    pub current: Option<f32>,
    /// Battery manufacturer
    pub manufacturer: Option<String>,
    /// Battery model
    pub model: Option<String>,
    /// Battery serial number
    pub serial_number: Option<String>,
}

impl BatteryInfo {
    /// Query battery information
    pub fn query() -> Result<Self> {
        // This would be platform-specific implementation
        // For now, return an error if no battery detected
        Err(HardwareQueryError::device_not_found("No battery detected"))
    }

    /// Get battery percentage
    pub fn percentage(&self) -> f32 {
        self.percentage
    }

    /// Get battery status
    pub fn status(&self) -> &BatteryStatus {
        &self.status
    }

    /// Check if battery is charging
    pub fn is_charging(&self) -> bool {
        matches!(self.status, BatteryStatus::Charging)
    }

    /// Check if battery is discharging
    pub fn is_discharging(&self) -> bool {
        matches!(self.status, BatteryStatus::Discharging)
    }

    /// Get time remaining in hours
    pub fn time_remaining_hours(&self) -> Option<f32> {
        self.time_remaining_minutes
            .map(|minutes| minutes as f32 / 60.0)
    }

    /// Get battery health percentage
    pub fn health_percent(&self) -> Option<f32> {
        self.health_percent
    }

    /// Calculate battery wear percentage
    pub fn wear_percent(&self) -> Option<f32> {
        match (self.design_capacity_wh, self.current_capacity_wh) {
            (Some(design), Some(current)) if design > 0.0 => {
                Some(((design - current) / design) * 100.0)
            }
            _ => None,
        }
    }

    /// Check if battery needs replacement (>20% wear or <80% health)
    pub fn needs_replacement(&self) -> bool {
        if let Some(health) = self.health_percent {
            health < 80.0
        } else if let Some(wear) = self.wear_percent() {
            wear > 20.0
        } else {
            false
        }
    }

    /// Get current capacity in Wh (used by power estimation)
    pub fn capacity_wh(&self) -> Option<f32> {
        self.current_capacity_wh
    }

    /// Get charge percentage (alias for percentage)
    pub fn charge_percent(&self) -> f32 {
        self.percentage
    }
}
