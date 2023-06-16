use std::error::Error;
use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {
    let file_name = String::from("hello.txt");
    match read_short_impl(file_name) {
        Ok(s) => {println!("{s}");},
        Err(e) => {panic!{"Encountered error in reading file: {e}"};}
    }
}

fn reading_files() {
    let file_name = "hello.txt";

    let read_result = File::open(file_name).unwrap_or_else(|read_error| {
        if read_error.kind() == ErrorKind::NotFound {
            println!("File not found: {file_name}");
            println!("Therefore, creating new file: {file_name}");
            return File::create(file_name).unwrap_or_else(|create_error| {
                panic!("Could not create file: {file_name}");
                panic!("Due to error: {create_error}");            
            });
        } else {
            panic!("Error in reading file: {read_error}");
        }
    });
}

fn read_data_from_file(file_name: String) -> Result<String, io::Error> {
    let open_file_result = File::open(file_name);

    match open_file_result {
        Ok(mut f) => {
            let mut data = String::new();
            match f.read_to_string(&mut data) {
                Ok(_) => return Ok(data),
                Err(r) => Err(r)
            }

        },
        Err(e) => return Err(e)
    }
}

fn read_short_impl(file_name: String) -> Result<String, Box <dyn Error>> {

    let mut data = String::new();
    File::open(file_name)?.read_to_string(&mut data)?;
    Ok(data)
}
