//use external package - rand
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    //we set number once from range of 1 to 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        //veriable guess is a mut string
        let mut guess = String::new();

        //read line and get success or error if fail just notifay 
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //string to int if we fail we notifay and try again
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("not a number");
                continue;
            },
        };

        println!("You guessed: {guess}");

        //comparing to our
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
