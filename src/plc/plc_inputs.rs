// Copyright (C) 2023 Codex Microsystems. All rights reserved. This
// work is licensed under the terms of the MIT license which can be
// found in the root directory of this project.

pub struct PLCInputs {
    pub field_estop: bool,
    pub red_estop_1: bool,
    pub red_estop_2: bool,
    pub red_estop_3: bool,
    pub blue_estop_1: bool,
    pub blue_estop_2: bool,
    pub blue_estop_3: bool,
    pub red_connected_1: bool,
    pub red_connected_2: bool,
    pub red_connected_3: bool,
    pub blue_connected_1: bool,
    pub blue_connected_2: bool,
    pub blue_connected_3: bool,
    pub input_count: u8,
}

impl PLCInputs {
    pub fn from_array(&mut self, inputs: [bool; 13]) -> &mut PLCInputs {
        if inputs.len() > 6 {
            self.field_estop = !inputs[0];
            self.red_estop_1 = inputs[1];
            self.red_estop_2 = inputs[2];
            self.red_estop_3 = inputs[3];
            self.blue_estop_1 = inputs[4];
            self.blue_estop_2 = inputs[5];
            self.blue_estop_3 = inputs[6];
        } else if inputs.len() > 12 {
            self.red_connected_1 = inputs[7];
            self.red_connected_2 = inputs[8];
            self.red_connected_3 = inputs[9];
            self.blue_connected_1 = inputs[10];
            self.blue_connected_2 = inputs[11];
            self.blue_connected_3 = inputs[12];
        } else {
            println!(
                "[ERROR] Unable to read inputs from PLC. Recieved input length is not long enough."
            )
        }
        return self;
    }

    pub fn to_array(&self) -> [bool; 7] {
        return [
            !self.field_estop,
            self.red_estop_1,
            self.red_estop_2,
            self.red_estop_3,
            self.blue_estop_1,
            self.blue_estop_2,
            self.blue_estop_3,
        ];
    }
}

impl Default for PLCInputs {
    fn default() -> Self {
        PLCInputs {
            field_estop: false,
            red_estop_1: false,
            red_estop_2: false,
            red_estop_3: false,
            blue_estop_1: false,
            blue_estop_2: false,
            blue_estop_3: false,
            red_connected_1: false,
            red_connected_2: false,
            red_connected_3: false,
            blue_connected_1: false,
            blue_connected_2: false,
            blue_connected_3: false,
            input_count: 13,
        }
    }
}
