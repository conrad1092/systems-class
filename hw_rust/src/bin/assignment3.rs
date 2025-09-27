// Assignment 3: Guessing Game
// Simple guessing game that compares guesses to a secret number.

fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn main() {
    let secret: i32 = 27;
    let mut attempts: i32 = 0;
    let mut guess: i32 = 0;

    loop {
        attempts += 1;
        let result = check_guess(guess, secret);

        if result == 0 {
            println!("Guess #{attempts}: {guess} — Correct!");
            break;
        } else if result == 1 {
            println!("Guess #{attempts}: {guess} — Too high");
            guess -= 1;
        } else {
            println!("Guess #{attempts}: {guess} — Too low");
            guess += 1;
        }
    }

    println!("Total guesses: {attempts}");
}
