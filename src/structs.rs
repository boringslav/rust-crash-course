#[derive(Debug)]
struct User {
    username: String,
    email: String,
    is_active: bool,
    sign_in_count: usize,
}

pub fn run() {
    let username = String::from("borko");
    let email = String::from("borko@example.com");
    let user1 = create_user(email, username);

    let user2 = User {
        username: String::from("Borkoto"),
        email: String::from("borkoto@example.com"),
        ..user1
    };

    println!("User1: {:?}", user1);
    println!("User2: {:?}", user2);
}

fn create_user(email: String, username: String) -> User {
    return User {
        email,
        username,
        is_active: true,
        sign_in_count: 1,
    };
}
