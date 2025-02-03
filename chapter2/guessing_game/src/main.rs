use std::{
    cmp::Ordering,
    io::{stdin, stdout, Write},
};

fn main() {
    let secret_num = rand::random_range(1..=100);

    let mut guess: String;

    loop {
        guess = String::new();

        print!("Enter guess: ");
        stdout().flush().expect("Error: Failed to flush stdout");

        stdin()
            .read_line(&mut guess)
            .expect("Error: Failed to read from stdin");

        let guess_num: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a valid number!");
                continue;
            }
        };

        match guess_num.cmp(&secret_num) {
            Ordering::Greater => println!("Too large!"),
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("Correct!");
                break;
            }
        }
    }
}
