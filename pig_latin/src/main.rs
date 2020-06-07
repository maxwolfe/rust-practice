fn main() {
    let mut vowel_test = String::from("apple");
    let mut consonant_test = String::from("first");

    pig_latin(&mut vowel_test);
    pig_latin(&mut consonant_test);

    println!("Piglatin of vowel is: {}", vowel_test);
    println!("Piglatin of consonant is: {}", consonant_test);
}

fn pig_latin(word: &mut String) {
    match &word[..1] {
        "a" | "e" | "i" | "o" | "u" => word.push_str("-hay"),
        _ => {
            word.push('-');
            let first = word.remove(0);
            word.push(first);
            word.push_str("ay");
        }
    };
}
