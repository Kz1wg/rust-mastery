pub fn count_words(content: &str) -> usize {
    content.split_whitespace().count()
}

pub fn run(path: &str) -> Result<String, std::io::Error> {
    let content = std::fs::read_to_string(path)?;
    Ok(format!("{} words", count_words(&content)))
}
