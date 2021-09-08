use std::sync::mpsc::Receiver;

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    println!("Hello, world!");

    let width1 = 30;
    let height1 = 50;
    let rect1 = &Rectangle {
        width: 30,
        height: 50
    };

    println!(
        "he area of the rectangle is {} square pixels.",
        // area(width1, height1)
        rect1.area2()
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

impl Rectangle {
    fn area2(&self) -> u32 {
        self.width * self.height
    }
}
