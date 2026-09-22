pub fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

pub struct Config {
    name: String,
}

impl Config {
    pub fn new(name: impl Into<String>) -> Self {
        Config { name: name.into() }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
