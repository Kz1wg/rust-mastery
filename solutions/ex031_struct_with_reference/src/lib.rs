pub struct Parser<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Parser { input, pos: 0 }
    }

    pub fn next_word(&mut self) -> Option<&'a str> {
        let rest = self.input[self.pos..].trim_start();
        if rest.is_empty() {
            self.pos = self.input.len();
            return None;
        }
        let start = self.input.len() - rest.len();
        let end = rest
            .find(char::is_whitespace)
            .map(|i| start + i)
            .unwrap_or(self.input.len());
        self.pos = end;
        Some(&self.input[start..end])
    }
}
