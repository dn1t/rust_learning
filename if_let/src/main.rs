#[derive(Debug)]
enum UsState {
    Alabama,
    // ...
}

enum Coin {
    _Penny,
    Quarter(UsState),
}

fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alabama);

    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }

    println!("{}", count);
    count = 0;

    let coin = Coin::Quarter(UsState::Alabama);

    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }

    println!("{}", count);
}
