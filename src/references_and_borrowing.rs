//At any given time, you can have either one mutable reference or any number of immutable references.
//References must always be valid.

pub fn run() {
    let mut s1 = String::from("hello");
    let len = calculate_length(&s1);
    //The &s1 syntax let us create a reference that refers the value of s1 but does not own it
    //Because it does not own it, the value it points to will not be dropped when the reference goes out of scope
    change(&mut s1);
    println!("Length is {}", len);
    println!("Mutable references {}", s1); //You can have only one mutable reference
}
fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} //Here, s goes out of scope. But because it doesnt have ownership of what it refers to, nothing happens
  // When functions have references as parameters instead of the actual values, we won’t need to return the values in order
  //to give back ownership, because we never had ownership.

fn change(some_string: &mut String) {
    some_string.push_str(", neshto")
}
