use std::io::{self, ErrorKind, Read};
use std::fs::{self, File};

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();
    match f.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_better() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

#[allow(unused)]
fn read_username_from_file_betterer() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn main() {
    let filename = "hello.txt";
    let f = File::open(filename); // returns a Result

    let _f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(filename) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file: {:?}", e),
            },
            error => panic!("Problem opening file: {:?}", error),
        },
    };

    // This code does the same, but is more readable
    let _f = File::open(filename).unwrap_or_else(|error| {
        match error.kind() {
            ErrorKind::NotFound => File::create("hello.txt").expect("Problem creating file"),
            error => panic!("Problem opening file: {:?}", error),
        }
    });

    let username = read_username_from_file().unwrap(); // Panic on error
    println!("username is: {}", username);

    let username = read_username_from_file_better().unwrap(); // Panic on error
    println!("username is: {}", username);
}
