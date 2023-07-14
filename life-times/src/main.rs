use std::fmt::Display;

fn main() {
    // let r;

    // {
    //     let x = 5;
    //     r = &x;
    // }

    // println!("r: {}", r);

    let string1 = "abcd";
    let string2 = "xyz";

    let result = longest(string1, string2, 4);
    println!("The longest string is {}", result);

    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let res = longest(string1.as_str(), string2.as_str(), "hhee");
        println!("The longest string is {}", res);
    }
}

fn longest<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcements {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
