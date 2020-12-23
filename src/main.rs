use std::io;
use std::time::{SystemTime, UNIX_EPOCH};
fn main() {
    let random_number: u32 = generate_random_number(101);
    println!("{}", random_number);

    println!("A number has been between 0 and 100 has been generated. Guess the number!");

    let mut guess: i32 = get_guess();

    let random_number = random_number as i32;
    while guess != random_number {
        if guess > random_number {
            println!("Too high!");
            guess = get_guess();
        } else if guess < random_number {
            println!("Too low!");
            guess = get_guess();
        }
    }

    println!("Bingo!");


}

fn get_guess() -> i32 {
    let guess: String = read_guess_from_stdin();
    get_guess_as_i32(guess)
}

fn read_guess_from_stdin() -> String {
    let mut guess = String::new();
    match io::stdin().read_line(&mut guess) {
        Ok(_) => {},
        Err(e) => eprintln!("error: {}", e),
    }
    guess
}

fn get_guess_as_i32(guess: String) -> i32 {
    let guess_int: i32 = match guess.trim().parse::<i32>() {
        Ok(v) => v,
        Err(_) => panic!("Sorry, could not understand your guess."),
    };
    guess_int
}


/*
Generates a random number between 0 and m
*/
fn generate_random_number(m: u32) -> u32 {
    let multiplier: u32 = 1103515245;
    let increment: u32 = 12345;
    let seed: u32 = get_time_since_unix_epoch_in_nanoseconds();

    (multiplier.wrapping_mul(seed) + increment) % m
}

fn get_time_since_unix_epoch_in_nanoseconds() -> u32 {
    SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .subsec_nanos()
}


