use std::process::Command;

#[derive(Debug)]
pub struct SimpleShell {
    pub cmd: String,
    pub tokens: Vec<String>,
}

impl SimpleShell {
    /**
    Creates a new Simple Shell, as well as parsing the cmd into tokens
    */
    pub fn new(cmd: String) -> SimpleShell {
        let tokens: Vec<String> = cmd.split_whitespace().map(|word| word.to_string()).collect();
        SimpleShell { cmd, tokens }
    }
    pub fn is_quit(&self) -> bool {
        self.tokens[0].eq_ignore_ascii_case("quit")
    }
    pub fn exec_command(&self) {
        let command = self.tokens[0].clone();
        let output = Command::new(command)
            .arg(self.tokens[1..].join(" "))
            .spawn()
            .expect("Error: Failed to execute command")
            .wait()
            .expect("Error: Failed to execute command");;
    }
}