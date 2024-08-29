use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    print!("{}[2J", 27 as char);

    println!("Welcome to the guessing game!");

    let rand: i32 = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please guess a name!");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        let guess: i32 = match guess.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&rand) {
            Ordering::Greater => println!("Its lower than {} !", guess),
            Ordering::Less => println!("Its higher than {} !", guess),
            Ordering::Equal => {
                println!("Mah man!");
                break;
            }
        }
    }
}
