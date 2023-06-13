use std::fs::File;
use std::io::{self, Read};


fn call_with_details(){
     let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}


fn call_with_panic(){
    let greeting_file = File::open("hello.txt").unwrap();
}


fn error_propagation() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn shorter_error_propagation_using_question_mark() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn main() {
    //call_with_details();
    //call_with_panic();
    let user_name_result=error_propagation();
}