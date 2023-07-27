// Copyright (C) 2023 Codex Microsystems. All rights reserved. This
// work is licensed under the terms of the MIT license which can be
// found in the root directory of this project.

pub struct FMSStatus {
    bypassed: bool,
    auto: bool,
    enabled: bool,
    estop: bool,
}

pub struct DSStatus {
    linked: bool,
}
