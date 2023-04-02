use std::io;

pub fn get_dimensions() {

    let mut new_rectangle = Rectangle {
        width: 0.0,
        length: 0.0,
    };

    loop {
        println!("Please enter width of a rectangle: ");
        let mut width:String = String::new();
        io::stdin().read_line(&mut width).expect("Failed to read line");

        new_rectangle.width  = match width.trim().parse() {
            Ok(x) => x,
            Err(_) => {
                println!("Please enter a valid value");
                continue;
            }
        };

        println!("Please enter length of a rectangle: ");
        let mut length:String = String::new();
        io::stdin().read_line(&mut length).expect("Failed to read line");

        new_rectangle.length = match length.trim().parse() {
            Ok(x) => x,
            Err(_) => {
                println!("Please enter a valid value");
                continue;
            }
        };
        break;
    };
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
