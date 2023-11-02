// Copyright (C) 2023 Codex Microsystems. All rights reserved. This
// work is licensed under the terms of the MIT license which can be
// found in the root directory of this project.

use s7::{
    client::Client,
    tcp::{self, Transport},
    transport::Connection,
};
use std::{
    net::{IpAddr, Ipv4Addr},
    time::Duration,
};

use super::{plc_inputs::PLCInputs, plc_outputs::PLCOutputs};

pub struct PLC {
    ip_address: Ipv4Addr,
    s7_client: Client<Transport>,
    is_enabled: bool,
    is_healthy: bool,
    inputs: PLCInputs,
    outputs: PLCOutputs,
}

impl PLC {
    #[must_use]
    pub const fn new(
        ip_address: Ipv4Addr,
        s7_client: Client<Transport>,
        is_enabled: bool,
        is_healthy: bool,
        inputs: PLCInputs,
        outputs: PLCOutputs,
    ) -> Self {
        return Self {
            ip_address,
            s7_client,
            is_enabled,
            is_healthy,
            inputs,
            outputs,
        };
    }

    /// Connects to the PLC.
    fn connect(&mut self) {
        let mut options = tcp::Options::new(IpAddr::from(self.ip_address), 0, 1, Connection::PG);

        // Set read and write timeout to 2 seconds.
        options.read_timeout = Duration::from_secs(2);
        options.write_timeout = Duration::from_secs(2);

        let transport = tcp::Transport::connect(options).unwrap();
        self.s7_client = Client::new(transport).unwrap();
    }

    /// Main loop to read inputs and write outputs to PLC.
    pub fn run(&mut self) {
        loop {
            if !self.is_enabled {
                self.is_healthy = false;
            } else {
                self.connect();
            }
        }
    }

    /// Reads inputs and writes outputs to the PLC once.
    fn update(&mut self) {}

    /// Returns the state of the field e-stop button.
    pub fn get_field_estop(&mut self) -> bool {
        return !self.inputs.field_estop;
    }

    /// Returns the state of the team e-stop buttons.
    pub fn get_team_estops(&mut self) {}

    /// Sets the states of the field stack light.
    pub fn set_field_stack_light(&mut self, red: bool, blue: bool, amber: bool, green: bool) {}

    /// Sets the states of the team stack lights.
    pub fn set_team_stack_lights(&mut self) {}
}
