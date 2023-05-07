use std::io;
use std::path::PathBuf;
use std::process;
use std::process::Command as StdCommand;

pub struct Command {
    inner: StdCommand,
}

impl Command {
    pub fn new(program: &str) -> Command {
        Command {
            inner: StdCommand::new(program),
        }
    }

    pub fn arg(&mut self, arg: &str) -> &mut Command {
        self.inner.arg(arg);
        self
    }

    pub fn current_dir(&mut self, dir: &PathBuf) -> &mut Command {
        self.inner.current_dir(dir);
        self
    }

    pub fn output(&mut self) -> io::Result<process::Output> {
        let output = self.inner.output();

        log::debug!("Executed command: {:?}", self.inner);
        log::debug!("Output: {:?}", output);
        output
    }
}
