use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    println!("GUESS THE SECRET NUMBER");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let gue: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("{err}");
                continue;
            }
        };
        println!("You guessed: {guess}"); 
        match gue.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!: {secret_number}");
                break;
            }
        }
    }
 }
