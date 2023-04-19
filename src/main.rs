use std::ffi::CString;
use std::fmt::{Debug};
use std::path::Path;

fn info<T: Debug + ?Sized>(a: &T) {
    println!("{:?}", a)
}

fn main() {
    let a = "?";
    let b = "?".to_string();
    info(&a);
    info(&b);

    //Advanced 1
    let input = CString::new("?").unwrap();
    info(&input);

    //Advanced 2
    let d = Path::new("/tmp/linkedin-learning");
    info(d);
}


#[test]
fn str() {
    let input = "Rust";
    info(&input);
}

#[test]
fn string() {
    let input = String::from("Rust");
    info(&input);
}

// #[test]
// fn chars() {
//     let input = 'r';
//     info(&input);
// }

// #[test]
// fn cstring() {
//     use std::ffi::{CString};
//     let input = CString::new("Rust").unwrap();
//     info(&input);
// }

// #[test]
// fn path() {
//     use std::path::Path;
//     let input = Path::new("/tmp/rust");
//     info(input);
// }
