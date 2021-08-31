use std::{num::Wrapping, ops::Add};

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    println!("Hello, world!");

    let mut r = 5;
    println!("the value of is: {}", r);
    r = 6;
    println!("the value of is: {}", r);

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();

    let spaces = 57u8;
    let a = 1;
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5];
    another_function();
    print_labeled_measurement(5, 'h');

}

fn another_function(){
    println!("Another function.");

}
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}