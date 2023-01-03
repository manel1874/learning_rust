use std::fs::File;

pub fn open_file() {
    let greeting_file_result = File::open("hello.txt");

    /*
    greeting_file_result can be one of two instances:
    
    1. An instance of 'Ok' that contains a file handle
    2. An instance of 'Err' that contains more information about
    the kind of error that happened.

    Now, we need to add to the code above to take different actions
    depending on the value File::oepn returns.
    */

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}

use std::io::ErrorKind;

pub fn open_and_match_diff() {
    let greeting_file_result = File::open("hello-world.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello-world.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}

pub fn open_with_unwrap() {
    let greeting_file = File::open("hello-world-unwrap.txt").unwrap();
}

pub fn open_with_expect() {
    let greeting_file = File::open("hello-world-expect.txt")
        .expect("hello-world-expect.txt should be included in this project first.");
}