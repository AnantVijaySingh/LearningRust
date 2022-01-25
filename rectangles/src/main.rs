fn main() {

    // Simple way
    let width = 50;
    let height = 30;

    println!(
        "The area of the rectangle is {} square pixels",
        area_simple(width, height)
    );

    // Refactored with tuples
    let rectangle1 = (50, 30);

    println!(
        "The area of the rectangle is {} square pixels",
        area_tuples(rectangle1)
    );

    // Refactoring with structs
    let rectangle2 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels",
        area_struct(&rectangle2)
    );


}

// Simple way
fn area_simple(width: u32, height: u32) -> u32 {
    width * height
}

// Refactored with tuples
fn area_tuples(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// Refactoring with structs
struct Rectangle {
    width: u32,
    height: u32,
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}