use wasm_bindgen::prelude::*;
use web_sys::{window, HtmlTextAreaElement};
use crate::log;
use crate::process::{Process, BoxedProcess};
use crate::vfs::fs::SimpleFS;
use std::cell::RefCell;
use std::collections::{BTreeMap, VecDeque};
use std::rc::Rc;

thread_local! {
    static KERNEL: RefCell<Option<Rc<RefCell<Kernel>>>> = RefCell::new(None);
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
}

impl Kernel {
    pub fn new(console: HtmlTextAreaElement) -> Self {
        Self {
            console,
            last_pid: 0,
            processes: BTreeMap::new(),
            fs: SimpleFS::new(),
            tick_count: 0,
            messages: VecDeque::new(),
        }
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

    pub fn get_new_pid(&mut self) -> usize {
        self.last_pid += 1;
        self.last_pid
    }

    pub fn spawn(&mut self, mut p: BoxedProcess) {
        p.set_pid(self.get_new_pid());

        log(&format!("Spawning process with pid {}", p.pid()));
        self.processes.insert(p.pid(), p);

        log(&format!("Processes: {:?}", self.processes.keys()));
    }

    pub fn kill(&mut self, pid: usize) {
        if self.processes.remove(&pid).is_some() {
            log(&format!("Killed process with pid {}", pid));
        } else {
            log(&format!("No process with pid {} found", pid));
        }
    }

    pub fn tick(&mut self) {
        let kernel_ptr: *mut Kernel = self;
        self.tick_count += 1;

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

pub async fn start_kernel() -> Result<(), JsValue> {
    let window = window().ok_or("no window")?;
    let doc = window.document().ok_or("no document")?;
    let ta = doc
        .get_element_by_id("console")
        .ok_or("no console element")?
        .dyn_into::<HtmlTextAreaElement>()?;

    let kernel = Kernel::new(ta).init().await?;
    let kernel = Rc::new(RefCell::new(kernel));
    KERNEL.with(|k| *k.borrow_mut() = Some(kernel.clone()));
    
    // spawn a shell process
    kernel.borrow_mut().spawn(crate::core::shell::make_shell());

    // kick the run loop using requestAnimationFrame
    schedule_tick();

    Ok(())
}

fn schedule_tick() {
    use wasm_bindgen::closure::Closure;
    use web_sys::window;

    let f = Closure::wrap(Box::new(move || {
        KERNEL.with(|k| {
            if let Some(kref) = k.borrow().as_ref() {
                kref.borrow_mut().tick();
            }
        });
        // re-schedule
        schedule_tick();
    }) as Box<dyn FnMut()>);

    window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register rAF");
    f.forget();
}