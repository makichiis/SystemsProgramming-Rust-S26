// Create a program that:
// 1. Takes a string of text as input
// 2. Splits the text int owords (spaces as separator) // text.split_whitespace().collect();
// 3. Counts the frequency of each word
// 4. Returns the word with the highest frequency and its count
// Requirements:
// - Use mutable references where appropriate
// - Avoid using HashMaps or other complex data structures

fn most_frequent_word(text: &str) -> (String, usize) {
    let words: Vec<&str> = text.split_whitespace().collect();

    let mut most_freq: (String, usize) = (words[0].to_string(), 1);
    
    for i in 0..words.len() {
        let mut current_entry: (String, usize) = (words[i].to_string(), 1);
        for j in i+1..words.len() {
            if current_entry.0 == words[j] { current_entry.1 += 1 as usize; }
        }
        if current_entry.1 > most_freq.1 {
            most_freq = current_entry;
        }
    }

    most_freq
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}

