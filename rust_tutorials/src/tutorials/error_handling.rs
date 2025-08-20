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
    let _file = file_result.unwrap();
    // let _file = file_result.expect("Failed to open file");
}

// ---------------------------------------------------------------------- //

pub fn error_propagation_demo() {
    let file_result = File::open("random.rdm");

    // explicitly propagate errors
    match explicit_error_thrower(&file_result) {
        Ok(s) => println!("Explicit propagation: {s}"),
        Err(e) => println!("Explicit propagation error: {:?}", e),
    };

    // implicitly propagate errors
    match implicit_error_thrower(file_result) {
        Ok(s) => println!("Implicit propagation: {s}"),
        Err(e) => println!("Implicit propagation error: {:?}", e),
    };

    // implicitly resolve options (Compare with None and empty string to understand)
    match implicit_option_extractor(Some(String::from("Apple"))) {
        Some(s) => println!("Implicit resolution: {s}"),
        None => println!("Implicit resolution: None")
    };
}

fn explicit_error_thrower(file_result: &Result<File, Error>) -> Result<String, Error> {
    // explicitly match the error and propagate an error with custom message
    match file_result {
        Ok(_f) => Ok(String::from("Success!")),
        Err(_error) => Err(Error::new(ErrorKind::Other, "Failed!"))
    }
}

fn implicit_error_thrower(file_result: Result<File, Error>) -> Result<String, Error> {
    // we can directly place a `?` after the Result to propagate the error from there if any
    let _file_size = file_result?.metadata()?.len();
    Ok(String::from("Success"))
}

fn implicit_option_extractor(msg: Option<String>) -> Option<bool> {
    // can do multiple option extractions as long as Option is returned even with different type
    Some(msg?.chars().next()?.is_alphabetic())
}
