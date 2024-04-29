use std::io;

fn main() {
    println!("Enter n");

    let mut n = String::new();

    io::stdin().read_line(&mut n).expect("Failed to read line");

    let n: u8 = n.trim().parse().expect("n entered was not a number");

    let result = fibonacci(n);

    println!("{result}");
}

fn fibonacci(n: u8) -> u32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}
