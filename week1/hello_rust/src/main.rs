fn main() {
    // Getting Started with Rust
    println!("Hello, world!");
    println!("Welcome to Rust! 🦀");

    // Exercise 1: Create an immutable variable
    let name = "Siddhartha Devkota";
    println!("Hello, {}!", name);

    // Exercise 2: Create a mutable variable
    let mut count = 0;
    count = count + 1;
    println!("Count: {}", count);

    // Exercise 3: Different data types
    let integer: i32 = 42;
    let float: f64 = 3.14;
    let boolean: bool = true;
    println!("Number: {}, Pi: {}, Bool: {}", integer, float, boolean);

    // Exercise 4: Tuples and arrays
    let tuple = (100, "Rust", true);
    let array = [1, 2, 3, 4, 5];
    println!("First from tuple: {}, First from array: {}", tuple.0, array[0]);

}
