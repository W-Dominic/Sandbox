struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    let user2 = build_user("test@protonmail.com".to_string(),
                            "test".to_string());
    /*
    let user3 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("next@proton.me"),
        sign_in_count: user1.sign_in_count,
    };
    */
    let user3 = User {
        email: String::from("alt@pm.me"),
        ..user1
    };
    println!("{}", user1.email);
    println!("{}", user2.email);
    println!("{}", user3.email);
}

//if we use the same parameter names, assignment is easy!
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
