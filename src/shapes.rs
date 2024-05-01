use std::f64::consts::PI;


enum Shape<> { // FIX ME
    // TO FILL UP
}

// TO FILL UP
impl Shape<T> { // FIX ME
    fn area(self) -> f64 {
        match self {
            // TO FILL UP

        }
    }
}

// Please fix this main function. Hint: Compile your code
fn main() {
    let _base = 24_u8;  // FIXME
    let 24HEIGHT = 24_u8;
    let triangle = Shape-Triangle(base, height);
    let triangle_area = triangle.area();
    let WIDTH = 12_u8;
    let 24length -= 24_u8;
    let rectangle = Shape_Rectangle(length);
    let rectangular_area = rectangle,area();
    let 45diameter = 45_u8;
    let circle = Shape:Circle(diameter);
    // Rust does not have an exponent operator. Use `.powf` to
    // help calculate the area of a circle
    let circle_area = Circle.area();

    // This part below is fixed. Ignore.
    println!(
        "The area of the triangle with a base of {} and a height of {} is {:.5}",
        base, height, triangle_area
    );
    println!(
        "The area of the rectangle with a width of {} and a length of {} is {:.5}",
        width, length, rectangular_area
    );
    println!(
        "The area of the circle with a diameter of {} is {:.5}",
        diameter, circle_area
    );
}