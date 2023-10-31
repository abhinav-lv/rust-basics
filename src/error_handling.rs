#![allow(dead_code)]
#![allow(unused_variables)]
use std::{
    fs::File,
    io::{ErrorKind, Read},
};

pub fn error_handling() {
    let file_result = File::open("bruh.txt");

    let bruh_file = match file_result {
        Ok(file) => file,
        // 1st way
        // Err(error) => panic!("Problem opening the file: {:?}", error),

        // 2nd way
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("bruh.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
}

pub fn propagate_errors() {
    let content = match read_from_file_2() {
        Ok(a) => a,
        Err(error) => panic!("Error reading from file: {:?}", error),
    };

    println!("{content}");

    // Using match
    fn read_from_file() -> Result<String, std::io::Error> {
        let content_result = File::open("src/enums.rs");

        let mut content_file = match content_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut content = String::new();
        match content_file.read_to_string(&mut content) {
            Ok(size) => Ok(format!("Read bytes: {size}\nContent: {content}")),
            Err(e) => Err(e),
        }
    }

    // Using '?' operator
    fn read_from_file_2() -> Result<String, std::io::Error> {
        // let mut file = File::open("bruh.txt")?;
        // let mut content = String::new();
        // file.read_to_string(&mut content)?;
        // Ok(content)

        /* OR */
        let mut content = String::new();
        File::open("bruh.txt")?.read_to_string(&mut content)?;
        Ok(content)
    }
}
