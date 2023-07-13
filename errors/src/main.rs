use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};
fn main() {
    let greeting_file = match File::open("hello.txt") {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", error),
        },
    };

    let gg = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("There was a problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    let greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");

    println!("Contents of the file: {:?} {:?}", greeting_file, gg);
}

// error propagation;
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("username.txt");
    let mut file = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut name = String::new();
    match file.read_to_string(&mut name) {
        Ok(_) => Ok(name),
        Err(e) => return Err(e),
    }
}
