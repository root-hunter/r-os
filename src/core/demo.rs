use std::sync::{Arc, Mutex};

use crate::{core::shell::Shell, kernel::{Kernel, Message}, process::Process};

pub struct DemoProcess {
    pid: usize,
    counter: u32,
    life: u32,
    shell: Arc<Mutex<Shell>>,
}

impl DemoProcess {
    pub fn new(shell: &mut Shell) -> Self {
        Self {
            pid: 0,
            counter: 0,
            life: 120,
            shell: Arc::new(Mutex::new(shell.clone())),
        }
    }
}

impl Process for DemoProcess {
    fn pid(&self) -> usize {
        self.pid
    }

    fn set_pid(&mut self, pid: usize) {
        self.pid = pid;
    }

    fn tick(&mut self, k: &mut Kernel) {
        if self.life == 120 {
            let shell_pid = self.shell.lock().unwrap().pid();
            k.send(shell_pid, Message::SetWaitingForInput(false));
        }

        if self.life == 0 {
            let shell_pid = self.shell.lock().unwrap().pid();
            k.send(shell_pid, Message::SetWaitingForInput(true));
            k.kill(self.pid);
            return;
        }
        
        if self.counter % 30 == 0 {
            k.print(&format!(
                "\n[demo] tick {} ({} left)\n",
                self.counter, self.life
            ));

            if self.life == 30 {
                k.print("[demo] demo process is about to terminate...");
                k.print("\n");
            }
        }
        self.counter += 1;
        self.life -= 1;
    }
}
