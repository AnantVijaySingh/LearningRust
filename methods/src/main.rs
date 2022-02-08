fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!(
        "The ares of the rectangle is {} square pixels",
        rect1.area()
    );

    if rect1.width() {
        println!("The rectangle has a non-zero width of {} pixels", rect1.width);
    }

    println!("Can rect1 hold rect2 {}", rect1.can_hold(&rect2));
    println!("Can rect2 hold rect3 {}", rect2.can_hold(&rect3));
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Methods must have a parameter named self of type Self for their first parameter
impl Rectangle{
    fn area(self: &Self) -> u32 {
        self.width * self.height
    }

// Method can have the same name as one of the struct’s fields
    fn width(&self) -> bool { // Shot hand for self: &Self
        self.width > 0
    }

    fn can_hold(self: &Self, other_rectangle: &Rectangle) -> bool {
        self.width > other_rectangle.width && self.height > other_rectangle.height
    }

}


