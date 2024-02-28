use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("🟢 Guess the number 🟢");

    let secret: u32 = gen_rand_num(false);
    let mut tries_count: u8 = 0;

    loop {
        println!("\n⏱️ Num of Tries: {tries_count}");
        let guess: u32 = take_guess();
        let correct: bool = cmp_guess(guess, secret);
        
        if correct {
             break
        } else {
            tries_count = tries_count + 1;
        }
        
    }
}

fn gen_rand_num(print_it: bool) -> u32 {
    let secret_num = rand::thread_rng().gen_range(1..=100);

    if print_it {
        println!("🔑 The secret number is: {secret_num}");
    }

    secret_num
}

fn take_guess() -> u32 {
    println!("Please input your guess.");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    let guess: u32 = guess.trim().parse().expect("Please type a number.");

    println!("You guessed: {guess}");

    guess
}

fn cmp_guess(guess: u32, secret: u32) -> bool {
    // Comparison
    match guess.cmp(&secret) {
        Ordering::Less => {
            println!("🔴👎 - It is too smalll");
            false
        },
        Ordering::Greater => {
            println!("🔴👍 - It is too big");
            false
        },
        Ordering::Equal => {
            println!("Correct! 🎊");
            true
        }
    }
}
