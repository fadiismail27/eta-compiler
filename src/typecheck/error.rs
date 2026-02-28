#[derive(Debug)]
pub struct SemanticError {
    pub line: usize,
    pub col: usize,
    pub message: String,
}

impl std::fmt::Display for SemanticError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{} error:{}", self.line, self.col, self.message)
    }
}
