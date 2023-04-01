use std::io;

pub fn get_n_fibonacci() {

    loop {
        println!("Please enter how many Fibonacci numbers you want to get: ");

        let mut input:String = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse() {
            Ok(num) => {
                let mut fibonacci_sequence: Vec<u128> = Vec::new();
                let mut counter: u8 = 1;
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
                println!("Fibonacci {input}|th value is {}", fibonacci_sequence.last().unwrap());
                break;
            },
            Err(_) => {
                println!("Not a valid input");
                continue;
            },
        }
    }
}
