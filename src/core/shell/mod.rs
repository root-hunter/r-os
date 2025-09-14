use crate::{
    kernel::Kernel,
    process::{BoxedProcess, Process},
};

#[derive(Clone)]
pub struct Shell {
    buffer: String,
    waiting_for_input: bool,
}

impl Shell {
    pub fn new() -> Self {
        Shell {
            buffer: String::new(),
            waiting_for_input: true,
        }
    }

    pub fn set_waiting_for_input(&mut self, waiting: bool) {
        self.waiting_for_input = waiting;
    }
}

impl Process for Shell {
    fn tick(&mut self, k: &mut Kernel) {
        let shell_prompt = format!("{}@{}:~$ ", "user", crate::HOSTNAME);

        if k.tick_count == 0 && self.buffer.is_empty() {
            k.print(&shell_prompt);
        }

        if self.waiting_for_input {
            let text = k.console.value();

            if text.ends_with('\n') {
                if let Some(pos) = text[..text.len() - 1].rfind('\n') {
                    // c’è più di una riga → prendiamo l’ultima prima del newline finale
                    let last = &text[pos + 1..text.len() - 1];
                    let cmd = last.trim().to_string();
                    let cmd = cmd.replace(&shell_prompt.trim(), "").trim().to_string();

                    if !cmd.is_empty() {
                        self.execute_command(&cmd, k);
                    }
                } else {
                    // era la prima riga
                    let last = &text[..text.len() - 1];
                    let cmd = last.trim();

                    if !cmd.is_empty() {
                        self.execute_command(cmd, k);
                    }
                }

                k.print("\n");
                k.print(&shell_prompt);
            }
        }
    }
}

impl Shell {
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
            _ => {
                k.print(&format!("\nUnknown: {}\n", cmd));
            }
        }
    }
}

pub fn make_shell() -> BoxedProcess {
    Box::new(Shell::new())
}
