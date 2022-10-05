use std::io::stdin;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new();

    stdin()
        .read_line(&mut guess)
        .expect("Filed to read line");
    
    println!("You guessed: {}", guess);
}
