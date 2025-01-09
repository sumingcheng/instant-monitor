use crate::models::{SystemInfo, DiskInfo, NetworkInfo};
use chrono::Utc;
use sysinfo::{CpuExt, DiskExt, System, SystemExt, NetworkExt};
use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct SystemService {
    sys: System,
    network_stats: HashMap<String, (Instant, u64, u64)>, // 接口名 -> (时间戳, 接收字节, 发送字节)
}

impl SystemService {
    pub fn new() -> Self {
        SystemService {
            sys: System::new_all(),
            network_stats: HashMap::new(),
        }
    }

    pub fn get_system_info(&mut self) -> SystemInfo {
        self.sys.refresh_all();

        SystemInfo {
            // 基础系统信息
            hostname: self.sys.host_name().unwrap_or_default(),
            os_type: self.sys.os_version().unwrap_or_default(),
            os_version: self.sys.long_os_version().unwrap_or_default(),
            kernel_version: self.sys.kernel_version().unwrap_or_default(),
            system_time: Utc::now(),
            uptime: self.sys.uptime(),

            // CPU 信息
            cpu_model: self.sys.cpus().first()
                .map(|cpu| cpu.brand().to_string())
                .unwrap_or_default(),
            cpu_cores: self.sys.cpus().len(),
            cpu_usage: self.get_cpu_usage(),
            cpu_frequency: self.sys.cpus().first()
                .map(|cpu| cpu.frequency())
                .unwrap_or_default(),
            cpu_temperature: self.get_cpu_temperature(),

            // 内存信息
            memory_total: self.sys.total_memory(),
            memory_used: self.sys.used_memory(),
            memory_free: self.sys.free_memory(),
            swap_total: self.sys.total_swap(),
            swap_used: self.sys.used_swap(),

            // 存储信息
            disks: self.get_disks_info(),

            // 网络信息
            networks: self.get_networks_info(),
        }
    }

    fn get_cpu_usage(&self) -> f32 {
        let cpu_usage: f32 = self.sys.cpus()
            .iter()
            .map(|cpu| cpu.cpu_usage())
            .sum::<f32>() / self.sys.cpus().len() as f32;
        cpu_usage
    }

    fn get_cpu_temperature(&self) -> Option<f32> {
        self.sys.components()
            .iter()
            .find(|component| component.label().contains("CPU"))
            .map(|cpu| cpu.temperature())
    }

    fn get_disks_info(&self) -> Vec<DiskInfo> {
        self.sys.disks()
            .iter()
            .map(|disk| DiskInfo {
                name: disk.name().to_string_lossy().into_owned(),
                mount_point: disk.mount_point().to_string_lossy().into_owned(),
                total_space: disk.total_space(),
                used_space: disk.total_space() - disk.available_space(),
                available_space: disk.available_space(),
                file_system: String::from_utf8_lossy(disk.file_system()).into_owned(),
            })
            .collect()
    }

    fn get_networks_info(&mut self) -> Vec<NetworkInfo> {
        let current_time = Instant::now();
        let mut network_info = Vec::new();

        for (interface_name, data) in self.sys.networks() {
            let (rx_speed, tx_speed) = if let Some((last_time, last_rx, last_tx)) = 
                self.network_stats.get(interface_name) 
            {
                let time_diff = current_time.duration_since(*last_time).as_secs_f64();
                let rx_diff = data.received() - last_rx;
                let tx_diff = data.transmitted() - last_tx;
                
                (rx_diff as f64 / time_diff, tx_diff as f64 / time_diff)
            } else {
                (0.0, 0.0)
            };

            // 更新网络统计信息
            self.network_stats.insert(
                interface_name.to_string(),
                (current_time, data.received(), data.transmitted())
            );

            // 获取网络接口的IP地址
            let ip_addresses = self.get_interface_ips(interface_name);

            network_info.push(NetworkInfo {
                interface: interface_name.to_string(),
                ip_addresses,
                mac_address: self.get_mac_address(interface_name),
                received_bytes: data.received(),
                transmitted_bytes: data.transmitted(),
                received_packets: data.packets_received(),
                transmitted_packets: data.packets_transmitted(),
                rx_speed,
                tx_speed,
            });
        }

        network_info
    }

    fn get_interface_ips(&self, interface: &str) -> Vec<String> {
        // 这里使用 if_addrs 或 pnet 库获取IP地址
        // 简单实现可以先返回空数组
        Vec::new()
    }

    fn get_mac_address(&self, interface: &str) -> Option<String> {
        // 这里使用 mac_address 或 pnet 库获取MAC地址
        // 简单实现可以先返回 None
        None
    }
}
