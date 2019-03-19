extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    let secret = rand::thread_rng().gen_range(1,101);
    
    println!("Number guessing game!");
    loop {
        let mut guess = String::new();
        println!(" Please enter your guess:");
        io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        match guess.cmp(&secret) {
            Ordering::Less => println!("Number is too small"),
            Ordering::Equal => {
                println!("You win!!!");
                break;
            },
            Ordering::Greater => println!("Number is too big"),
        }
    }
    
}
