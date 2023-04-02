use std::io;
mod celsius_to_fahrenheit;
mod get_n_fibonacci;
mod get_rectangle_area;

fn main() {
    println!("Select what you want to do:");
    println!("   1.  Convert Celsius to Fahrenheit or Fahrenheit to Celsius");
    println!("   2.  Get a Fibonacci sequence of first N numbers");
    println!("   3.  Calculate rectangle area");
    println!("   4.  Do something else (not implemented)");
    println!("   ...");
    println!("   99. Test temporary code");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    match input.trim().parse() {
            Ok(num) => match num {
                1 => celsius_to_fahrenheit::celsius_to_fahrenheit(),
                2 => get_n_fibonacci::get_n_fibonacci(),
                3 => get_rectangle_area::get_dimensions(),
                4 => println!("Do something else (not implemented)"),
                99 => println!("Do something else (not implemented)"),
                _ => println!("Not a valid input"),
            },
            Err(_) => {
                println!("Not a valid input");
            }
        };
    }