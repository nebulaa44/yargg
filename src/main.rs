use rand::prelude::*;

fn main() {
    let target = rand::thread_rng().gen_range(1..=100);
    println!("{target}");
}
