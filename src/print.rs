pub fn run(){
    //print to console
    println!("Hello from print.rs file!");

    //Basic formatting
    println!("Number: {}",1);
    println!("{} is from {}", "Borko", "Varna");  

    //Positional Arguments
    println!("{0} is from {1}", "Borko", "Varna");

    //Named Arguments
    println!("{name} is from {city}", name="Borko", city="Varna");

    //Placeholder traits
    println!("Decimal: {0} Hex: {:x} Binary: {:b} Octal: {:o}", 10, 10, 10);

    //Placeholder for debug trait
    println!("{:?}", (12,true, "hello"))
}