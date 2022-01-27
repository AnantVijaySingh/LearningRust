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

    // dbg and structs
    let scale = 2;

    let rectangle3 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    // The dbg! macro takes ownership of an expression, prints the file and line number of where
    // that dbg! macro call occurs in your code along with the resulting value of that expression,
    // and returns ownership of the value.

    dbg!(&rectangle3);

    println!(
        "The area of the rectangle is {} square pixels",
        area_struct(&rectangle2)
    );

    println!("Rectangle2 is {:#?}", rectangle2);
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
#[derive(Debug)] // outer attribute trait
struct Rectangle {
    width: u32,
    height: u32,
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}