use std::panic;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::fs::{self, File};
use std::io::{self, Read};

fn fn_panic() {
    let v = vec![10, 20, 30];
    //println!("v[100]: {}", v[100]);
}

fn stack_backward() {
    let result = panic::catch_unwind(|| {
        //println!("hello!");
    });
    //assert!(result.is_ok());
    
    let result = panic::catch_unwind(|| {
        //panic!("oh no!");
    });
    //assert!(result.is_err());
}

fn structured_error_handling_using_results() {
    let file = fs::File::open("diary.txt");
    match file {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents);
            println!("Dear diary: {contents}");
        },
        Err(err) => {
            println!("The diary could not be opened: {err}");
        }
    }
}

fn read_username(path: &str) -> Result<String, io::Error> {
    let username_file_result = fs::File::open(path);
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(err) => return Err(err),
    };

    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(err) => Err(err),
    }
}

fn error_propagation_using_question_mark() {
    //fs::write("config.dat", "alice").unwrap();
    let username = read_username("config.dat");
    println!("username or error: {username:?}");
}

#[derive(Debug)]
enum ReadUsernameError {
    IoError(io::Error),
    EmptyUsername(String),
}

impl Error for ReadUsernameError {}

impl Display for ReadUsernameError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::IoError(e) => write!(f, "IO error: {e}"),
            Self::EmptyUsername(filename) => write!(f, "Found no username in {filename}"),
        }
    }
}

impl From<io::Error> for ReadUsernameError {
    fn from(err: io::Error) -> ReadUsernameError {
        ReadUsernameError::IoError(err)
    }
}

fn read_username2(path: &str) -> Result<String, ReadUsernameError> {
    let mut username = String::with_capacity(100);
    File::open(path)?.read_to_string(&mut username)?;
    if username.is_empty() {
        return Err(ReadUsernameError::EmptyUsername(String::from(path)));
    }
    Ok(username)
}

fn if_the_error_type_is_different() {
    //fs::write("config.dat", "").unwrap();
    let username = read_username2("config.dat");
    println!("username or error: {username:?}");
}

// #[derive(Debug, thiserror::Error)]
// enum ReadUsernameError2 {
//     #[error("Could not read: {0}")]
//     IoError(#[from] io::Error),
//     #[error("Found no username in {0}")]
//     EmptyUsername(String),
// }

// fn read_username3(path: &str) -> Result<String, ReadUsernameError2> {
//     let mut username = String::new();
//     fs::File::open(path)?.read_to_string(&mut username)?;
//     if username.is_empty() {
//         return Err(ReadUsernameError2::EmptyUsername(String::from(path)));
//     }
//     Ok(username)
// }

// fn another_error_enumeration() {
//     //fs::write("config.dat", "").unwrap();
//     match read_username3("config.dat") {
//         Ok(username) => println!("Username: {username}"),
//         Err(err)     => println!("Error: {err}"),
//     }
// }

// use std::fs;
// use std::io::Read;
// use thiserror::Error;
// use std::error::Error;

// #[derive(Clone, Debug, Eq, Error, PartialEq)]
// #[error("Found no username in {0}")]
// struct EmptyUsernameError(String);

// fn read_username4(path: &str) -> Result<String, Box<dyn Error>> {
//     let mut username = String::new();
//     fs::File::open(path)?.read_to_string(&mut username)?;
//     if username.is_empty() {
//         return Err(EmptyUsernameError(String::from(path)).into());
//     }
//     Ok(username)
// }

// fn dynamic_error_type() {
//     //fs::write("config.dat", "").unwrap();
//     match read_username4("config.dat") {
//         Ok(username) => println!("Username: {username}"),
//         Err(err)     => println!("Error: {err}"),
//     }
// }

fn first_word(text: &str) -> &str {
    match text.find(' ') {
        Some(idx) => &text[..idx],
        None => &text,
    }
}

#[test]
fn test_empty() {
    assert_eq!(first_word(""), "");
}

#[test]
fn test_single_word() {
    assert_eq!(first_word("Hello"), "Hello");
}

#[test]
fn test_multiple_words() {
    assert_eq!(first_word("Hello World"), "Hello");
}

fn helper(a: &str, b: &str) -> String {
    format!("{a} {b}")
}

pub fn main() {
    println!("{}", helper("Hello", "World"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_helper() {
        assert_eq!(helper("foo", "bar"), "foo bar");
    }
}

/// Shortens a string to the given length.
///
/// ```
/// pub mod week6;
/// use week6::shorten_string;
/// assert_eq!(shorten_string("Hello World", 5), "Hello");
/// assert_eq!(shorten_string("Hello World", 20), "Hello World");
/// ```
pub fn shorten_string(s: &str, length: usize) -> &str {
    &s[..std::cmp::min(length, s.len())]
}

pub fn week6() {
    fn_panic();
    stack_backward();
    structured_error_handling_using_results();
    error_propagation_using_question_mark();
    if_the_error_type_is_different();
    //dynamic_error_type();
}