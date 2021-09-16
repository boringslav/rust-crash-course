//Ownership Rules
/*Each variable in Rust has a variable that is called its owner.
 *There can only be one owner at a time
 *When the owner goes out of scope, the value will be dropped
 */

pub fn ownership() {
    {
        // s not valid here, its not yet declared
        let s = String::from("Hello");
        //do stuff with s
    } //When a variable goes out of scope rust calls a special function called drop (Rust automatically calls drop at the closing curly bracket)

    {
        //String type is allocated in the heap
        let mut s = String::from("Hello");
        //:: operator allows us to namespace this particular from function under the String type
        let s1 = s.clone(); //clone creates deep copy of the heap data
        s.push_str(", world!");
        println!("{}", s);
    }
}
