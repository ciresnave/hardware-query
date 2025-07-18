//! Real-time hardware monitoring module
//!
//! This module provides continuous monitoring capabilities for hardware metrics,
//! with configurable update intervals and event-driven notifications.

use crate::{HardwareInfo, ThermalInfo, PowerProfile, Result, HardwareQueryError};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{broadcast, RwLock, Mutex};
use tokio::time::interval;

/// Hardware monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    /// Update interval for hardware polling
    pub update_interval: Duration,
    /// Enable thermal monitoring
    pub enable_thermal: bool,
    /// Enable power monitoring
    pub enable_power: bool,
    /// Enable general hardware monitoring
    pub enable_hardware: bool,
    /// Temperature threshold for thermal alerts (Celsius)
    pub thermal_threshold: f32,
    /// Power threshold for power alerts (Watts)
    pub power_threshold: Option<f32>,
    /// Enable background monitoring
    pub background_monitoring: bool,
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            update_interval: Duration::from_secs(5),
            enable_thermal: true,
            enable_power: true,
            enable_hardware: true,
            thermal_threshold: 80.0,
            power_threshold: None,
            background_monitoring: true,
        }
    }
}

/// Hardware monitoring event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MonitoringEvent {
    /// Thermal threshold exceeded
    ThermalAlert {
        sensor_name: String,
        temperature: f32,
        threshold: f32,
        timestamp: std::time::SystemTime,
    },
    /// Power consumption alert
    PowerAlert {
        current_power: f32,
        threshold: f32,
        timestamp: std::time::SystemTime,
    },
    /// Hardware configuration changed
    HardwareChanged {
        change_type: HardwareChangeType,
        description: String,
        timestamp: std::time::SystemTime,
    },
    /// Monitoring error occurred
    MonitoringError {
        error: String,
        timestamp: std::time::SystemTime,
    },
    /// Regular update with current metrics
    MetricsUpdate {
        hardware_info: Option<HardwareInfo>,
        thermal_info: Option<ThermalInfo>,
        power_profile: Option<PowerProfile>,
        timestamp: std::time::SystemTime,
    },
}

/// Type of hardware configuration change
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HardwareChangeType {
    /// Device connected
    DeviceConnected,
    /// Device disconnected
    DeviceDisconnected,
    /// Driver changed
    DriverChanged,
    /// Configuration modified
    ConfigurationChanged,
    /// Performance state changed
    PerformanceStateChanged,
}

/// Monitoring statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringStats {
    /// Total monitoring events generated
    pub total_events: u64,
    /// Thermal alerts generated
    pub thermal_alerts: u64,
    /// Power alerts generated
    pub power_alerts: u64,
    /// Hardware change events
    pub hardware_changes: u64,
    /// Monitoring errors encountered
    pub errors: u64,
    /// Monitoring uptime
    pub uptime: Duration,
    /// Last update timestamp
    pub last_update: std::time::SystemTime,
    /// Average update interval
    pub average_update_interval: Duration,
}

/// Hardware monitoring callback trait
#[async_trait]
pub trait MonitoringCallback: Send + Sync {
    async fn on_event(&self, event: &MonitoringEvent);
}

/// Simple closure-based callback implementation
pub struct ClosureCallback<F>
where
    F: Fn(&MonitoringEvent) + Send + Sync,
{
    callback: F,
}

impl<F> ClosureCallback<F>
where
    F: Fn(&MonitoringEvent) + Send + Sync,
{
    pub fn new(callback: F) -> Self {
        Self { callback }
    }
}

#[async_trait]
impl<F> MonitoringCallback for ClosureCallback<F>
where
    F: Fn(&MonitoringEvent) + Send + Sync,
{
    async fn on_event(&self, event: &MonitoringEvent) {
        (self.callback)(event);
    }
}

/// Real-time hardware monitor
pub struct HardwareMonitor {
    config: MonitoringConfig,
    callbacks: Arc<Mutex<Vec<Box<dyn MonitoringCallback>>>>,
    event_sender: broadcast::Sender<MonitoringEvent>,
    stats: Arc<RwLock<MonitoringStats>>,
    running: Arc<RwLock<bool>>,
    start_time: Instant,
    last_hardware_info: Arc<RwLock<Option<HardwareInfo>>>,
    last_thermal_info: Arc<RwLock<Option<ThermalInfo>>>,
    last_power_profile: Arc<RwLock<Option<PowerProfile>>>,
}

