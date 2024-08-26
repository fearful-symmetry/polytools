//! Handlers for the Management* subset of REST APIs.

use std::collections::HashMap;
use crate::{errors::PolyRestError, PolyRest};
use reqwest::{header::CONTENT_TYPE, Method};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PolyWrapper<ApiResponse> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename(deserialize = "data", serialize="data"))]
    pub data: ApiResponse
}

// got: {"data": {"ModelNumber": "VVX 411", "FirmwareRelease": "5.5.0.22173", "DeviceType": "hardwareEndpoint", "MACAddress": "64167fcacee2", 
// "DeviceVendor": "Polycom", "UpTimeSinceLastReboot": "0 Day 6:16:10", "IPV4Address": "192.168.1.9", "IPV6Address": "::", "AttachedHardware": {"EM": []}}, "Status": "2000"}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
/// Returns device info for the given Polycom device
pub struct DeviceInfo {
    pub model_number: String,
    pub firmware_release: String,
    pub device_type: String,
    pub device_vendor: String,
    pub up_time_since_last_reboot: String,
    #[serde(rename(deserialize = "IPV4Address"))]
    pub ipv4_address: String, 
    #[serde(rename(deserialize = "IPV6Address"))]
    pub ipv6_address: String, 
    #[serde(rename(deserialize = "MACAddress"))]
    pub mac_address: String, 
}


#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
/// Carries network config for the device
pub struct NetworkInfo {
    pub default_gateway: String,
    #[serde(rename(deserialize = "IPV4Address"))]
    pub ipv4_address: String, 
    #[serde(rename(deserialize = "IPV6Address"))]
    pub ipv6_address: String, 
    #[serde(rename(deserialize = "DHCPServer"))]
    pub dhcp_server: String,
    #[serde(rename(deserialize = "DHCP"))]
    pub dhcp: String,
    pub upgrade_server: String,
    #[serde(rename(deserialize = "DHCPOption60Format"))]
    pub dhcp_option_60_format: String,
    #[serde(rename(deserialize = "DHCPBootServerUseOption"))]
    pub dhcp_boot_server_use_option: String,
    #[serde(rename(deserialize = "ZTPStatus"))]
    pub ztp_status: String,
    #[serde(rename(deserialize = "DHCPBootServerOption"))]
    pub dhcp_boot_server_option: String,
    #[serde(rename(deserialize = "DHCPBootServerOptionType"))]
    pub dhcp_boot_server_option_type: String,
    #[serde(rename(deserialize = "LLDP"))]
    pub lldp: String,
    #[serde(rename(deserialize = "LANPortStatus"))]
    pub lan_port_status: String,
    pub subnet_mask: String,
    #[serde(rename(deserialize = "AlternateDNSServer"))]
    pub alternate_dns_server: String,
    #[serde(rename(deserialize = "DNSServer"))]
    pub dns_server: String,
    #[serde(rename(deserialize = "DNSDomain"))]
    pub dns_domain: String,
    #[serde(rename(deserialize = "LANSpeed"))]
    pub lan_speed: String,
    #[serde(rename(deserialize = "SNTPAddress"))]
    pub sntp_address: String,
    #[serde(rename(deserialize = "VLANDiscoveryMode"))]
    pub vlan_discovery_mode: String,
    #[serde(rename(deserialize = "CDPCompability"))]
    pub cdp_compability: String,
    #[serde(rename(deserialize = "VLANID"))]
    pub vlan_id: String,
    #[serde(rename(deserialize = "VLANIDOption"))]
    pub vlan_id_option: String,
    pub prov_server_address: String,
    pub prov_server_user: String,
    pub prov_server_type: String,
    pub wifi: Option<NetworkInfoWifi>
}

