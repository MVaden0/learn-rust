fn main() {
    let my_string = String::from("Hello, world!");

    let f_word = first_word(&my_string);

    println!("first word: {}", f_word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

