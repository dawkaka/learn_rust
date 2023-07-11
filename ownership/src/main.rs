fn main() {
    let mut str = "Hello world";
    str = "Hello there!";
    println!("New string: {}", str);

    let s1 = String::from("Hello!");
    let s2 = s1;
    println!("{}", s2); // s1 gives error because it's been moved to avoid double `drop` calls

    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        //println!("{}", s); // ... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,

    let (ss, sl) = calculate_length(s2);
    println!("{}:{}", ss, sl);
    let mut sMut = String::from("Original");
    modify_string_must(&mut sMut);
    println!("{}", sMut); // We still have acces to `sMut` here because it hasn't been moved
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
// This calculate_length function won't work
// fn calculate_length(s: String) -> (String, usize) {
//     (s, s.len())
// }
// return `s` means moving `s` and calling `s.len()` after moving it gives an error
// calling `s.len()` before moving `s` works though eg: (s.len(), s)

// We'll still have access to s after the closing curly brace of the function
// Because it's a reference it won't be `dropped` by the compiler
fn calculate_lengthR(s: &String) -> usize {
    s.len()
}

// we can't also modify borrowed values
// this won't work
// fn modify_string(s: &String) -> usize {
//     s.push_str(" Changed!"); // Error: `s` is a `&` reference, so the data it refers to cannot be borrowed as mutable
//     s.len()
// }

// To be able to do that we need to use mutable references
fn modify_string_must(s: &mut String) {
    s.push_str(" Changed!")
}
