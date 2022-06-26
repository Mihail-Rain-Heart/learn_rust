fn main() {
    let s = "Mishka:-)";
    println!("{}", get_word(s))
}

fn get_word(str: &str) -> &str {
    let bytes = str.as_bytes();
    for (i, &char) in bytes.iter().enumerate() {
        if char == b' ' {
            return &str[..i];
        }
    }

    &str[..]
}
