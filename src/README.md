# Rust CLI Todo App 📝

A command-line todo list application built with Rust as part of my journey learning Rust for Solana development.

## Features

- Add, update, and remove tasks
- Track task status (Todo, In Progress, Completed)
- Persistent storage with JSON
- Clean CLI interface with emoji icons
- Filter tasks by status

## What I Learned

This project taught me fundamental Rust concepts:

- **Ownership & Borrowing**: Understanding `&self` vs `&mut self`
- **Error Handling**: Custom error types with `thiserror`
- **Traits**: Implemented a `Storable` trait for persistence
- **Pattern Matching**: Used enums extensively for command parsing
- **Functional Programming**: Iterators, `filter()`, `map()`, and `collect()`
- **Serialization**: Using `serde` for JSON serialization

## Installation

Make sure you have [Rust installed](https://rustup.rs/), then:

```bash
git clone https://github.com/YOUR_USERNAME/rust-cli-todo
cd rust-cli-todo
cargo build --release
```

## Usage

Run the application:

```bash
cargo run
```

### Available Commands

```
add <description>        Add a new task
list [status]            List all tasks (or filter by status)
update <num> <status>    Update task status (todo/in-progress/done)
remove <num>             Remove a task
clear                    Remove all completed tasks
save                     Save tasks to file
help                     Show help message
exit                     Save and exit
```

### Examples

```bash
> add Buy groceries
✅ Task added successfully!

> add Learn Rust
✅ Task added successfully!

> list
📋 Your Tasks:
─────────────────────────────────────
⚪ 1. Buy groceries [TODO]
⚪ 2. Learn Rust [TODO]
─────────────────────────────────────

> update 1 in-progress
✅ Task status updated!

> list in-progress
📋 Your Tasks:
─────────────────────────────────────
🔵 1. Buy groceries [IN-PROGRESS]
─────────────────────────────────────
```

## Project Structure

```
rust-cli-todo/
├── src/
│   ├── main.rs          # Entry point and REPL loop
│   ├── todo.rs          # Core todo logic and data structures
│   └── parse.rs         # Command parsing and handlers
├── Cargo.toml           # Dependencies
└── README.md
```

## Dependencies

- `serde` - Serialization/deserialization
- `serde_json` - JSON support
- `thiserror` - Error handling

## Roadmap

- [x] Basic CRUD operations
- [x] Persistent storage
- [x] Status filtering
- [ ] Due dates
- [ ] Priority levels
- [ ] Task categories/tags
- [ ] Export to CSV

## License

MIT License - feel free to use this code for learning!

## Connect

Following my journey learning Rust and Solana development:

- Twitter/X: [@eazicoder]
- GitHub: [@EaziCoder]

---
