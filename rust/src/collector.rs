//! 网络流量数据采集模块
//! 通过 Windows API (GetIfTable2) 采集各网卡的累计收发字节数，供上层统计和绘图使用。

use std::collections::HashMap;
use std::time::Instant;

#[cfg(target_os = "windows")]
use winapi::um::iphlpapi::GetIfTable2;
#[cfg(target_os = "windows")]
use winapi::um::iphlpapi::FreeMibTable;
#[cfg(target_os = "windows")]
use winapi::shared::iptypes::MIB_IF_TABLE2;
#[cfg(target_os = "windows")]
use winapi::shared::winsock2::AF_INET;
#[cfg(target_os = "windows")]
use std::ffi::OsStr;
#[cfg(target_os = "windows")]
use std::os::windows::ffi::OsStrExt;

/// 单次采样快照
#[derive(Clone, Debug)]
pub struct Snapshot {
    /// 自程序启动以来的秒数
    pub elapsed_secs: f64,
    /// 累计接收字节数
    pub bytes_recv: u64,
    /// 累计发送字节数
    pub bytes_sent: u64,
}

/// 网卡设备信息
#[derive(Clone, Debug)]
pub struct DeviceInfo {
    /// 设备名称
    pub name: String,
    /// IPv4 地址列表
    pub addrs: Vec<String>,
}

/// 网络流量采集器
pub struct Collector {
    start: Instant,
}

impl Collector {
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
        }
    }

    /// 获取自启动以来的秒数
    pub fn elapsed_secs(&self) -> f64 {
        self.start.elapsed().as_secs_f64()
    }

    /// 打印所有网络接口的调试信息
    pub fn print_debug_info(&self) {
        println!("\n=== Network Interfaces Debug Info ===");
        let devices = self.devices();
        println!("Total interfaces detected: {}\n", devices.len());

        for dev in &devices {
            println!("Interface: {}", dev.name);
            println!("  IP addresses:");
            if dev.addrs.is_empty() {
                println!("    (none)");
            } else {
                for addr in &dev.addrs {
                    println!("    - {}", addr);
                }
            }
            println!();
        }
    }

    /// 获取所有可用设备信息（按名称排序）
    pub fn devices(&self) -> Vec<DeviceInfo> {
        #[cfg(target_os = "windows")]
        {
            get_network_interfaces_windows()
        }
        #[cfg(not(target_os = "windows"))]
        {
            vec![]
        }
    }

    /// 采集一次所有网卡的当前累计数据
    pub fn collect(&mut self) -> HashMap<String, Snapshot> {
        let elapsed = self.start.elapsed().as_secs_f64();

        #[cfg(target_os = "windows")]
        {
            get_network_stats_windows(elapsed)
        }
        #[cfg(not(target_os = "windows"))]
        {
            HashMap::new()
        }
    }
}

#[cfg(target_os = "windows")]
fn get_network_interfaces_windows() -> Vec<DeviceInfo> {
    use std::ptr;

    unsafe {
        let mut if_table: *mut MIB_IF_TABLE2 = ptr::null_mut();

        if GetIfTable2(&mut if_table) != 0 {
            return vec![];
        }

        let mut devices = Vec::new();

        if !if_table.is_null() {
            let table = &*if_table;
            let rows = std::slice::from_raw_parts(table.Table.as_ptr(), table.NumEntries as usize);

            for row in rows {
                // 跳过非活跃接口
                if row.OperStatus != 1 {
                    continue;
                }

                let name = wide_to_string(row.Description.as_ptr());

                // 简单处理：假设每个接口有一个 IPv4 地址（实际应该查询 GetAdaptersInfo）
                let addrs = if name.to_lowercase().contains("loopback") {
                    vec!["127.0.0.1".to_string()]
                } else {
                    vec![]
                };

                devices.push(DeviceInfo { name, addrs });
            }

            FreeMibTable(if_table as *mut _);
        }

        devices.sort_by(|a, b| a.name.cmp(&b.name));
        devices
    }
}

#[cfg(target_os = "windows")]
fn get_network_stats_windows(elapsed: f64) -> HashMap<String, Snapshot> {
    use std::ptr;

    let mut snapshots = HashMap::new();

    unsafe {
        let mut if_table: *mut MIB_IF_TABLE2 = ptr::null_mut();

        if GetIfTable2(&mut if_table) != 0 {
            return snapshots;
        }

        if !if_table.is_null() {
            let table = &*if_table;
            let rows = std::slice::from_raw_parts(table.Table.as_ptr(), table.NumEntries as usize);

            for row in rows {
                // 跳过非活跃接口
                if row.OperStatus != 1 {
                    continue;
                }

                let name = wide_to_string(row.Description.as_ptr());

                snapshots.insert(
                    name,
                    Snapshot {
                        elapsed_secs: elapsed,
                        bytes_recv: row.InOctets,
                        bytes_sent: row.OutOctets,
                    },
                );
            }

            FreeMibTable(if_table as *mut _);
        }
    }

    snapshots
}

#[cfg(target_os = "windows")]
fn wide_to_string(ptr: *const u16) -> String {
    use std::ptr;

    if ptr.is_null() {
        return String::new();
    }

    unsafe {
        let mut len = 0;
        let mut p = ptr;
        while *p != 0 {
            len += 1;
            p = p.offset(1);
        }

        let slice = std::slice::from_raw_parts(ptr, len);
        String::from_utf16_lossy(slice).to_string()
    }
}
