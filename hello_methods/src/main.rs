#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }
}

impl Rectangle {
    fn parameter(&self) -> u32 {
        2 * (self.length + self.width)
    }
}

fn main() {
    let rectangle1 = Rectangle {
        length: 30,
        width: 40
    };
    println!("Paramter of the rectangle is => {}", rectangle1.parameter());
    println!("Area of the rectangle is => {}", rectangle1.area());
}
