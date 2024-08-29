use std::io;

fn main() {
    let x = 2.6;

    let y: f32 = 3.6;
    println!("{} {}", x, y);

    loop {
        println!("Do you want to:\n1) Add\n2) Subtract\n3) Multiply\n4) Divide\n5) Truncate\n6) Remainder");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };
        if input > 6 || input < 1 {
            println!("Please type a number between 1 and 6!");
            continue;
        }

        match input {
            // 1 => println!("{} + {} = {}", x, y, add(x, y)),
            2 => println!("{} - {} = {}", x, y, diff(x, y)),
            // 3 => println!("{} * {} = {}", x, y, mul(x, y)),
            4 => println!("{} / {} = {}", x, y, div(x, y)),
            // 5 => println!("{} / {} = {}", x, y, truncate(x, y)),
            // 6 => println!("{} % {} = {}", x, y, rem(x, y)),
            _ => println!("Invalid input"),
        }
        break;
    }
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn diff(x: f32, y: f32) -> f32 {
    x - y
}

fn mul(x: u32, y: u32) -> u32 {
    x * y
}

fn div(x: f32, y: f32) -> f32 {
    x / y
}

fn truncate(x: i32, y: i32) -> i32 {
    x / y
}

fn rem(x: i32, y: i32) -> i32 {
    x % y
}
