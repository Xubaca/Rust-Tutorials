use clearscreen::clear;
use std::io;

fn main() {
    loop {
        clear().expect("Failled to clear screen");
        println!("Hello please enter your age: ");
        let mut age = String::new();
        io::stdin()
            .read_line(&mut age)
            .expect("Failled to read age");
        let age: i32 = match age.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That is not a valid age");
                continue;
                0 //for testing purposes;
            }
        };

        clear().expect("Failled to clear screen");
        println!("You have experieced the following years: ");
        for i in 0..=age {
            println!("{}", i);
        }
        break;
    }
}
