//Functions - Used to store blocks of code for re-use
pub fn run() {
    greeting("Hello", "Borislav");
    let sum: i32 = sum(5, 5);

    println!("Sum is {}", sum);

    //Closure
    let add_nums = |n1: i32, n2: i32| n1 + n2;
    println!("Closure sum: {}", add_nums(5, 5));
}
fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name)
}

fn sum(n1: i32, n2: i32) -> i32 {
    //-> return type
    n1 + n2 //return values
            //return value is without ;
}
