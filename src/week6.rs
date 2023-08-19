use std::panic;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::fs::{self, File};
use std::io::{self, Read};
use std::mem::size_of_val;
use std::slice;

fn fn_panic() {
    let v = vec![10, 20, 30];
    println!("v[100]: {}", v[100]);
}

fn stack_backward() {
    let result = panic::catch_unwind(|| {
        println!("hello!");
    });
    //println!("{:?}", result);
    assert!(result.is_ok());
    
    let result = panic::catch_unwind(|| {
        panic!("oh no!");
    });
    assert!(result.is_err());
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
/// assert_eq!(shorten_string("Hello World", 5), "Hello");
/// assert_eq!(shorten_string("Hello World", 20), "Hello World");
/// ```
pub fn shorten_string(s: &str, length: usize) -> &str {
    &s[..std::cmp::min(length, s.len())]
}

fn raw_pointer_reverse_reference() {
    let mut num = 5;

    let r1 = &mut num as *mut i32;
    let r2 = r1 as *const i32;

    // Safe because r1 and r2 were obtained from references and so are
    // guaranteed to be non-null and properly aligned, the objects underlying
    // the references from which they were obtained are live throughout the
    // whole unsafe block, and they are not accessed either through the
    // references or concurrently through any other pointers.
    unsafe {
        println!("r1 is: {}", *r1);
        *r1 = 10;
        println!("r2 is: {}", *r2);
    }
}

static HELLO_WORLD: &str = "Hello, world!";
fn static_variable() {
    println!("HELLO_WORLD: {HELLO_WORLD}");
}

static mut COUNTER: u32 = 0;
fn add_to_counter(inc: u32) {
    unsafe { COUNTER += inc; }  // 잠재적 데이터 경합!
}

fn use_add_to_counter() {
    add_to_counter(42);

    unsafe { println!("COUNTER: {COUNTER}"); }  // 잠재적 데이터 경합!
}

#[repr(C)]
union MyUnion {
    i: u8,
    b: bool,
}

fn fn_union() {
    let u = MyUnion { i: 42 };
    println!("int: {}", unsafe { u.i });
    println!("bool: {}", unsafe { u.b });  // Undefined behavior!
}

fn count_chars(s: &str) -> usize {
    s.chars().map(|_| 1).sum()
}

fn call_insecure_function() {
    let emojis = "🗻∈🌏";

    // Safe because the indices are in the correct order, within the bounds of
    // the string slice, and lie on UTF-8 sequence boundaries.
    unsafe {
        println!("emoji: {}", emojis.get_unchecked(0..4));
        println!("emoji: {}", emojis.get_unchecked(4..7));
        println!("emoji: {}", emojis.get_unchecked(7..11));
    }

    println!("char count: {}", count_chars(unsafe { emojis.get_unchecked(0..7) }));

    // Not upholding the UTF-8 encoding requirement breaks memory safety!
    // println!("emoji: {}", unsafe { emojis.get_unchecked(0..3) });
    // println!("char count: {}", count_chars(unsafe { emojis.get_unchecked(0..3) }));
}

unsafe fn swap(a: *mut u8, b: *mut u8) {
    let temp = *a;
    *a = *b;
    *b = temp;
}

fn to_create_an_unsafe_function() {
    let mut a = 42;
    let mut b = 66;

    // Safe because ...
    unsafe {
        swap(&mut a, &mut b);
    }

    println!("a = {}, b = {}", a, b);
}

extern "C" {
    fn abs(input: i32) -> i32;
}

fn call_external_code() {
    unsafe {
        // Undefined behavior if abs misbehaves.
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

pub unsafe trait AsBytes {
    fn as_bytes(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self as *const Self as *const u8, size_of_val(self))
        }
    }
}

// Safe because u32 has a defined representation and no padding.
unsafe impl AsBytes for u32 {}

pub fn week6() {
    //fn_panic();
    stack_backward();
    structured_error_handling_using_results();
    error_propagation_using_question_mark();
    if_the_error_type_is_different();
    //dynamic_error_type();
    raw_pointer_reverse_reference();
    static_variable();
    use_add_to_counter();
    fn_union();
    call_insecure_function();
    to_create_an_unsafe_function();
    call_external_code();
}