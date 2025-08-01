use std::fmt;

#[derive(Debug, Clone)]
pub struct Paragraph {
    pub content: String,
}

impl fmt::Display for Paragraph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.content)
    }
}
