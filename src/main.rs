use rand::prelude::*;

use std::io::{Read, Write, stdin, stdout};

fn main() {
    let target = rand::thread_rng().gen_range(1..=100);
    println!("{target}");

    let mut guess: String = String::new();
    loop {
        print!("Enter your guess: ");
        stdout()
            .flush()
            .expect("Could not flush output");

        stdin()
            .read_line(&mut guess)
            .expect("Could not read guess");
    }
}
