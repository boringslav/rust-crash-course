use std::collections::HashMap;

pub fn run() {
    let mut scores = HashMap::new(); //key:String, value:i32

    scores.insert(String::from("Black"), 10);
    scores.insert(String::from("White"), 20);

    // println!("Scores: {:?}", scores);
    let team_name = String::from("Black");
    let score = scores.get(&team_name).unwrap(); //returns Option -> with unwrap get the value, panic if there is no value

    println!("Team: {} Score: {}", &team_name, &score);

    for (team, score) in &scores {
        println!("Team: {}, Score: {}", team, score);
    }

    let teams = vec![String::from("Blue"), String::from("Yellow")]; //["Blue", "Yellow"]
    let initial_scores = vec![10, 50];
    /*
     * For the parameters for the key and value types, however,
     * we use underscores, and Rust can infer the types that the hash map contains based on the types of the data in the * vectors
     */
    let scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    // println!("Hashmap: {:?}",scores);
}

pub fn ownership() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    /*
    For types that implement the Copy trait, like i32, the values are copied into the hash map. For owned values like String, the values will be moved and the hash map will be the owner of those values,
    */
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
}

pub fn updating_hash_map() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);

    /*
    The return value of the entry method is an enum called Entry that represents a value that might or might not exist.
    */
    scores.entry(String::from("Blue")).or_insert(60);
    scores.entry(String::from("Black")).or_insert(70);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}
pub fn updating_based_on_old_value(){
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}