# rOS 🖥️  
_An experimental Rust + WebAssembly Operating System in the Browser_

![Rust](https://img.shields.io/badge/language-Rust-orange)
![WebAssembly](https://img.shields.io/badge/target-WebAssembly-blueviolet)
![IndexedDB](https://img.shields.io/badge/storage-IndexedDB-lightgrey)

---

## 📖 Introduction
**rOS** (or `r-os`) is an experimental operating system that runs **entirely inside the browser**.  
It is built in **Rust**, compiled to **WebAssembly**, and provides a minimal kernel, a shell-like command interpreter, and a persistent virtual file system backed by **IndexedDB**.

The project is primarily a playground for systems programming in the browser, exploring how far we can push OS-like abstractions on top of WebAssembly.

Core goals:
- A lightweight kernel with cooperative multitasking.
- An interactive shell for user commands.
- A persistent **Virtual File System (VFS)** stored in the browser.
- A foundation for further experiments: async tasks, UI, and more.

---

## 🚀 Features
- **Kernel**
  - Cooperative process scheduler (`tick`-based).
  - Process management with PIDs and message passing.
  - Asynchronous execution via `spawn_local`.

- **Shell**
  - Interactive prompt (`user@r-os:~$`).
  - Command parsing (e.g. `mkdir`).
  - Asynchronous command execution.

- **Virtual File System (VFS)**
  - Backed by **IndexedDB** for persistence across sessions.
  - Hierarchical path keys (`/dir/subdir/file`).
  - Supports directory creation (`mkdir`).

- **Architecture**
  - Written in **Rust**.
  - Compiled to **WebAssembly** for browser execution.
  - Uses modern async Rust (`async/await`) for non-blocking tasks.

---

## 📂 Project Structure

```
r-os/
├── src/
│ ├── kernel.rs # Main kernel (scheduler, tick loop, messaging)
│ ├── core/
│ │ └── shell/ # Shell and command implementations
│ ├── vfs/ # Virtual File System (IndexedDB backend)
│ ├── process.rs # Process traits and definitions
│ └── lib.rs # WebAssembly entrypoint
├── pkg/ # wasm-pack build output
├── static/ # HTML/JS frontend files
└── Cargo.toml
```

---

## 🛠️ Technology Stack
- **Rust** (safe systems programming)
- **WebAssembly** (via `wasm-bindgen` + `wasm-pack`)
- **IndexedDB** (via [`idb`](https://crates.io/crates/idb))
- **requestAnimationFrame** as the kernel "clock"
- **Async Rust** for non-blocking I/O

---

## ▶️ Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/) (nightly recommended for wasm)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/)

### Build & Run
```bash
# Clone the repository
git clone https://gitlab.com/root-hunter/r-os.git
cd r-os

# Compile to WebAssembly
wasm-pack build --target web

# Start a static web server (example: Python)
python3 -m http.server 8080