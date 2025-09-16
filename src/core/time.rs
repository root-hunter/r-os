use crate::{kernel::Kernel, process::{BoxedProcess, Process}};

pub struct SystemClockProcess {
    pid: usize,
    name: String
}

impl SystemClockProcess {
    fn new() -> Self {
        Self { pid: 0, name: "system_clock".into() }
    }
}

impl Process for SystemClockProcess {
    fn pid(&self) -> usize {
        self.pid
    }

    fn set_pid(&mut self, _pid: usize) {
        self.pid = _pid;
    }

    fn tick(&mut self, _k: &mut Kernel) {
        let now = chrono::Utc::now();
        _k.set_time(now.timestamp_millis());

        let timestamp = now.format("%Y-%m-%d %H:%M:%S").to_string();
        _k.set_timestamp(timestamp);
    }
    
    fn name(&self) -> String {
        self.name.clone()
    }
}

pub fn init() -> BoxedProcess {
    Box::new(SystemClockProcess::new())
}