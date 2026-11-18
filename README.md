# FemboyFS 🦀

A lightweight command-line utility and interactive terminal shell for basic file and directory management, built in pure Rust using only the standard library (`std`).

## ✨ Features

- **Zero external dependencies:** Built strictly with Rust's `std` library (`std::fs`, `std::io`, `std::env`, `std::path`).
- **Dual execution modes:** Run single commands directly via CLI arguments or launch an interactive terminal session.
- **Built-in file editor:** Create and write multi-line files directly from the terminal using `Ctrl + X` to save and exit.
- **Safety prompts:** Asks for confirmation before deleting files or directories.

## 🚀 Getting Started

### Prerequisites
- [Rust & Cargo](https://rust-lang.org) installed on your machine.

### Build and Run
1. Clone the repository:
   ```bash
   git clone https://github.com
   cd FemboyFS
   ```

2. Build the project in release mode:
   ```bash
   cargo build --release
   ```

## 🛠️ Usage

You can run the tool using `cargo run -- <command>` or by executing the compiled binary directly.

### 1. Direct CLI Mode
Pass a command and its argument directly when running the program:
```bash
cargo run -- folder my_new_folder
cargo run -- new notes.txt
cargo run -- fls
cargo run -- erase notes.txt
cargo run -- delDir my_new_folder
cargo run -- cmd
```

### 2. Interactive Terminal Mode
Run the program without any arguments to enter interactive mode:
```bash
cargo run
```
Once inside `TERMINAL MODE`, type commands directly without repeating the binary name.

## 📋 Available Commands

| Command | Argument | Description |
| :--- | :--- | :--- |
| `folder` | `<name>` | Creates a new directory if it does not already exist. |
| `new` | `<name>` | Creates a new file and opens an input prompt (`Ctrl + X` + `Enter` to finish writing). |
| `fls` | *None* | Lists all files and folders in the current directory. |
| `goto` | `<path>` | Changes the current working directory *(Interactive Terminal Mode only)*. |
| `erase` | `<name>` | Deletes a specific file after confirmation (`y/n`). |
| `delDir` | `<path>` | Deletes a directory recursively after confirmation (`y/n`). |
| `cmd` | *None* | Displays the list of available commands. |
| `end` | *None* | Exits Interactive Terminal Mode. |

## 👤 Author

**Miguel Zanon** — [@Zanon-dev2](https://github.com)
