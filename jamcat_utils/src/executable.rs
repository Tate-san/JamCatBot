use std::{
    io,
    process::Command,
    process::{Output, Stdio},
};

#[derive(Debug)]
pub struct Executable {
    command: Command,
    args: Vec<String>,
}

impl Executable {
    pub fn new(command: impl ToString) -> Self {
        let mut cmd = Command::new(command.to_string());
        cmd//.env("LC_ALL", "en_US.UTF-8")
            .stdout(Stdio::piped())
            .stdin(Stdio::piped())
            .stderr(Stdio::piped());
        Self {
            command: cmd,
            args: vec![],
        }
    }

    pub fn arg(&mut self, arg: impl ToString) -> &mut Self {
        self.args.push(arg.to_string());
        self
    }

    pub fn execute(&mut self) -> io::Result<Output> {
        self.command
            .args(self.args.clone())
            .spawn()
            .and_then(std::process::Child::wait_with_output)
    }
}