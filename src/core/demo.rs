use std::sync::{Arc, Mutex};

use crate::{core::shell::Shell, kernel::Kernel, process::Process};

pub struct DemoProcess {
    counter: u32,
    life: u32,
    shell: Arc<Mutex<Shell>>,
}

impl DemoProcess {
    pub fn new(shell: &mut Shell) -> Self {
        shell.set_waiting_for_input(false);
        Self {
            counter: 0,
            life: 120,
            shell: Arc::new(Mutex::new(shell.clone())),
        }
    }
}

impl Process for DemoProcess {
    fn tick(&mut self, k: &mut Kernel) {
        if self.life == 0 {
            // no-op: in un kernel reale rimuoveremmo il processo
            //self.shell.lock().unwrap().set_waiting_for_input(true);
            return;
        }
        if self.counter % 30 == 0 {
            k.print(&format!(
                "\n[demo] tick {} ({} left)\n",
                self.counter, self.life
            ));
        }
        self.counter += 1;
        self.life -= 1;
    }
}
