use wasm_bindgen::prelude::*;
use web_sys::{window, HtmlTextAreaElement};
use crate::process::{Process, BoxedProcess};
use crate::vfs::fs::SimpleFS;
use std::cell::RefCell;
use std::rc::Rc;

thread_local! {
    static KERNEL: RefCell<Option<Rc<RefCell<Kernel>>>> = RefCell::new(None);
}

pub struct Kernel {
    pub console: HtmlTextAreaElement,
    pub processes: Vec<BoxedProcess>,
    pub fs: SimpleFS,
    pub tick_count: u64,
}

impl Kernel {
    pub fn new(console: HtmlTextAreaElement) -> Self {
        Self {
            console,
            processes: Vec::new(),
            fs: SimpleFS::new(),
            tick_count: 0,
        }
    }

    pub fn print(&self, s: &str) {
        let mut val = self.console.value();
        val.push_str(s);
        self.console.set_value(&val);
        // scroll to bottom
        self.console.set_scroll_top(self.console.scroll_height());
    }

    pub fn clear(&self) {
        self.console.set_value("");
    }

    pub fn spawn(&mut self, p: BoxedProcess) {
        self.processes.push(p);
    }

    pub fn tick(&mut self) {
        // estrai tick_count per usarlo dopo
        self.tick_count += 1;

        // usiamo raw pointer per evitare il borrow annidato
        let kernel_ptr: *mut Kernel = self;

        for proc in self.processes.iter_mut() {
            unsafe {
                // ora possiamo passare &mut *kernel_ptr senza conflitto
                proc.tick(&mut *kernel_ptr);
            }
        }
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

// Provide a way for other modules to access kernel (if needed)
pub fn with_kernel<F,R>(f: F) -> Option<R>
where
    F: FnOnce(&mut Kernel) -> R
{
    KERNEL.with(|k| {
        k.borrow().as_ref().map(|rc| {
            let mut k = rc.borrow_mut();
            f(&mut k)
        })
    })
}