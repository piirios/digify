
#[derive(Debug)]
pub enum Error {
    Pest(pest::error::Error<Pairs<Rule>>),
    Io(std::io::Error)
}

impl<E> From<std::io::Error> for Error<E> {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

impl<E> From<pest::error::Error<E>> for Error<E> {
    fn from(err: pest::error::Error<E>) -> Self {
        Self::Pest(err)
    }
}
