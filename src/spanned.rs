use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Clone)]
#[derivative(Debug)]
#[derivative(PartialEq)]
#[derivative(PartialOrd)]
pub struct Spanned<T>(pub usize, pub T, pub usize);

impl<T> Spanned<T> {
    pub fn map<R>(self, map_fn: impl FnOnce(T) -> R) -> Spanned<R> {
        (self.0, map_fn(self.1), self.2).into()
    }
}

impl<T> From<(usize, T, usize)> for Spanned<T> {
    fn from(value: (usize, T, usize)) -> Self {
        Self(value.0, value.1, value.2)
    }
}

impl From<(usize, usize)> for Spanned<()> {
    fn from(value: (usize, usize)) -> Self {
        Self(value.0, (), value.1)
    }
}
