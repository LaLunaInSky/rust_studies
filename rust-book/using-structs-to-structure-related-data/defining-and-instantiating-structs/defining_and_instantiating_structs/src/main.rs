fn main() {
    let mut user_01 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someaone@example.com"),
        sign_in_count: 1,
    };

    user_01.email = String::from("anotheremail@example.com");

    let user_02 = build_a_user(
        String::from("someone02@example.com"),
        String::from("someusername234")
    );

    println!(
        "The user_01 is: {user_01:?}\nThe user_02 is: {user_02:?}"
    );

    let user_03 = User {
        email: String::from("another@example.com"),
        ..user_01
    };

    println!(
        "The user_03 is: {user_03:?}"
    );
}

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_a_user(
    email: String,
    username: String
) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1
    }
}