//Tuples group together values of different types.
//Fixed length: They cant grow or shrink in size

pub fn run() {
    let person: (&str, &str, i8) = ("Borislav", "Stoyanov", 21);

    let (first_name, last_name, age) = person; //destructuring

    println!("{} is from {} and is {}", person.0, person.1, person.2);
    println!("{} is from {} and is {}", first_name, last_name, age);
}
