mod generics;
fn main() {
    let number_list = vec![32,4,5,67,7];
    let largest_number = generics::get_largest(number_list);
    println!("The largest number in the vector is: {}", largest_number);

}

