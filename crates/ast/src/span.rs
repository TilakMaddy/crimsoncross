use std::ops::Range;

#[derive(Debug, Clone)]
pub struct Span {
    pub char_offset: usize,
    pub char_count: usize,
}

impl From<Range<usize>> for Span {
    fn from(value: Range<usize>) -> Self {
        Self { char_offset: value.start, char_count: value.end }
    }
}
