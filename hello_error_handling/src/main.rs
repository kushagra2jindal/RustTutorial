use std::fs::File;

fn main() {

    // panic
    // panic!("Explicit panic");

    // let v = [1,2,3];
    // v[10];

    // Result Error Handling
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    
}
