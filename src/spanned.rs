use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Debug)]
#[derivative(PartialEq)]
#[derivative(PartialOrd)]
pub struct Spanned<T>(pub usize, pub T, pub usize);

impl<T> From<(usize, T, usize)> for Spanned<T> {
    fn from(value: (usize, T, usize)) -> Self {
        Self(value.0, value.1, value.2)
    }
}
