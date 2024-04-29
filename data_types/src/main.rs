use std::io;

fn main() {
    let _x = 2.0;
    let _y: f32 = 3.0;

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;
    let truncated1 = -5.0 / 3.0;
    let remainder = 43 % 5;

    println!(
        "sum: {}, difference: {}, product: {}, quotient: {}, truncated: {}, truncated1: {}, remainder: {}",
        sum, difference, product, quotient, truncated, truncated1, remainder
    );

    let _t = true;
    let _f: bool = false;

    let _c = 'z';
    let _z: char = 'ℤ';
    let _heart_eyed_cat = '😻';

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_, y, _) = tup;

    println!("The value of y is: {y}");

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("{} {} {}", five_hundred, six_point_four, one);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];

    println!("{a:?} {first} {second}");

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
    println!("{months:?}");

    let a = [3; 5];
    println!("{a:?}");

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
