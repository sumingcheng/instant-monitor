use serde::Serialize;
use chrono::{DateTime, Utc};

#[derive(Serialize)]
pub struct SystemInfo {
    // 基础系统信息
    pub hostname: String,
    pub os_type: String,
    pub os_version: String,
    pub kernel_version: String,
    pub system_time: DateTime<Utc>,
    pub uptime: u64,  // 系统运行时间(秒)

    // CPU 信息
    pub cpu_model: String,
    pub cpu_cores: usize,
    pub cpu_usage: f32,
    pub cpu_frequency: u64,  // MHz
    pub cpu_temperature: Option<f32>,  // 温度，部分系统可能不支持

    // 内存信息
    pub memory_total: u64,
    pub memory_used: u64,
    pub memory_free: u64,
    pub swap_total: u64,
    pub swap_used: u64,

    // 存储信息
    pub disks: Vec<DiskInfo>,

    // 网络信息
    pub networks: Vec<NetworkInfo>,
}

#[derive(Serialize)]
pub struct DiskInfo {
    pub name: String,
    pub mount_point: String,
    pub total_space: u64,
    pub used_space: u64,
    pub available_space: u64,
    pub file_system: String,
}

#[derive(Serialize)]
pub struct NetworkInfo {
    pub interface: String,
    pub ip_addresses: Vec<String>,
    pub mac_address: Option<String>,
    pub received_bytes: u64,
    pub transmitted_bytes: u64,
    pub received_packets: u64,
    pub transmitted_packets: u64,
    pub rx_speed: f64,  // 当前接收速度 (bytes/s)
    pub tx_speed: f64,  // 当前发送速度 (bytes/s)
}
