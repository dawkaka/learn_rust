fn main() {
    let mut s = String::from("Hell world and shit");
    let f = first_word_slice(&s);
    println!("{}", f);
    s.clear();
}

// fn first_word(s: &str) -> usize {
//     let bytes: &[u8] = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//     s.len()
// }

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    return &s[..];
}
