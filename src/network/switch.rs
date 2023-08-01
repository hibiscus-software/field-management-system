// Copyright (C) 2023 Codex Microsystems. All rights reserved. This
// work is licensed under the terms of the MIT license which can be
// found in the root directory of this project.

use ssh2::Session;
use std::io::prelude::*;
use std::net::TcpStream;

const SSH_PORT: &str = "22";

const RED_1_VLAN: u8 = 10;
const RED_2_VLAN: u8 = 20;
const RED_3_VLAN: u8 = 30;
const BLUE_1_VLAN: u8 = 40;
const BLUE_2_VLAN: u8 = 50;
const BLUE_3_VLAN: u8 = 60;

pub struct Switch {
    ip_address: &'static str,
    username: &'static str,
    password: &'static str,
}

impl Switch {
    #[must_use]
    pub const fn new(
        ip_address: &'static str,
        username: &'static str,
        password: &'static str,
    ) -> Self {
        return Self {
            ip_address,
            username,
            password,
        };
    }

    fn config_team_ethernet() {}

    /// Returns a map of currently-configured team IDs to VLANs.
    fn get_team_vlans(&mut self) {
        // Get the entire config dump.
        let mut config = self.run_command("show running-config\n");

        // Parse out the team IDs and VLANs from the config dump

        // Build the map of the team IDs to VLANs.
    }

    /// Logs into the switch via SSH and runs the given command in user exec mode.
    /// Reads the output and returns it as a string.
    fn run_command(&mut self, command: &str) -> String {
        // Open a SSH connection to the switch.
        let tcp = TcpStream::connect(self.ip_address.to_owned() + ":" + SSH_PORT).unwrap();
        let mut session = Session::new().unwrap();
        session.set_tcp_stream(tcp);
        session.handshake().unwrap();
        session
            .userauth_password(self.username, self.password)
            .unwrap();

        // Send the command.
        let mut channel = session.channel_session().unwrap();
        channel.exec(command).unwrap();

        // Read the response.
        let mut response = String::new();
        channel.read_to_string(&mut response).unwrap();
        return response;
    }

    /// Logs into the switch via SSH and runs the given command in global
    /// config mode. Reads the output and returns it as a string.
    fn run_config_command(&mut self, command: &str) -> String {
        return self.run_command(
            &("config terminal\n%send\ncopy running-config startup-config\n\n".to_owned()
                + command),
        );
    }
}
