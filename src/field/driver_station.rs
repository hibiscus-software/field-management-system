// Copyright (C) 2023 Codex Microsystems. All rights reserved. This
// work is licensed under the terms of the MIT license which can be
// found in the root directory of this project.

const DS_TCP_LISTEN_PORT: u16 = 1750;
const DS_UDP_SEND_PORT: u16 = 1121;
const DS_UDP_RECEIVE_PORT: u16 = 1160;
const DS_TCP_LINK_TIMEOUT_SECONDS: u8 = 5;
const DS_UDP_LINK_TIMEOUT_SECONDS: u8 = 1;
const MAX_TCP_PACKET_BYTES: u16 = 4096;

const DS_NAMES: [str; 6] = ["RED 1", "RED 2", "RED 3", "BLUE 1", "BLUE 2", "BLUE 3"];

pub struct DriverStation {}

pub impl DriverStation {
    fn encodeControlPacket(driver_station: u8) -> [u8; 22] {
        let mut packet: [u8; 22] = [];

        // Protocol version
        packet[2] = 0;

        // Robot status
        packet[3] = 0;

        return packet;
    }
}
