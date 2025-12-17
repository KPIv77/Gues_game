use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // random value for guess by rand libraly
    let secret_number = rand::rng().random_range(1..100);

    // println!("The secret number is: {secret_number}");

    loop {

        println!("Please input your guess.");

        // create variable none value 
        let mut guess: String = String::new();

        // input value by io libraly from keybord and write value into guess
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to  read line");

        // transform type input value to u32 (0 <= value)
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you guessed: {guess}");

        // condition for if value equal randome_rang or not equal
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
