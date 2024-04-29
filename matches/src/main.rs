#[derive(Debug)]
enum UsState {
    _Alabama,
    Alaska,
    // ...
}

enum Coin {
    Penny,
    _Nickel,
    _Dime,
    Quarter(UsState),
}

fn main() {
    value_in_cents(Coin::Quarter(UsState::Alaska));
    let value = value_in_cents(Coin::Penny);

    println!("{}", value);

    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);

    let dice_roll = 9;

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::_Nickel => 5,
        Coin::_Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