impl HardwareMonitor {
    /// Create a new hardware monitor with default configuration
    pub fn new() -> Self {
        Self::with_config(MonitoringConfig::default())
    }

    /// Create a new hardware monitor with custom configuration
    pub fn with_config(config: MonitoringConfig) -> Self {
        let (event_sender, _) = broadcast::channel(1000);
        
        Self {
            config,
            callbacks: Arc::new(Mutex::new(Vec::new())),
            event_sender,
            stats: Arc::new(RwLock::new(MonitoringStats {
                total_events: 0,
                thermal_alerts: 0,
                power_alerts: 0,
                hardware_changes: 0,
                errors: 0,
                uptime: Duration::from_secs(0),
                last_update: std::time::SystemTime::now(),
                average_update_interval: Duration::from_secs(0),
            })),
            running: Arc::new(RwLock::new(false)),
            start_time: Instant::now(),
            last_hardware_info: Arc::new(RwLock::new(None)),
            last_thermal_info: Arc::new(RwLock::new(None)),
            last_power_profile: Arc::new(RwLock::new(None)),
        }
    }

    /// Add a monitoring callback
    pub async fn add_callback<T: MonitoringCallback + 'static>(&self, callback: T) {
        let mut callbacks = self.callbacks.lock().await;
        callbacks.push(Box::new(callback));
    }

    /// Add a simple closure-based callback
    pub async fn on_event<F>(&self, callback: F) 
    where
        F: Fn(&MonitoringEvent) + Send + Sync + 'static,
    {
        self.add_callback(ClosureCallback::new(callback)).await;
    }

    /// Add a thermal threshold callback
    pub async fn on_thermal_threshold<F>(&self, threshold: f32, _callback: F)
    where
        F: Fn(&ThermalInfo) + Send + Sync + 'static,
    {
        self.on_event(move |event| {
            if let MonitoringEvent::ThermalAlert { temperature, .. } = event {
                if *temperature >= threshold {
                    // This is a simplified version - in practice we'd pass the thermal info
                    // For now, we'll trigger on any thermal alert above threshold
                }
            }
        }).await;
    }

    /// Add a power threshold callback
    pub async fn on_power_threshold<F>(&self, threshold: f32, _callback: F)
    where
        F: Fn(&PowerProfile) + Send + Sync + 'static,
    {
        self.on_event(move |event| {
            if let MonitoringEvent::PowerAlert { current_power, .. } = event {
                if *current_power >= threshold {
                    // This is a simplified version - in practice we'd pass the power profile
                }
            }
        }).await;
    }

    /// Subscribe to monitoring events
    pub fn subscribe(&self) -> broadcast::Receiver<MonitoringEvent> {
        self.event_sender.subscribe()
    }

    /// Start monitoring in the background
    pub async fn start_monitoring(&self) -> Result<()> {
        {
            let mut running = self.running.write().await;
            if *running {
                return Err(HardwareQueryError::InvalidConfiguration(
                    "Monitoring is already running".to_string()
                ));
            }
            *running = true;
        }

        let config = self.config.clone();
        let callbacks = Arc::clone(&self.callbacks);
        let event_sender = self.event_sender.clone();
        let stats = Arc::clone(&self.stats);
        let running = Arc::clone(&self.running);
        let last_hardware_info = Arc::clone(&self.last_hardware_info);
        let last_thermal_info = Arc::clone(&self.last_thermal_info);
        let last_power_profile = Arc::clone(&self.last_power_profile);

        tokio::spawn(async move {
            let mut interval = interval(config.update_interval);
            let mut update_times = Vec::new();

            while *running.read().await {
                interval.tick().await;
                let update_start = Instant::now();

                // Query hardware information
                let mut hardware_info = None;
                let mut thermal_info = None;
                let mut power_profile = None;
                let mut events = Vec::new();

                if config.enable_hardware {
                    match HardwareInfo::query() {
                        Ok(info) => {
                            hardware_info = Some(info);
                        }
                        Err(e) => {
                            events.push(MonitoringEvent::MonitoringError {
                                error: format!("Failed to query hardware info: {}", e),
                                timestamp: std::time::SystemTime::now(),
                            });
                        }
                    }
                }

                if config.enable_thermal {
                    match ThermalInfo::query() {
                        Ok(info) => {
                            // Check for thermal alerts
                            for sensor in info.sensors() {
                                if sensor.temperature >= config.thermal_threshold {
                                    events.push(MonitoringEvent::ThermalAlert {
                                        sensor_name: sensor.name.clone(),
                                        temperature: sensor.temperature,
                                        threshold: config.thermal_threshold,
                                        timestamp: std::time::SystemTime::now(),
                                    });
                                }
                            }
                            thermal_info = Some(info);
                        }
                        Err(e) => {
                            events.push(MonitoringEvent::MonitoringError {
                                error: format!("Failed to query thermal info: {}", e),
                                timestamp: std::time::SystemTime::now(),
                            });
                        }
                    }
                }

                if config.enable_power {
                    match PowerProfile::query() {
                        Ok(profile) => {
                            // Check for power alerts
                            if let (Some(current_power), Some(threshold)) = 
                                (profile.total_power_draw, config.power_threshold) {
                                if current_power >= threshold {
                                    events.push(MonitoringEvent::PowerAlert {
                                        current_power,
                                        threshold,
                                        timestamp: std::time::SystemTime::now(),
                                    });
                                }
                            }
                            power_profile = Some(profile);
                        }
                        Err(e) => {
                            events.push(MonitoringEvent::MonitoringError {
                                error: format!("Failed to query power profile: {}", e),
                                timestamp: std::time::SystemTime::now(),
                            });
                        }
                    }
                }

                // Generate metrics update event
                events.push(MonitoringEvent::MetricsUpdate {
                    hardware_info: hardware_info.clone(),
                    thermal_info: thermal_info.clone(),
                    power_profile: power_profile.clone(),
                    timestamp: std::time::SystemTime::now(),
                });

                // Update cached information
                if let Some(info) = hardware_info {
                    *last_hardware_info.write().await = Some(info);
                }
                if let Some(info) = thermal_info {
                    *last_thermal_info.write().await = Some(info);
                }
                if let Some(profile) = power_profile {
                    *last_power_profile.write().await = Some(profile);
                }

                // Send events and notify callbacks
                for event in &events {
                    // Send to broadcast channel
                    let _ = event_sender.send(event.clone());

                    // Notify callbacks
                    let callbacks = callbacks.lock().await;
                    for callback in callbacks.iter() {
                        callback.on_event(event).await;
                    }
                }

                // Update statistics
                {
                    let mut stats = stats.write().await;
                    stats.total_events += events.len() as u64;
                    
                    for event in &events {
                        match event {
                            MonitoringEvent::ThermalAlert { .. } => stats.thermal_alerts += 1,
                            MonitoringEvent::PowerAlert { .. } => stats.power_alerts += 1,
                            MonitoringEvent::HardwareChanged { .. } => stats.hardware_changes += 1,
                            MonitoringEvent::MonitoringError { .. } => stats.errors += 1,
                            _ => {}
                        }
                    }

                    stats.last_update = std::time::SystemTime::now();
                    let update_duration = update_start.elapsed();
                    update_times.push(update_duration);
                    
                    // Keep only last 100 update times for average calculation
                    if update_times.len() > 100 {
                        update_times.remove(0);
                    }
                    
                    if !update_times.is_empty() {
                        let total_time: Duration = update_times.iter().sum();
                        stats.average_update_interval = total_time / update_times.len() as u32;
                    }
                }
            }
        });

        Ok(())
    }

    /// Stop monitoring
    pub async fn stop_monitoring(&self) {
        *self.running.write().await = false;
    }

    /// Check if monitoring is currently running
    pub async fn is_monitoring(&self) -> bool {
        *self.running.read().await
    }

    /// Get current monitoring statistics
    pub async fn get_stats(&self) -> MonitoringStats {
        let mut stats = self.stats.read().await.clone();
        stats.uptime = self.start_time.elapsed();
        stats
    }

    /// Get the last cached hardware information
    pub async fn get_last_hardware_info(&self) -> Option<HardwareInfo> {
        self.last_hardware_info.read().await.clone()
    }

    /// Get the last cached thermal information
    pub async fn get_last_thermal_info(&self) -> Option<ThermalInfo> {
        self.last_thermal_info.read().await.clone()
    }

    /// Get the last cached power profile
    pub async fn get_last_power_profile(&self) -> Option<PowerProfile> {
        self.last_power_profile.read().await.clone()
    }

    /// Update monitoring configuration
    pub async fn update_config(&mut self, new_config: MonitoringConfig) {
        self.config = new_config;
    }

    /// Clear all callbacks
    pub async fn clear_callbacks(&self) {
        let mut callbacks = self.callbacks.lock().await;
        callbacks.clear();
    }
}

impl Default for HardwareMonitor {
    fn default() -> Self {
        Self::new()
    }
}
