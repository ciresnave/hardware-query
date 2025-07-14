use crate::Result;
use serde::{Deserialize, Serialize};
use sysinfo::Networks;

/// Network interface type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkType {
    Ethernet,
    WiFi,
    Bluetooth,
    Cellular,
    VPN,
    Loopback,
    Unknown,
}

impl std::fmt::Display for NetworkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NetworkType::Ethernet => write!(f, "Ethernet"),
            NetworkType::WiFi => write!(f, "WiFi"),
            NetworkType::Bluetooth => write!(f, "Bluetooth"),
            NetworkType::Cellular => write!(f, "Cellular"),
            NetworkType::VPN => write!(f, "VPN"),
            NetworkType::Loopback => write!(f, "Loopback"),
            NetworkType::Unknown => write!(f, "Unknown"),
        }
    }
}

/// Network interface information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInfo {
    /// Interface name
    pub name: String,
    /// Network type
    pub network_type: NetworkType,
    /// MAC address
    pub mac_address: String,
    /// IP addresses
    pub ip_addresses: Vec<String>,
    /// Interface speed in Mbps (if available)
    pub speed_mbps: Option<u32>,
    /// Is interface up/active
    pub is_up: bool,
    /// Bytes received
    pub bytes_received: u64,
    /// Bytes transmitted
    pub bytes_transmitted: u64,
    /// Packets received
    pub packets_received: u64,
    /// Packets transmitted
    pub packets_transmitted: u64,
    /// Receive errors
    pub receive_errors: u64,
    /// Transmit errors
    pub transmit_errors: u64,
}

impl NetworkInfo {
    /// Query all network interfaces
    pub fn query_all() -> Result<Vec<Self>> {
        let networks = Networks::new_with_refreshed_list();

        let mut network_interfaces = Vec::new();

        for (interface_name, network_data) in &networks {
            let network_info = Self {
                name: interface_name.clone(),
                network_type: Self::detect_network_type(interface_name),
                mac_address: Self::get_mac_address(interface_name),
                ip_addresses: Self::get_ip_addresses(interface_name),
                speed_mbps: None, // Would need platform-specific implementation
                is_up: network_data.received() > 0 || network_data.transmitted() > 0,
                bytes_received: network_data.received(),
                bytes_transmitted: network_data.transmitted(),
                packets_received: network_data.packets_received(),
                packets_transmitted: network_data.packets_transmitted(),
                receive_errors: network_data.errors_on_received(),
                transmit_errors: network_data.errors_on_transmitted(),
            };

            network_interfaces.push(network_info);
        }

        Ok(network_interfaces)
    }

    /// Get interface name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get network type
    pub fn network_type(&self) -> &NetworkType {
        &self.network_type
    }

    /// Get MAC address
    pub fn mac_address(&self) -> &str {
        &self.mac_address
    }

    /// Get IP addresses
    pub fn ip_addresses(&self) -> &[String] {
        &self.ip_addresses
    }

    /// Check if interface is active
    pub fn is_active(&self) -> bool {
        self.is_up
    }

    /// Get total bytes transferred
    pub fn total_bytes(&self) -> u64 {
        self.bytes_received + self.bytes_transmitted
    }

    /// Get total packets transferred
    pub fn total_packets(&self) -> u64 {
        self.packets_received + self.packets_transmitted
    }

    /// Get total errors
    pub fn total_errors(&self) -> u64 {
        self.receive_errors + self.transmit_errors
    }

    fn detect_network_type(name: &str) -> NetworkType {
        let name_lower = name.to_lowercase();

        if name_lower.contains("lo") || name_lower.contains("loopback") {
            NetworkType::Loopback
        } else if name_lower.contains("eth") || name_lower.contains("ethernet") {
            NetworkType::Ethernet
        } else if name_lower.contains("wlan")
            || name_lower.contains("wifi")
            || name_lower.contains("wi-fi")
        {
            NetworkType::WiFi
        } else if name_lower.contains("bluetooth") || name_lower.contains("bt") {
            NetworkType::Bluetooth
        } else if name_lower.contains("cellular") || name_lower.contains("mobile") {
            NetworkType::Cellular
        } else if name_lower.contains("vpn") || name_lower.contains("tunnel") {
            NetworkType::VPN
        } else {
            NetworkType::Unknown
        }
    }

    fn get_mac_address(_interface_name: &str) -> String {
        // Platform-specific implementation would go here
        "00:00:00:00:00:00".to_string()
    }

    fn get_ip_addresses(_interface_name: &str) -> Vec<String> {
        // Platform-specific implementation would go here
        vec![]
    }
}
