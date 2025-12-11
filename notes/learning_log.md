# My Rust Learning Notes

## Week 1: Getting Started

### Date: December 11, 2025

#### What I Learned Today
- 

#### Code I Wrote
- Created hello_rust project

#### Questions
- 

#### Challenges
- 

#### Tomorrow's Goals
- 

---

## Useful Commands

```bash
# Create new project
cargo new project_name

# Run project
cargo run

# Build project
cargo build

# Check for errors (faster)
cargo check

# Format code
cargo fmt

# Run linter
cargo clippy

# Run tests
cargo test
```

## Key Concepts

### Variables
```rust
let x = 5;        // immutable
let mut y = 10;   // mutable
```

### Data Types
- **Integers:** i8, i16, i32, i64, i128, isize (also u* for unsigned)
- **Floats:** f32, f64
- **Boolean:** bool (true/false)
- **Character:** char ('a', '🦀')

### Ownership Rules
1. Each value has a single owner
2. When owner goes out of scope, value is dropped
3. Only one owner at a time

---
