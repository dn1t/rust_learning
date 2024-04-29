use std::io;

fn main() {
    let mut unit = String::new();
    let mut temp = String::new();

    println!("Enter unit");
    io::stdin()
        .read_line(&mut unit)
        .expect("Failed to read line");

    println!("Enter temperature");
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");

    let unit = unit.trim();
    let temp: f64 = temp
        .trim()
        .parse()
        .expect("Temperature entered was not a number");

    if unit == "F" {
        let c = (5.0 / 9.0) * (temp - 32.0);
        println!("{c}°C")
    } else if unit == "C" {
        let f = (9.0 / 5.0) * temp + 32.0;
        println!("{f}°F")
    } else {
        println!("Invalid unit");
    }
}
