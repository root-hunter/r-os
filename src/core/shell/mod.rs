use std::sync::Arc;

use async_std::sync::Mutex;
use regex::Regex;
use wasm_bindgen_futures::spawn_local;

mod command;

use crate::{
    console_log, core::shell::{self, command::{ShellCommand, ShellCommandWithData, ShellCommandWithShell}}, kernel::{Kernel, Message}, process::{BoxedProcess, Process}
};

static REG_SHELL: &str = r"(^[\w\d-]+@[\w\d-]+:.+\$\s?)(.+)$";

#[derive(Debug, Clone)]
pub struct Shell {
    pid: usize,
    buffer: String,
    waiting_for_input: bool,
    regex: Regex,
    folder: Arc<Mutex<String>>,
}

impl Shell {
    pub fn new() -> Self {
        Shell {
            pid: 0,
            buffer: String::new(),
            waiting_for_input: true,
            regex: Regex::new(REG_SHELL).unwrap(),
            folder: Arc::new(Mutex::new("/".to_string())),
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
        let shell_prompt = self.shell_prompt();

        if k.tick_count == 1 && self.buffer.is_empty() {
            console_log("[shell] Shell process started");
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

                console_log(&format!("[shell] detected command: '{}'", command.trim()));

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
    pub fn shell_prompt(&self) -> String {
        let folder = futures::executor::block_on(async { self.folder.lock().await.clone() });
        format!("user@r-os:{}$ ", folder)
    }

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

    fn execute_command<'a>(&mut self, cmd: &'a str, k: &mut Kernel) {
        match cmd {
            "help" => {
                k.print("\nCommands: help, clear, ls, echo <text>, spawn_demo\n");
            }
            "clear" => {
                k.clear();
            }
            c if c.starts_with("ls") => {
                let k_clone = Kernel::clone_rc();
                let c_owned = c.to_string();

                spawn_local(async move {
                    let result = {
                        let mut kernel = k_clone.lock().await;
                        command::ls::LsCommand::execute(&mut kernel, &c_owned).await
                    };

                    {
                        let kernel = k_clone.lock().await;
                        kernel.print(&format!("\n{}\n", result));
                    }
                });
            }
            c if c.starts_with("exists") => {
                let k_clone = Kernel::clone_rc();
                let c_owned = c.to_string();

                spawn_local(async move {
                    let result = {
                        let mut kernel = k_clone.lock().await;
                        command::exists::ExistsCommand::execute(&mut kernel, &c_owned).await
                    };

                    {
                        let kernel = k_clone.lock().await;
                        kernel.print(&format!("\n{}\n", result));
                    }
                });
            }
            c if c.starts_with("echo ") => {
                let rest = c.trim_start_matches("echo ").trim();
                k.print(&format!("\n{}\n", rest));
            }
            c if c.starts_with("cd") => {
                let k_clone = Kernel::clone_rc();
                let c_owned = c.to_string();

                let folder_clone = self.folder.clone();

                spawn_local(async move {
                    let shell_prompt = format!("\nuser@r-os:{}$ ", folder_clone.lock().await);

                    let result = {
                        let mut kernel = k_clone.lock().await;
                        command::cd::CdCommand::execute_data(&mut kernel, &c_owned, folder_clone).await
                    };
                });
            }
            "demo" => {
                k.print("\nSpawning demo process...\n");
                k.spawn(Box::new(crate::core::demo::DemoProcess::new(self)));
            }
            c if c.starts_with("mkdir") => {
                let k_clone = Kernel::clone_rc();
                let c_owned = c.to_string();
                let mut shell = self.clone();

                console_log(&format!("[shell] executing command: '{}'", c_owned));

                spawn_local(async move {
                    let result = {
                        let mut kernel = k_clone.lock().await;
                        command::mkdir::MkDirCommand::execute(&mut kernel, &mut shell,  &c_owned).await
                    };

                    {
                        let kernel = k_clone.lock().await;
                        kernel.print(&format!("\n{}\n", result));
                    }
                });
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
