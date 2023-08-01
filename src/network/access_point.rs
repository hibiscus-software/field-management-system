// Copyright (C) 2023 Codex Microsystems. All rights reserved. This
// work is licensed under the terms of the MIT license which can be
// found in the root directory of this project.

const SSH_PORT: &str = "22";

pub struct AccessPoint {
    ip_address: &'static str,
    username: &'static str,
    password: &'static str,
}

impl AccessPoint {
    /// Logs into the access point via SSH and runs the given command. Reads the
    /// output and returns it as a string.
    fn run_command(&mut self, command: &str) -> String {
        // Open a SSH connection to the access point.
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
}
