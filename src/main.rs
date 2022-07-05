use colored::*;

use rand::prelude::*;

use std::cmp::Ordering::*;
use std::io::{Write, stdin, stdout};

fn main() {
    let target = rand::thread_rng().gen_range(1..=100);

    let mut guess: String = String::new();
    loop {
        // reading stdin to guess doesn't clear it
        // so we have to do it 
        guess.clear();

        print!("Enter your guess: ");
        
        // we need to flush the output so that the print above
        // will actually happen
        stdout()
            .flush()
            .expect("Could not flush output");

        stdin()
            .read_line(&mut guess)
            .expect("Could not read guess");

        // parse the guess into a u8
        let parse_result = guess.trim().parse::<u8>();
        let guess_parsed = match parse_result {
            Ok(v)    => v,

            // setting to 101 will ignore the guess
            // because of the next if statement
            Err(_) => 101
        };

        if guess_parsed > 100 || guess_parsed < 1 {
            continue;
        }

        match guess_parsed.cmp(&target) {
            Equal => { break },
            Less => { println!("{}", "Too low".red()); },
            Greater => { println!("{}", "Too high".red()); }
        }
    }

    println!("{}", "You win!".green());
}
