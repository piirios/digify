use crate::vtable::Name;

//
#[derive(Debug)]
pub enum Error<E> {
    Pest(pest::error::Error<E>),
    Io(std::io::Error),
    Digify(Digirror),
}

impl<E> From<std::io::Error> for Error<E> {
    #[inline]
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

impl<E> From<pest::error::Error<E>> for Error<E> {
    #[inline]
    fn from(err: pest::error::Error<E>) -> Self {
        Self::Pest(err)
    }
}

impl<E> From<Digirror> for Error<E> {
    #[inline]
    fn from(err: Digirror) -> Self {
        Self::Digify(err)
    }
}

#[derive(Debug)]
pub enum Digirror {
    NameAlreadyExist(Name),
}
