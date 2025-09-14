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

```

## 📜 Roadmap

The development of **rOS** is experimental and exploratory, but here are the main milestones:

- ✅ **Core Kernel** — cooperative scheduler, process management, messaging
- ✅ **Interactive Shell** — command parsing & execution (e.g. `mkdir`)
- ✅ **Persistent Virtual File System (VFS)** — backed by IndexedDB
- 🔄 **Asynchronous Processes** — better task orchestration and async system calls
- ⏳ **User Programs** — ability to write and run sandboxed user applications
- ⏳ **Window Manager & UI** — graphical layer with multiple terminal windows
- ⏳ **Simulated Networking** — sockets, messaging between processes
- ⏳ **Extensible Commands** — richer shell environment with pluggable modules

_This roadmap is flexible and may evolve as the project grows._

---

## 🤝 Contributing

We welcome contributions of all kinds — from bug fixes and documentation improvements to entirely new features.

How to get involved:
1. **Fork** the repository on GitLab/GitHub.
2. **Create a branch** for your feature or bugfix.
3. **Submit a merge request (MR)** with a clear description of your changes.

Before contributing:
- Please make sure your code follows Rust’s best practices (formatting, linting).
- Add documentation/comments for new functionality.
- Where possible, include small examples or tests.

Discussions, ideas, and feedback are just as valuable as code. Don’t hesitate to open an **issue** to propose improvements or ask questions.

---

## 📄 License

This project is distributed under the terms of the **MIT License** and **Apche 2.0 License**.  
You are free to use, modify, and distribute this software for personal and commercial purposes, as long as you include the copyright notice.

See the full [LICENSE-APACHE](LICENSE-APACHE) file for details.
See the full [LICENSE-MIT](LICENSE-MIT) file for details.