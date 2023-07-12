fn main() {
    let mut v: Vec<String> = Vec::new();
    v.push(String::from("hello"));
    v.push(String::from("nice"));
    v.push(String::from("world"));
    println!("{}", v.len()); // Prints 3
    let third = &mut v[2];
    third.push('n');
    println!("The third element is {}", third); // The third element is worldn

    let fourth = v.get(4);
    match fourth {
        Some(f) => println!("The fourth element is {}", f),
        None => println!("There is no fourth element!"),
    }

    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    v.push(6);
    // println!("The first element is: {first}"); // uncommenting this gives error (borrowing mutable and immutable reference)

    let vl = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    for i in &vl {
        println!("{i}");
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(1),
        SpreadsheetCell::Float(2.4),
        SpreadsheetCell::Text(String::from("Hello")),
    ];

    let mut s1 = String::from("foo");
    let mut s2 = String::from("bar");
    s1.push_str(&s2);
    s2.push_str(" hh");
    println!("s2 is {s2}");
    let s1 = String::from("Hello, ");
    let mut s2 = String::from("world!");
    let s3 = s1 + &s2;
    s2.push('2');
    println!("{s3}");
    let hello = String::from("Здравствуйте");
    println!("{}", hello.len());
    let s = &hello[0..4];
    println!("{}", s);
    for v in hello.chars() {
        print!("{v}")
    }
    println!();
    let hindi = String::from("नमस्ते");
    for h in hindi.chars() {
        print!("{h}");
    }
}
