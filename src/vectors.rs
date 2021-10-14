//Vectors can only store items that are the same type
pub fn run() {
    let _v: Vec<i32> = Vec::new(); //Create a new empty vector of i32
    let _v1 = vec![1, 2, 3]; //Creates a vector of i32 with 1,2,3 inside

    let mut mutable_vector = Vec::new(); //empty vector
    mutable_vector.push(2); //add element to vector
    mutable_vector.push(3);
    mutable_vector.push(23);
    mutable_vector.push(45);
    println!("Vector: {:?}", mutable_vector);

    let third: &i32 = &mutable_vector[2]; //Indexing syntax. If there is no  el the program will panic because it references a nonexistent element
    let _second = &mutable_vector[1]; //Indexing syntax

    println!("Third element is: {}", third);

    match mutable_vector.get(1) {//get syntax gives an Option<&T> If there is no el the program will return None value and wont crash
        Some(second) => println!("The second element is {}", second),
        None => println!("There is no second element"),
    }

    {
        let _vec_scope = vec![1, 23, 4, 5];
        //do stuff with v
    } // <- v goes out of scope and is freed here
}
pub fn iterate_values() {
    let mut v = vec![100,50,25];
    println!("{:?}", v);

    for i in &mut v {
        *i +=50;//dereference operator
        println!("Element: {}",i);
    }

  
}
pub fn using_enum_to_store_multiple_types(){
    #[derive(Debug)]
    enum SpreadsheetCell{
        ClientNumber(i32),
        Name(String),
        Balance(f64),
    }

    let row = vec![
        SpreadsheetCell::ClientNumber(1),
        SpreadsheetCell::Name(String::from("Borislav")),
        SpreadsheetCell::Balance(65.10),
    ];

    println!("Enum Vector {:?}",row);
}