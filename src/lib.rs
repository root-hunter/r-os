use std::cell::RefCell;
use wasm_bindgen::prelude::*;

mod vfs;
mod kernel;
mod process;
mod core;

pub static HOSTNAME: &str = "r-os";

thread_local! {
    static INPUT_QUEUE: RefCell<Vec<String>> = RefCell::new(Vec::new());
}

#[wasm_bindgen]
pub fn receive_line(line: String) {
    INPUT_QUEUE.with(|q| q.borrow_mut().push(line));
}

// funzione di utilità per la shell
pub fn pop_line() -> Option<String> {
    INPUT_QUEUE.with(|q| q.borrow_mut().pop())
}

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use wasm_bindgen_futures::spawn_local;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    spawn_local(async {
        if let Err(e) = kernel::start_kernel().await {
            log(&format!("Error starting kernel: {:?}", e));
        }
    });

    Ok(())
}

use web_sys::console;

pub fn log(msg: &str) {
    console::log_1(&msg.into());
}
