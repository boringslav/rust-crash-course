use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

pub fn run () {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
}

pub fn shortcuts_for_panic_on_err(){
    let f = File::open("hello.txt").unwrap(); //if the result is the err varian uwrap will call panic for us
    let f = File::open("hello.txt").expect("Failed to open hello.txt");//same behavior like unwrap with the difference that we can create our panic message
}
//Error progagation
pub fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

pub fn shortcut_for_error_propagation() -> Result<String, io::Error>{
    let mut s = String::new();
// If the value of the Result is an Ok, the value inside the Ok will get returned from this expression, and the program will continue. If the value is an Err, the Err will be returned from the whole function as if we had used the return keyword so the error value gets propagated to the calling code.
    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}