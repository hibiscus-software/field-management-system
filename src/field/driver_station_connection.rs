// Copyright (C) 2023 Codex Microsystems. All rights reserved. This
// work is licensed under the terms of the MIT license which can be
// found in the root directory of this project.

const DS_TCP_LISTEN_PORT: i32 = 1750;
const DS_UDP_SEND_PORT: i32 = 1121;
const DS_UDP_RECEIVE_PORT: i32 = 1160;
const DS_TCP_LINK_TIMEOUT_SECONDS: i32 = 5;
const DS_UDP_LINK_TIMEOUT_SECONDS: i32 = 1;
const MAX_TCP_PACKET_BYTES: i32 = 4096;

pub struct DriverStationConnection {
    team_id: i32,
    alliance_station: str,
}

impl DriverStationConnection {}
