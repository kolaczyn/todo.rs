pub fn random_id() -> String {
    let word_one = random_word::gen();
    let word_two = random_word::gen();
    format!("{} {}", word_one, word_two)
    
}