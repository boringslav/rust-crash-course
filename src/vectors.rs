//Vectors - resizable arrays
use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    //Re-assign value
    numbers[2] = 50;

    //Add to vector
    numbers.push(5);

    //Pop off last value
    numbers.pop();
    println!("{:?}", numbers);

    //Get single value
    println!("Single value: {}", numbers[0]);

    //Get array length
    println!("Vector length: {}", numbers.len());

    //Arrays are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers)); //each number is 4 bytes

    //Get Slice
    let slice: &[i32] = &numbers[0..2]; //excludes item on index 2
    println!("Slice: {:?}", slice);

    //Loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    //Loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("Numbers Vec: {:?}", numbers);
}
