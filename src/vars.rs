//Variables hold primitive data or references to data 
//Variables are immutable by default
//Rust is a block-scoped language

pub fn run(){
    let name = "Borko";
    let mut age = 20; //mutable variable

    println!("My Name is {} and I am {}", name, age);
    age = 21;
    println!("My Name is {} and I am {}", name, age);
    
    //Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    //Assign multiple variables
    let (my_name, my_age) = ("Borko", 21);
    

}