use std::io;

pub fn celsius_to_fahrenheit() {
    println!("Please enter known temperature base ('F' for Fahrenheit or 'C' for Celsius:");

    let mut basis:String = String::new();
    io::stdin().read_line(&mut basis).expect("Failed to read line");

    let basis = basis.chars().next().unwrap().to_ascii_uppercase();

    println!("You entered {basis}");

    let temperature: f32; // Declare the temperature variable before the loop
    loop {
        println!("Please enter the temperature you want to convert: ");
        let mut input:String = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read line");

        temperature = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            },
        };
        break;
    };
    match basis{
        'C' => {
            let outcome: f32 = temperature * (9.0/5.0) + 32.0;
            println!("Temperature in Fahrenheit of outcome is: {outcome}");
        },
        'F' => {
            let outcome: f32 = (temperature - 32.0) * (5.0 / 9.0);
            println!("Temperature in Celsius of outcome is: {outcome}");
        }
        _ => {
        println!("The basis is not supported");
        return; // or panic!(), or handle the error in some other way
        },
    };
}