fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");

    change(&mut s);

    println!("{s}");

    let r1 = &mut s;
    // let r2 = &mut s;

    // println!("{}, {}", r1, r2);
    println!("{}", r1);

    {
        let _r1 = &mut s;
    }
    let _r2 = &mut s;

    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    // let r3 = &mut s;

    // println!("{}, {}, and {}", r1, r2, r3);
    println!("{} and {}", r1, r2);

    let r3 = &mut s;
    println!("{}", r3);

    // let reference_to_nothing = dangle();
    let reference_to_nothing = no_dangle();

    println!("{reference_to_nothing}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
