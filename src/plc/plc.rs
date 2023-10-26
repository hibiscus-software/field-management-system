// Copyright (C) 2023 Codex Microsystems. All rights reserved. This
// work is licensed under the terms of the MIT license which can be
// found in the root directory of this project.

use std::net::Ipv4Addr;

use super::{plc_inputs::PLCInputs, plc_outputs::PLCOutputs};

pub struct PLC {
    ip_address: Ipv4Addr,
    inputs: PLCInputs,
    outputs: PLCOutputs,
}

impl PLC {
    #[must_use]
    pub const fn new(ip_address: Ipv4Addr, inputs: PLCInputs, outputs: PLCOutputs) -> Self {
        return Self {
            ip_address,
            inputs,
            outputs,
        };
    }

    /// Connects to the PLC.
    pub fn connect() {}

    /// Main loop to read inputs and write outputs to PLC.
    pub fn run() {}

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
