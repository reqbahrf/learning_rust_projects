fn main() {
    let mut s = String::from("Hello world");

    let word = first_word(&s);
    
    println!("First word is {}", word);
    s.clear();
}

fn first_word(s: &str) -> &str {
    let bytes: &[u8] = s.as_bytes();

    println!("Bytes is {:?}", bytes);
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
