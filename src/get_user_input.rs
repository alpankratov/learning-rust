use std::io;

pub fn get_user_input_float(prompt: &str) -> f64 {
    loop {
        println!("{prompt}");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse() {
            Ok(x) => return x,
            Err(_) => {
                println!("Please enter a valid value");
                continue;
            },
        };
    }
}

pub fn get_user_input_integer(prompt: &str) -> u32 {
    loop {
        println!("{prompt}");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse() {
            Ok(x) => return x,
            Err(_) => {
                println!("Please enter a valid value");
                continue;
                },
            };
        };
    }