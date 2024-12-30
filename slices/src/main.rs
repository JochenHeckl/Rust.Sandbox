fn main() {
    let source = String::from("Hello, world!");
    // let first_word_length = first_word(&source);
    // let first_word = &source[0..first_word_length];
    let first_word = first_word(&source);

    println!("The first world in {source} is {first_word}.");

    // let broken_slice = &source[..107];
    // println!("Does this crash the program? {broken_slice}.");

    // -> Yes it does. But the crash happens at &source[..107]!
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    s
}
