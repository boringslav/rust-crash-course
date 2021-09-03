//Primitive str = Immutable fixed-length string somewhere in memory.
//String = Growable, heap-allocated data structure - Use when you need to modify of own string data

pub fn run(){
let mut hello = String::from("Hello");

println!("Length {}", hello.len()); // works on both types of strings

hello.push(' '); //push char works only on String
hello.push_str("World");//push string works only on String

println!("{}",hello);

//Capacity in bytes
println!("Capacity: {} bytes",hello.capacity());

//Loop through string by whitespace
for word in hello.split_whitespace() {
    println!("{}",word);
}

//Create string with capacity
let mut s = String::with_capacity(10);
s.push('a');
s.push('b');

println!("{}",s);

//Assetion testing
assert_eq!(2,s.len()); //if assertion fails -> error, else nothing 

}