use std::thread;

use regex::Regex;

mod command;

use crate::{
    core::shell::command::ShellCommand, kernel::{Kernel, Message}, log, process::{BoxedProcess, Process}
};

static REG_SHELL: &str = r"(^[\w\d-]+@[\w\d-]+:.+\$\s?)(.+)$";

#[derive(Clone)]
pub struct Shell {
    pid: usize,
    buffer: String,
    waiting_for_input: bool,
    regex: Regex,
}

impl Shell {
    pub fn new() -> Self {
        Shell {
            pid: 0,
            buffer: String::new(),
            waiting_for_input: true,
            regex: Regex::new(REG_SHELL).unwrap(),
        }
    }
}

impl Process for Shell {
    fn pid(&self) -> usize {
        self.pid
    }

    fn set_pid(&mut self, pid: usize) {
        self.pid = pid;
    }

    fn tick(&mut self, k: &mut Kernel) {
        let shell_prompt = format!("{}@{}:~$ ", "user", crate::HOSTNAME);

        if k.tick_count == 1 && self.buffer.is_empty() {
            log("[shell] Shell process started");
            self.print_welcome(k);
            k.print(&shell_prompt);
        }

        if self.waiting_for_input {
            let text = k.console.value();

            let last_line = text.lines().last().unwrap_or("").to_string();

            if (last_line.is_empty() || self.regex.is_match(&last_line)) && text.ends_with("\n") {
                if last_line.is_empty() {
                    k.print(&shell_prompt);
                    return;
                }

                let split = self
                    .regex
                    .captures(&last_line)
                    .expect("regex matched but no captures");

                let command = split.get(2).map_or("", |m| m.as_str());

                log(&format!("[shell] detected command: '{}'", command.trim()));

                if !command.is_empty() {
                    self.execute_command(&command, k);
                }

                k.print("\n");
                k.print(&shell_prompt);
            }
        }
    }

    fn on_message(&mut self, _k: &mut Kernel, msg: Message) {
        match msg {
            Message::SetWaitingForInput(wait) => {
                self.waiting_for_input = wait;
            }
            Message::Print(s) => {
                _k.print(&s);
            }
            Message::Kill => {
                // opzionale: gestione di terminazione
            }
        }
    }
}

impl Shell {
    pub fn print_welcome(&mut self, kernel: &mut Kernel) {
        let welcome = r#"
==================================================
                Welcome to R-OS
==================================================

Version: 0.1.0
Author : Antonio Ricciardi
Kernel : Custom Rust/WebAssembly Kernel
Type   : Experimental Browser OS

Type 'help' to see available commands.
Enjoy your stay!

"#;

        kernel.print(welcome);
    }

    fn execute_command(&mut self, cmd: &str, k: &mut Kernel) {
        match cmd {
            "help" => {
                k.print("\nCommands: help, clear, ls, echo <text>, spawn_demo\n");
            }
            "clear" => {
                k.clear();
            }
            "ls" => {
                let list: Vec<String> = k.fs.list();
                k.print(&format!("\n{}\n", list.join("\n")));
            }
            c if c.starts_with("echo ") => {
                let rest = c.trim_start_matches("echo ").trim();
                k.print(&format!("\n{}\n", rest));
            }
            "demo" => {
                k.print("\nSpawning demo process...\n");
                k.spawn(Box::new(crate::core::demo::DemoProcess::new(self)));
            }
            c if c.starts_with("mkdir") => {
                let args: Vec<&str> = c.split_whitespace().skip(1).collect();
                let cmd = command::mkdir::MkDirCommand;
                let result = cmd.execute(k, args);
                k.print(&format!("\n{}\n", result));
            }
            _ => {
                k.print(&format!("\nUnknown: {}\n", cmd));
            }
        }
    }
}

pub fn make_shell() -> BoxedProcess {
    Box::new(Shell::new())
}
