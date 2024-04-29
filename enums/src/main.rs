#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    _Quit,
    _Move { x: i32, y: i32 },
    Write(String),
    _ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        dbg!(&self);
    }
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    route(home);
    route(loopback);

    let m = Message::Write(String::from("hello"));
    m.call();

    let _some_number = Some(5);
    let _some_char = Some('e');

    let _absent_number: Option<i32> = None;
}

fn route(ip: IpAddr) {
    dbg!(ip);
}
