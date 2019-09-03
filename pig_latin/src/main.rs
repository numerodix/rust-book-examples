fn is_vowel(ch: &char) -> bool {
    match "aeiouy".find(*ch) {
        None => false,
        _ => true,
    }
}

fn to_pig_latin(text: &str) -> String {
    let words_ref: Vec<&str> = text.split_whitespace().collect();
    let mut words: Vec<String> = vec![];

    for word_ref in &words_ref {
        let mut word = word_ref.to_string();
        
        let first_char = word.chars().next().unwrap();
        match is_vowel(&first_char) {
            true => {
                word.push_str("-hay");
            },
            false => {
                let suffix = format!("-{}ay", first_char);
                word.remove(0);
                word.push_str(&suffix);
            },
        }

        words.push(word);
    }

    // println!("{:?}", words);
    let result = words.join(" ");

    result
}

fn main() {
    // let story = "first apple";
    let story = "the quick brown fox jumps over the lazy dog";
    println!("Story       : {}", story);

    let pig_latin = to_pig_latin(story);
    println!("In pig latin: {}", pig_latin);
}