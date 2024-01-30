

#[derive(Debug, Default, Clone)]
pub struct Span<'a> {
    input: &'a str,
    start: Position,
    _end: Position,
}

impl<'a> Span<'a> {
    pub fn start(&self) -> &Position {
        &self.start
    }

    pub fn _end(&self) -> &Position {
        &self._end
    }

    pub fn input(&self) -> &str {
        self.input
    }
}

#[derive(Debug, Default, Clone)]
pub struct Position {
    line: usize,
    col: usize,
}

impl Position {
    pub fn line(&self) -> usize {
        self.line
    }

    pub fn col(&self) -> usize {
        self.col
    }
}

impl<'a> From<pest::Span<'a>> for Span<'a> {
    fn from(value: pest::Span<'a>) -> Self {
        let start = value.start_pos().into();
        let end = value.end_pos().into();

        Self {
            input: value.as_str(),
            start,
            _end: end,
        }
    }
}

impl From<pest::Position<'_>> for Position {
    fn from(value: pest::Position) -> Self {
        let (line, col) = value.line_col();
        Self { line, col }
    }
}

