// Copyright (C) 2023 Codex Microsystems. All rights reserved. This
// work is licensed under the terms of the MIT license which can be
// found in the root directory of this project.

pub struct APStatus {
    pub linked: bool,
    pub signal: &'static str,
    pub quality: [&'static str],
}

/// Statuses that the robot sends to FMS
pub struct RobotStatus {
    pub enabled: bool,
    pub mode: u8,
    pub estop: bool,
    pub radio_ping: bool,
    pub rio_ping: bool,
    pub last_linked_time: u16,
    pub comms_active: bool,
    pub battery_voltage: u16,
    pub trip_time_ms: u16,
    pub brownout: bool,
    pub bandwidth: u16,
}

pub struct FMSStatus {
    pub bypassed: bool,
    pub auto: bool,
    pub enabled: bool,
    pub estop: bool,
}

pub struct DSStatus {
    pub linked: bool,
    pub missed_packet_count: u16,
    pub last_packet_time: u16,
    pub packet_count: u8,
    pub ip_address: &'static str,
    pub missed_packet_offset: u16,
    pub computer_battery_percent: u16,
    pub computer_cpu_percent: u16,
    pub last_log: str,
}
