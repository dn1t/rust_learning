fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear();

    println!("'{}', {}", s, word);

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("'{}' '{}'", hello, world);

    let _slice = &s[0..2];
    let _slice = &s[..2];

    let len = s.len();

    let _slice = &s[3..len];
    let _slice = &s[3..];

    let _slice = &s[0..len];
    let _slice = &s[..];

    // let mut s = String::from("hello world");

    // let word = first_word_slices(&s);

    // s.clear();

    // println!("'{}', {}", s, word);

    let my_string = String::from("hello world");

    let _word = first_word_slices(&my_string[0..6]);
    let _word = first_word_slices(&my_string[..]);
    let _word = first_word_slices(&my_string);

    let my_string_literal = "hello world";

    let _word = first_word_slices(&my_string_literal[0..6]);
    let _word = first_word_slices(&my_string_literal[..]);
    let word = first_word_slices(my_string_literal);

    println!("{}", word);

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_slices(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
