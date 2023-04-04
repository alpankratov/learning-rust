use std::cmp::Ordering;
use rand::Rng;
use crate::get_user_input;


pub fn guessing_game() {

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let guess:u32 = get_user_input::get_user_input_integer(
        "Please input your guess (between 1 and 100): ");

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!!!");
                break;
            },
        };
    }
}


