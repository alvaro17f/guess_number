use std::{io::stdin, process};

use rand::Rng;
use text_io::read;

fn main() {
    loop {
        let number: u32 = randomize();
        let mut count: u32 = 0;
        print!("\x1B[2J\x1B[1;1H");
        println!("WELCOME TO... GUESS THE NUMBER");
        println!();
        println!("Please, insert a number between 0 and 100:");
        println!();
        loop {
            let guess: u32 = read!();
            count += 1;
            if guess == number {
                print!("\x1B[2J\x1B[1;1H");
                println!();
                println!("WINNER HERE");
                println!("{count} trials");
                break;
            }
            if guess > number {
                print!("\x1B[2J\x1B[1;1H");
                println!("WELCOME TO... GUESS THE NUMBER");
                println!();
                println!("Please, insert a number in order to guess the mine:");
                println!();
                println!("your number {} is bigger", guess);
                println!("Try again:");
                println!();
            } else {
                print!("\x1B[2J\x1B[1;1H");
                println!("WELCOME TO... GUESS THE NUMBER");
                println!();
                println!("Please, insert a number in order to guess the mine:");
                println!();
                println!("your number {} is smaller", guess);
                println!("Try again:");
                println!();
            }
        }
        println!("Would you like to try again? (Y/n): ");
        let prompt: &mut String = &mut String::new();
        stdin().read_line(prompt).expect("failed to read line");
        if let "y\n" | "Y\n" | "yes\n" | "Yes\n" | "YES\n" | "\n" = prompt.as_str() {
            continue;
        } else {
            process::exit(1);
        } 
    }
}

fn randomize() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..=100)
}