#[derive(Deserialize, Debug)]
/// Carries Wi-Fi info for the device, if it exists
pub struct NetworkInfoWifi {
    #[serde(rename(deserialize = "Signal Strength"))]
    pub signal_strength: String, 
    #[serde(rename(deserialize = "State"))]
    pub state: String, 
    #[serde(rename(deserialize = "Security Mode"))]
    pub security_mode: String, 
    #[serde(rename(deserialize = "SSID"))]
    pub ssid: String, 
    #[serde(rename(deserialize = "Duration"))]
    pub duration: String, 
}

#[derive(Deserialize, Debug)]
/// Carries network stats for the device
pub struct NetworkStats {
    #[serde(rename(deserialize = "UpTime"))]
    pub uptime: String, 
    #[serde(rename(deserialize = "RxPackets"))]
    pub rx_packets: String, 
    #[serde(rename(deserialize = "TxPackets"))]
    pub tx_packets: String, 
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
/// Wraps the raw config response when making a call to the config/get REST endpoint
pub struct ConfigResponseValue {
    pub value: String,
    pub source: String,
}

impl PolyRest {
    /// fetch device info
    pub fn device_info(&mut self) -> Result<DeviceInfo, PolyRestError> {
        let path = format!("{}/api/v1/mgmt/device/info", self.url);
        let resp = self.raw_get(path)?;
        let parsed: PolyWrapper<DeviceInfo> = serde_json::from_str(&resp)?;
        Ok(parsed.data)
    }

    /// fetch network info
    pub fn network_info(&mut self) -> Result<NetworkInfo, PolyRestError> {
        let path = format!("{}/api/v1/mgmt/network/info", self.url);
        let resp = self.raw_get(path)?;
        let parsed: PolyWrapper<NetworkInfo> = serde_json::from_str(&resp)?;
        Ok(parsed.data)
    }

    /// fetch network stats
    pub fn network_stats(&mut self) -> Result<NetworkStats, PolyRestError> {
        let path = format!("{}/api/v1/mgmt/network/stats", self.url);
        let resp = self.raw_get(path)?;
        let parsed: PolyWrapper<NetworkStats> = serde_json::from_str(&resp)?;
        Ok(parsed.data)
    }

    /// set a config value
    pub fn config_set(&mut self, key: String, value: String) -> Result<String, PolyRestError> {
        let setter: HashMap<String, String> = HashMap::from([(key, value)]);
        let req = serde_json::to_string(&PolyWrapper{data: setter, status: None})?;

        let path = format!("{}/api/v1/mgmt/config/set", self.url);
        let req = self.client.request(Method::POST, path)
        .basic_auth(&self.username, Some(&self.password)).body(req)
        .header(CONTENT_TYPE, "application/json").build()?;

        let resp = self.client.execute(req)?.error_for_status()?;
        let raw_resp = resp.bytes()?;
        let resp_str = String::from_utf8_lossy(&raw_resp);
        Ok(resp_str.to_string())

    }

    /// fetch a config value for the given string
    pub fn config_get(&mut self, value: String) -> Result<HashMap<String, ConfigResponseValue>, PolyRestError> {
        let config: Vec<String> = vec![value];
        let req =  serde_json::to_string(&PolyWrapper{data: config, status: None})?;

        let path = format!("{}/api/v1/mgmt/config/get", self.url);
        let req = self.client.request(Method::POST, path)
        .basic_auth(&self.username, Some(&self.password)).body(req)
        .header(CONTENT_TYPE, "application/json")
        .build()?;

        let resp = self.client.execute(req)?.error_for_status()?;
        let raw_resp = resp.bytes()?;   
        
        let resp_str = String::from_utf8_lossy(&raw_resp);
        let parsed: PolyWrapper<HashMap<String, ConfigResponseValue>> = serde_json::from_str(&resp_str)?;

        Ok(parsed.data)
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut handler = PolyRest::new("Polycom".to_string(), "789".to_string(), "https://192.168.1.9".to_string(), true).unwrap();

        let resp =handler.device_info().unwrap();

        println!("got: {:?}", resp);
    }
}