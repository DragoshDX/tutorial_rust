use std::{io, cmp::Ordering};
use rand::Rng;

fn main() {
    println!("Hello, world!");

    let secret_number  = rand::thread_rng()
        .gen_range(1..=100);

    loop {
        let mut guess = String::new();

        println!("Please input your guess.");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");

                break;
            }
        }
    }
}
