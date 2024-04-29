struct User {
    _active: bool,
    _username: String,
    email: String,
    _sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn main() {
    let mut user1 = User {
        _active: true,
        _username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        _sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    // let _user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };
    let _user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    let _subject = AlwaysEqual;
}

fn _build_user(email: String, username: String) -> User {
    User {
        _active: true,
        _username: username,
        email,
        _sign_in_count: 1,
    }
}
