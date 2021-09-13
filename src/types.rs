/*
Primitive Types --
    Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, (number of bits they take in memory)
    Floats: f32, f64
    Boolean: (bool) Rust will not automatically try to convert non-Boolean types to a Boolean.
    Characters: char
    Tuples
    Arrays
    */

/*Rust is a statically typed language, which means that it must know the types of all variables at compile time,
however, the compiler can usually infer what type we want to use based on the value and how we use it. */

pub fn run() {
    //Default is i32
    let x = 1;

    //Default is f64
    let y = 2.5;

    //Add explicit type
    let y: i64 = 4555555555;

    //Find max size
    println!("Max i32: {}", std::i32::MAX);

    let a1: char = 'a'; //Single '
    let face: char = '\u{1f600}'; //unicode

    println!("Emoji: {}", face);
}

pub fn bool() {
    let mut x = 1;

    // if x { //Will throw error
    //     x+= 1;
    // }

    if x != 0 {
        x += 1;
    }

    println!("Number {}", x);
}
