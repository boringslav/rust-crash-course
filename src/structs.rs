struct User {
    username: String,
    email: String,
    is_active: bool,
    sign_in_count: usize,
}

pub fn run() {
    let username = String::from("borko");
    let email = String::from("borko@example.com");
    let user1 = create_user(email,username);

    let user2 = User {
        username: String::from("Borkoto"),
        email: String::from("borkoto@example.com"),
        ..user1
    };

    println!("User2: {}, Email: {} isActive: {} sign in count: {}",user1.username, user1.email, user1.is_active,user1.sign_in_count);
    println!("User2: {}, Email: {} isActive: {} sign in count: {}",user2.username, user2.email, user2.is_active,user2.sign_in_count);
}

fn create_user(email: String, username: String) -> User {
    return User {
        email,
        username,
        is_active:true,
        sign_in_count: 1,
    };
}
