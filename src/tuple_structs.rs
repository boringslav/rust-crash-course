pub fn run() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    println!(
        "First color: {} Second Color: {} Third Color: {}",
        black.0, black.1, black.2
    );
}
