use crate::get_user_input;

pub fn get_n_fibonacci() {

    let num:u32 = get_user_input::get_user_input_integer(
        "Please enter how many Fibonacci numbers you want to get: ");
    get_fibonacci_sequence(num);
}

fn get_fibonacci_sequence(num: u32) {
    let mut fibonacci_sequence: Vec<u128> = Vec::new();
                let mut counter: u32 = 1;
                while counter <= num {
                    if counter <= 2 {
                        fibonacci_sequence.push(1);
                    }
                    else {
                        fibonacci_sequence.push(fibonacci_sequence.last().unwrap() +
                            fibonacci_sequence.get(fibonacci_sequence.len() - 2).unwrap());
                    };
                    counter += 1;
                };
                println!("Fibonacci sequencte would be {}",
                         fibonacci_sequence.iter().map(|i| i.to_string()).
                             collect::<Vec<String>>().join(", "));
                println!("Fibonacci {num}th value is {}", fibonacci_sequence.last().unwrap());
}
