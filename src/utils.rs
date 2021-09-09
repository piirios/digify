
pub trait RemoveQuotes {
    fn remove_quotes(&mut self) -> Self;
}

impl<'s> RemoveQuotes for &'s str {
    #[inline]
    fn remove_quotes(&mut self) -> Self {
        self.trim_matches('"')
    }
}