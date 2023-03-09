/* 
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");
}

 */

/* 
/// Indiscriminate use of `panic!`
use std::fs::File;

fn main() {
     let greeting_file_result = File::open("hello.txt");
 
     let greeting_file = match greeting_file_result {
         Ok(file) => file,
         Err(error) => panic!("Problem opening the file: {:?}", error),
     };
 }
*/

/*

Alternative with closures.

use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
 */

use std::fs::{self, File, OpenOptions};
use std::io::{self, ErrorKind, Read, Seek, Write};

/// Below is based on the example from the first part of chapter 9.2.
fn main1() {
    let greeting_file_result = OpenOptions::new()
        .write(true)
        .create(false)
        .truncate(false)
        .open("hello.txt");

    let mut greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    match greeting_file.seek(std::io::SeekFrom::End(0)) {
        Result::Ok(_) => {}
        Result::Err(err) => panic!("Problem seeking to end of file: {:?}", err),
    }

    let text = String::from("hello, this is hello.txt!");
    match greeting_file.write_all(text.as_bytes()) {
        Ok(_) => {},
        Err(err) => panic!("Problem writing to file: {:?}", err),
    };
}

///
///
///

/* 
fn main() {
    let greeting_file = File::open("hello.txt").unwrap();
}
 */

/* 
/// Using `expect` instead of `unwrap`.
fn main() {
     let greeting_file = File::open("hello.txt")
         .expect("hello.txt should be included in this project");
}
 */

fn read_username_from_file1() -> Result<String, io::Error> {
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

/// Shortcut to the above using `?`
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

/// Even shorter
fn read_username_from_file3() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

/// Idiomatic
fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

///
///
///

/*
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn main() {
    assert_eq!(
        last_char_of_first_line("Hello, world\nHow are you today?"),
        Some('d')
    );

    assert_eq!(last_char_of_first_line(""), None);
    assert_eq!(last_char_of_first_line("\nhi"), None);
    assert_eq!(last_char_of_first_line("acc\n"), Some('c'));
}
*/

use std::error::Error;

/// How to use `?` in `main` -Return type of `Result` - with a `Box` inside - required.
fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;
    Ok(())
}