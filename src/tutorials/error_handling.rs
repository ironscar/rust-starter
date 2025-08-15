use std::fs::File;
use std::io::{Error, ErrorKind};

pub fn default_panic_demo() -> i32 {
    let v = vec![1, 2, 3];
    v[99]
}

// ---------------------------------------------------------------------- //

pub fn recoverable_errors_demo_1() {
    // this file shouldn't exist to show the error
    let file_result = File::open("random.rdm");

    // lots of matches here, eventually we will see closures as an alternative
    match file_result {
        Ok(f) => f,
        Err(error) => match error.kind() {
            ErrorKind::PermissionDenied => panic!("Permission denied"),
            ErrorKind::NotFound => panic!("Not found!"),
            _ => panic!("Something went wrong: {:?}", error)
        }
    };
}

pub fn recoverable_errors_demo_2() {
    // this file shouldn't exist to show the error
    let file_result = File::open("random.rdm");

    // use direct unwrap to panic (comment out one of these two)
    let file = file_result.unwrap();
    // let file = file_result.expect("Failed to open file");
}

// ---------------------------------------------------------------------- //

pub fn error_propagation_demo() {
    let file_result = File::open("random.rdm");
    match explicit_error_thrower(&file_result) {
        Ok(s) => println!("{s}"),
        Err(e) => println!("Error: {:?}", e),
    };

    match implicit_error_thrower(file_result) {
        Ok(s) => println!("{s}"),
        Err(e) => println!("Error: {:?}", e),
    };
}

fn explicit_error_thrower(file_result: &Result<File, Error>) -> Result<String, Error> {
    // explicitly match the error and propagate an error with custom message
    match file_result {
        Ok(f) => Ok(String::from("Success!")),
        Err(error) => Err(Error::new(ErrorKind::Other, "Failed!"))
    }
}

fn implicit_error_thrower(file_result: Result<File, Error>) -> Result<String, Error> {
    // we can directly place a `?` after the Result to propagate the error from there if any
    let file_size = file_result?.metadata()?.len();
    Ok(String::from("Success"))
}
