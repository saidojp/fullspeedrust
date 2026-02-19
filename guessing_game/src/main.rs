use std::io;

fn main() {
    println!("Guess the number!");
    println!("Type shi");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");
    println!("You guessed {guess}");
}
