use std::io;

fn main() {
    println!("=== Temperature Converter ===\n");

    loop {
        println!("Choose conversion:");
        println!("1. Celsius to Fahrenheit");
        println!("2. Fahrenheit to Celsius");
        println!("3. Exit");
        println!("\nEnter choice (1-3): ");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input!\n");
                continue;
            }
        };

        match choice {
            1 => celsius_to_fahrenheit(),
            2 => fahrenheit_to_celsius(),
            3 => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice!\n"),
        }
    }
}

fn celsius_to_fahrenheit() {
    println!("\nEnter temperature in Celsius: ");

    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");

    let celsius: f64 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number!\n");
            return;
        }
    };

    let fahrenheit = (celsius * 9.0 / 5.0) + 32.0;
    println!("{}°C = {:.2}°F\n", celsius, fahrenheit);
}

fn fahrenheit_to_celsius() {
    println!("\nEnter temperature in Fahrenheit: ");

    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");

    let fahrenheit: f64 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number!\n");
            return;
        }
    };

    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
    println!("{}°F = {:.2}°C\n", fahrenheit, celsius);
}
