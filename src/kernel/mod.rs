use wasm_bindgen::prelude::*;
use web_sys::{window, HtmlTextAreaElement};
use crate::console_log;
use crate::kernel::defaults::{PID_DEFAULT_SYSTEM_CLOCK, PID_DEFAULT_SYSTEM_SHELL};
use crate::process::{Process, BoxedProcess};
use crate::vfs::fs::SimpleFS;
use std::cell::RefCell;
use std::collections::{BTreeMap, VecDeque};
use std::rc::Rc;

pub mod defaults;
pub mod errors;

thread_local! {
    static KERNEL: RefCell<Option<Rc<Mutex<Kernel>>>> = RefCell::new(None);
}

pub enum Message {
    SetWaitingForInput(bool),
    Print(String),
    Kill,
}
pub struct Kernel {
    pub console: HtmlTextAreaElement,
    pub last_pid: usize,
    pub processes: BTreeMap<usize, BoxedProcess>,
    pub fs: SimpleFS,
    pub tick_count: u64,
    pub messages: VecDeque<(usize, Message)>,
    time: i64,
    timestamp: String
}

impl Kernel {
    pub fn new(console: HtmlTextAreaElement) -> Self {
        Self {
            console,
            last_pid: 1000,
            processes: BTreeMap::new(),
            fs: SimpleFS::new(),
            tick_count: 0,
            messages: VecDeque::new(),
            time: 0,
            timestamp: "".into()
        }
    }

    pub fn clone_rc() -> Rc<Mutex<Kernel>> {
        KERNEL.with(|k| k.borrow().as_ref().unwrap().clone())
    }

    pub fn send(&mut self, pid: usize, msg: Message) {
        self.messages.push_back((pid, msg));
    }

    fn deliver_messages(&mut self) {
        let kernel_ptr: *mut Kernel = self;

        while let Some((pid, msg)) = self.messages.pop_front() {
            if let Some(proc) = self.processes.get_mut(&pid) {
                unsafe {
                    proc.on_message(&mut *kernel_ptr, msg);
                }
            }
        }
    }

    pub fn print(&self, s: &str) {
        let mut val = self.console.value();
        val.push_str(s);
        self.console.set_value(&val);
        self.console.set_scroll_top(self.console.scroll_height());
    }

    pub fn clear(&self) {
        self.console.set_value("");
    }

    pub fn get_next_pid(&mut self) -> usize {
        self.last_pid += 1;
        self.last_pid
    }

    pub fn spawn(&mut self, p: BoxedProcess) {
        let pid = self.get_next_pid();

        self.spawn_with_pid(p, pid);
    }

    pub fn set_time(&mut self, time: i64) {
        self.time = time;
    }

    pub fn get_time(&self) -> i64 {
        self.time
    }

    pub fn set_timestamp(&mut self, timestamp: String) {
        self.timestamp = timestamp;
    }

    pub fn get_timestamp(&self) -> String {
        self.timestamp.clone()
    }

    pub fn spawn_with_pid(&mut self, mut p: BoxedProcess, pid: usize) {
        p.set_pid(pid);

        let pname = p.name();

        self.processes.insert(p.pid(), p);
        console_log(&format!("Spawning process with pid {} ({})\nProcesses: {:?}", pid, pname, self.processes.keys()));
    }

    pub fn kill(&mut self, pid: usize) {
        if self.processes.remove(&pid).is_some() {
            console_log(&format!("Killed process with pid {}", pid));
        } else {
            console_log(&format!("No process with pid {} found", pid));
        }
    }

    pub fn tick(&mut self) {
        self.tick_count += 1;

        let kernel_ptr: *mut Kernel = self;

        for (pid, proc) in self.processes.iter_mut() {
            unsafe {
                proc.tick(&mut *kernel_ptr);
            }
        }
        self.deliver_messages();
    }

    pub async fn init(mut self) -> Result<Self, JsValue> {
        self.fs.init().await;
        Ok(self)
    }
}
use async_std::sync::Mutex;

pub async fn start_kernel() -> Result<(), JsValue> {
    let window = window().ok_or("no window")?;
    let doc = window.document().ok_or("no document")?;
    let ta = doc
        .get_element_by_id("console")
        .ok_or("no console element")?
        .dyn_into::<HtmlTextAreaElement>()?;

    let kernel = Kernel::new(ta).init().await?;
    let kernel = Rc::new(Mutex::new(kernel));

    KERNEL.with(|k| *k.borrow_mut() = Some(kernel.clone()));

    {
        let mut k = kernel.lock().await;
        let process = crate::core::time::init();
        k.spawn_with_pid(process, PID_DEFAULT_SYSTEM_CLOCK);
    }

    {
        let mut k = kernel.lock().await;
        let process = crate::core::shell::init();
        k.spawn_with_pid(process, PID_DEFAULT_SYSTEM_SHELL);
    }

    schedule_tick();

    Ok(())
}

fn schedule_tick() {
    use wasm_bindgen::closure::Closure;
    use web_sys::window;

    let f = Closure::wrap(Box::new(move || {
        KERNEL.with(|k| {
            if let Some(kref) = k.borrow().as_ref() {
                let k_clone = kref.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let mut kernel = k_clone.lock().await;
                    kernel.tick();
                });
            }
        });
        schedule_tick();
    }) as Box<dyn FnMut()>);

    window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register rAF");
    f.forget();
}