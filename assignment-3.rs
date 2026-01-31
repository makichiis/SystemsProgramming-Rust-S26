fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret { return 0; }
    else if guess > secret { return 1; }
    -1
}

fn main() {
    let mut secret = 512;

    let guesses = [1, 2048, 2, 1024, 4, 512, 8, 256, 16, 128, 32, 64];
    let mut guesses_made = 0;

    for guess in guesses {
        let res = check_guess(guess, secret);
        guesses_made += 1;

        print!("{}: ", guess);
        if res == 0 {
            println!("Correct!");
            break;
        } else if res == -1 {
            println!("Too low.");
        } else {
            println!("Too high!");
        }
    }
    println!("Game took {} guesses.", guesses_made);
}

