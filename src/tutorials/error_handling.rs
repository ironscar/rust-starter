use std::fs::File;

pub fn panic_demo() -> i32 {
    let v = vec![1, 2, 3];
    v[99]
}

pub fn recoverable_errors_demo() {
    // this file shouldn't exist to show the error
    let file_result = File::open("random.rdm");
    match file_result {
        Ok(f) => f,
        Err(error) => panic!("There was a problem opening the file: {:?}", error)
    };
}
