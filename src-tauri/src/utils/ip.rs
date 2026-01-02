// src/utils/ip.rs

use local_ip_address::list_afinet_netifas;
use std::net::IpAddr;

pub fn get_lan_ip() -> String {
    // 获取所有网络接口
    let network_interfaces = list_afinet_netifas().unwrap_or(vec![]);

    // 策略：优先寻找标准的私有 IP 段 (IPv4)
    // 优先级 1: 192.168.x.x (常见热点/路由)
    for (_, ip) in &network_interfaces {
        if let IpAddr::V4(ipv4) = ip {
            let octets = ipv4.octets();
            if octets[0] == 192 && octets[1] == 168 {
                return ipv4.to_string();
            }
        }
    }

    // 优先级 2: 10.x.x.x (常见企业内网/部分热点)
    for (_, ip) in &network_interfaces {
        if let IpAddr::V4(ipv4) = ip {
            let octets = ipv4.octets();
            if octets[0] == 10 {
                return ipv4.to_string();
            }
        }
    }

    // 优先级 3: 172.16.x.x - 172.31.x.x
    for (_, ip) in &network_interfaces {
        if let IpAddr::V4(ipv4) = ip {
            let octets = ipv4.octets();
            if octets[0] == 172 && (16..=31).contains(&octets[1]) {
                return ipv4.to_string();
            }
        }
    }

    // 优先级 4: 任何非 127.0.0.1 的 IPv4 地址 (保底)
    for (_, ip) in &network_interfaces {
        if let IpAddr::V4(ipv4) = ip {
            if !ipv4.is_loopback() {
                return ipv4.to_string();
            }
        }
    }

    // 如果真的什么都找不到（比如没联网），只能返回回环地址，
    // 但这种情况在漫展现场通常意味着设备没开热点。
    "127.0.0.1".to_string()
}