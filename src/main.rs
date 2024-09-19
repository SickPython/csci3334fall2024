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
    let secret: i32 = 42; 
    let mut guess: i32;
    let mut attempts: i32 = 0;

    loop {
        guess = 42; 
        attempts += 1;

        let result = check_guess(guess, secret);

        if result == 0 {
            println!("Correct! You guessed the secret number.");
            break;
        } else if result == 1 {
            println!("Too high!");
        } else {
            println!("Too low!");
        }
    }

    println!("It took you {} guesses.", attempts);
}