use std::collections::HashMap;

pub fn run(){
let mut scores = HashMap::new();//key:String, value:i32

scores.insert(String::from("Black"), 10);
scores.insert(String::from("White"), 20);

println!("Scores: {:?}", scores);


let teams = vec![String::from("Blue"), String::from("Yellow")];//["Blue", "Yellow"]
let initial_scores = vec![10,50];

let mut scores: HashMap<_,_> = teams.into_iter().zip(initial_scores.into_iter()).collect();
}