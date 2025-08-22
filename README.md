# solid-rpc-rs

> **Build blazing-fast, type-safe full-stack apps with SolidJS & Rust!**

🚀 **solid-rpc-rs** is a template repository designed to seamlessly combine [SolidJS](https://www.solidjs.com/) and [Rust](https://www.rust-lang.org/) using RPC (Remote Procedure Call) for full-stack web development. This project provides a modern, efficient, and type-safe foundation for building reactive web apps with a Rust-powered backend and a SolidJS frontend. There is **one** source of truth for whats callable on your server and thats your `code`, no `.proto` files needed!

---

## ✨ Features

- **SolidJS Frontend**: Lightning-fast, reactive UI with fine-grained reactivity.
- **Rust Backend**: Safe, performant, and reliable server-side logic.
- **RPC Integration**: Effortlessly call Rust backend functions from SolidJS with type safety.
- **Type Sharing**: Share types between frontend and backend for minimal runtime errors.
- **All-in-one Build**: Build the rust project and embed your solidjs app and serve it with one file!
- **Easily Configurable**: Gives your configuration some type safety and hassle-free loading
- **Style your way**: Comes with CSS modules and Tailwind (removable with 3 lines from `package.json` & `index.css`)

---

## 🚀 Getting Started

1. **Clone:**

   ```bash
   git clone https://github.com/NexRX/solid-rpc-rs.git
   cd solid-rpc-rs
   ```

2. **Install dependencies:**

   - For the frontend:

     ```bash
     pnpm install # or npm, yarn, bun, etc
     ```

   - For the backend:

     ```bash
     cargo build
     ```

3. **Run the development servers:**
   - Start the backend:

     ```bash
     cargo run
     ```

   - Start the frontend:

     ```bash
     pnpm run dev # or npm, yarn, bun, etc
     ```

## Build for production

Once you've got started, you can depend on `cargo build` to build the front end for you, it will detect your js package manager with prioity order of:

1. pnpm
2. bun
3. deno
4. yarn
5. npm

You can change this order in `build.rs > js_package_manager()` as you like.
In regular (debug) builds, it will re-use existing frontend builds, but when you build for release it will always rebuild your frontend to have the latest version.

1. **Compile**:

    ```rust
    cargo build --release
    ```

    Now copy solid `target/release/solid-rpc-rs` (.exe on windows) to your server

2. **Run**:

    ```bash
    chmod +x ./solid-rpc-rs # linux only
    ./solid-rpc-rs # .exe on windows
    ```

---

## 🛠️ Usage

- Define your Rust RPC endpoints in the backend.
- Call them from SolidJS using qubit client.
- Share types between frontend and backend for end-to-end type safety.

---

## 📦 Folder Structure

```toml
solid-rpc-rs/
├── src-backend/ # Rust backend with RPC endpoints
├── src/         # SolidJS frontend
├── bindings/    # Backend types for frontends
```
