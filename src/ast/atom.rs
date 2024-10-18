// TODO: Adjust naming, e.g.: "LiteralNumber", "LiteralString", etc.
// TODO: Use &'inp str instead of String
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Atom {
    Bool(bool),
    Number(String),
    StrLit(String),
    Symbol(Symbol),
}

impl Atom {
    pub fn new_number(value: impl Into<String>) -> Self {
        Self::Number(value.into())
    }
}

impl From<&str> for Atom {
    fn from(value: &str) -> Self {
        Atom::StrLit(value.into())
    }
}

impl From<bool> for Atom {
    fn from(value: bool) -> Self {
        Atom::Bool(value)
    }
}

impl From<Symbol> for Atom {
    fn from(value: Symbol) -> Self {
        Self::Symbol(value)
    }
}

// TODO: remove
#[deprecated]
pub use super::symbol::Symbol;
