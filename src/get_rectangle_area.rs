use crate::get_user_input::get_user_input_float;

pub fn get_dimensions() {

    let mut new_rectangle = Rectangle {
        width: 0.0,
        length: 0.0,
    };

    new_rectangle.width = get_user_input_float("Please enter width of a rectangle:");
    new_rectangle.length = get_user_input_float("Please enter length of a rectangle:");

    println!("Rectangle is {:?}", new_rectangle);
    println!("Area of a rectangle: {}", new_rectangle.area());
}

#[derive(Debug)]
struct Rectangle {
        width: f64,
        length: f64,
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.length * self.width
    }
}

