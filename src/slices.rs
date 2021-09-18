pub fn run() {
    let message = String::from("Hello world");
    let first_word = get_first_word(&message);

    println!("The first word is: {}", first_word);
}

fn get_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    return &s[..];
}
