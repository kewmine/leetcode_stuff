#[allow(dead_code)]
pub fn merge_alternately(word1: String, word2: String) -> String {

    let max_length =
        if word1.len() < word2.len() { word2.len()}
        else if word1.len() > word2.len() {word1.len()}
        else {word1.len()};

    let mut merged_string = String::new();

    for i in 0..max_length {
        if i < word1.len() {
            merged_string.push(word1.as_bytes()[i] as char);
        }
        if i < word2.len() {
            merged_string.push(word2.as_bytes()[i] as char);
        }
    }

    merged_string
}