// Copyright (C) 2023 Codex Microsystems. All rights reserved. This
// work is licensed under the terms of the MIT license which can be
// found in the root directory of this project.

use crate::field::driver_station_status::*;
use std::net::{TcpStream, UdpSocket};

const DS_TCP_LISTEN_PORT: u16 = 1750;
const DS_UDP_SEND_PORT: u16 = 1121;
const DS_UDP_RECEIVE_PORT: u16 = 1160;
const DS_TCP_LINK_TIMEOUT_SECONDS: u8 = 5;
const DS_UDP_LINK_TIMEOUT_SECONDS: u8 = 1;
const MAX_TCP_PACKET_BYTES: u16 = 4096;

const DS_NAMES: [&'static str; 6] = ["RED 1", "RED 2", "RED 3", "BLUE 1", "BLUE 2", "BLUE 3"];

pub struct DriverStation {
    team_id: u16,
    driver_station: u8,
    tcp_connection: TcpStream,
    udp_connection: UdpSocket,
    ap_status: &'static APStatus,
    fms_status: FMSStatus,
    robot_status: RobotStatus,
    ds_status: DSStatus,
}

impl DriverStation {
    fn new_driver_station_connection() {}

    /// Encodes the control information into a packet
    fn encode_control_packet(&mut self, driver_station: u8) -> [u8; 22] {
        let mut packet: [u8; 22] = [0; 22];

        // Packet number, stored big-endian in two bytes
        packet[0] = (self.ds_status.packet_count >> 8) & 0xff;
        packet[1] = self.ds_status.packet_count & 0xff;

        // Protocol version
        packet[2] = 0;

        // Robot status
        packet[3] = 0;
        if self.fms_status.auto {
            packet[3] |= 0x02;
        }
        if self.fms_status.enabled {
            packet[3] |= 0x04;
        }
        if self.fms_status.estop {
            packet[3] |= 0x80;
        }

        // Unknown or unused
        packet[4] = 0;

        // Driver station
        packet[5] = driver_station;

        self.ds_status.packet_count += 1;

        return packet;
    }

    /// Encodes and sends the next control packet to the driver station
    fn send_control_packet(&mut self, driver_station: u8) {
        let packet = self.encode_control_packet(driver_station);
        self.udp_connection
            .send(&packet)
            .expect("[ERROR] Unable to send control packet.");
    }

    /// Decodes a driver station status packet (0x16)
    fn decode_status_packet(&mut self, data: [u8; 36], driver_station: u8) {
        // Parse brownout (bit 7)
        self.robot_status.brownout = ((data[4] >> 7) & 0x1) != 0;
    }
}
