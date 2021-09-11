//Arrays - Fixed list where elements are the same data types
use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5]; //has to be exact (no empty positions)
    let months = ["January", "February", "March", "April", "May"];
    let same_value_array = ["value"; 5]; //same value array

    //Re-assign value
    numbers[2] = 50;

    println!("Same value array {:?}", same_value_array);

    //Get single value
    println!("Single value: {}", numbers[0]);

    //Get array length
    println!("Array length: {}", numbers.len());

    //Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers)); //each number is 4 bytes

    //Get Slice
    let slice: &[i32] = &numbers[0..2]; //excludes item on index 2
    println!("Slice: {:?}", slice);
}
