pub fn run() {
    //Strings are UTF-8 encoded
    let mut s = String::new(); //Create a new string
    s.push_str("Hello world!"); //Add to the end of the string

    let _d = "initial content (str)".to_string();
    let _d1 = String::from("initial content (str)");

    let s1 = String::from("Hello, ");
    // let h = s1[0]; this will throw an error because rust strings dont support indexing

    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    let slice = &s3[0..5];
    println!("Slice: {}", slice);
}
