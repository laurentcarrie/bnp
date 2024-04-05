use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::thread;

use serde::{Deserialize, Serialize};

// use std::process::ExitStatus;
use crate::util::error::MyError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessOutput {
    pub command: String,
    pub args: Vec<String>,
    pub pid: Option<u32>,
    pub stdout: String,
    pub stderr: String,
    pub exit_code: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessInput {
    pub command: String,
    pub args: Vec<String>,
}

pub fn run(pi: ProcessInput) -> Result<ProcessOutput, MyError> {
    let mut command = Command::new(&pi.command);
    let binding = command.stdout(Stdio::piped()).stderr(Stdio::piped());
    let binding = pi
        .args
        .iter()
        .fold(binding, |binding, arg| binding.arg(arg));

    let mut binding = match binding.spawn() {
        Ok(binding) => binding,
        Err(err) => {
            eprintln!("error in binding {}", err);
            let result: ProcessOutput = ProcessOutput {
                command: pi.command.clone(),
                args: pi.args.clone(),
                stdout: "".to_string(),
                stderr: format!("{}", err),
                pid: None,
                exit_code: None,
            };
            return Ok(result);
        }
    };

    let child_stdout = binding
        .stdout
        .take()
        .expect("Internal error, could not take stdout");
    let child_stderr = binding
        .stderr
        .take()
        .expect("Internal error, could not take stderr");

    let (stdout_tx, stdout_rx) = std::sync::mpsc::channel();
    let (stderr_tx, stderr_rx) = std::sync::mpsc::channel();

    let stdout_thread = thread::spawn(move || {
        let stdout_lines = BufReader::new(child_stdout).lines();
        for line in stdout_lines {
            let line = line.unwrap();
            eprintln!("{}", line);
            stdout_tx.send(line).unwrap();
        }
    });

    let stderr_thread = thread::spawn(move || {
        let stderr_lines = BufReader::new(child_stderr).lines();
        for line in stderr_lines {
            let line = line.unwrap();
            eprintln!("{}", line);
            stderr_tx.send(line).unwrap();
        }
    });

    let status = binding
        .wait()
        .expect("Internal error, failed to wait on child");

    stdout_thread.join().unwrap();
    stderr_thread.join().unwrap();

    let stdout = stdout_rx.into_iter().collect::<Vec<String>>().join("\n");
    let stderr = stderr_rx.into_iter().collect::<Vec<String>>().join("\n");

    // let exit_code = match command2.success() {
    //     Ok(e) => {
    //         dbg!(e);
    //         0
    //     }
    //     Err(e) => {
    //         dbg!(e);
    //         1
    //     }
    // };

    let exit_code = match status.success() {
        true => 0,
        false => 1,
    };
    eprintln!("exit code is {}", exit_code);

    let result: ProcessOutput = ProcessOutput {
        command: pi.command.clone(),
        args: pi.args.clone(),
        stdout: stdout,
        stderr: stderr,
        pid: Some(binding.id()),
        exit_code: Some(exit_code),
    };
    Ok(result)
}
