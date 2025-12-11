# Week 1: Getting Started with Rust

Focusing on: variables, data types, functions, ownership basics

## Projects I'm Working On

### hello_rust ✅
First project - learning syntax and cargo basics
- [ ] Variables (immutable vs mutable)
- [ ] Different data types
- [ ] println! macro

```bash
cd hello_rust && cargo run
```

### temperature_converter (todo)
Practicing functions and user input
- Celsius ↔ Fahrenheit conversion
- Using match statements
- Reading from stdin

### guessing_game (todo)
Learning loops and control flow
- Random number generation
- Loop until correct
- Using the rand crate

### calculator (todo)
Pattern matching practice
- Basic math operations
- Error handling (div by zero)
- match with enums

## Stuff to Try

Exercises in hello_rust/src/main.rs:

```rust
// variables
let x = 5;
let mut y = 10;
y = y + 5;

// types
let integer: i32 = 42;
let float: f64 = 3.14;
let boolean: bool = true;
let character: char = '🦀';

// tuples and arrays
let tuple = (500, 6.4, "hello");
let array = [1, 2, 3, 4, 5];
```

## Reading

- Rust Book chapters 3-4 (common concepts & ownership)
- Rust by Example for quick syntax reference

## Notes to Self

- Compiler errors are actually helpful, read them!
- Use `cargo check` for quick feedback
- `cargo clippy` for best practices
- Don't stress about ownership yet, get syntax down first
