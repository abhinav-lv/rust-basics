use std::cmp::Ordering;
use std::io;
use tinyrand::{Rand, RandRange, Seeded, StdRand};
use tinyrand_std::clock_seed::ClockSeed;

pub fn guess_the_number() {
    println!("Guess the number!");

    let seed = ClockSeed::default().next_u64();
    let mut rand = StdRand::seed(seed);
    let secret = rand.next_range(0..100);

    loop {
        let mut guess = String::new();
        println!("Please enter your guess: ");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
