use crate::get_user_input::get_user_input_float;

pub fn get_dimensions() {

    let mut new_rectangle = Rectangle {
        width: 0.0,
        length: 0.0,
    };

    new_rectangle.width = get_user_input_float("Please enter width of a rectangle:");
    new_rectangle.length = get_user_input_float("Please enter length of a rectangle:");

    let area = calculate_rectangle_area(new_rectangle);
    println!("Area of a rectangle: {area}");

}

struct Rectangle {
        width: f64,
        length: f64,
}


fn calculate_rectangle_area(dimension: Rectangle) -> f64 {
    dimension.width * dimension.length
}
