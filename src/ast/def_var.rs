use super::atom::Atom;
use super::symbol::Symbol;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DefVar {
    pub name: Symbol,
    pub value: DefVarValue,
}

impl DefVar {
    pub fn new(name: impl Into<Symbol>, value: impl Into<DefVarValue>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum DefVarValue {
    Atom(Atom),
}

impl From<Atom> for DefVarValue {
    fn from(value: Atom) -> Self {
        Self::Atom(value)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{
        ast::{atom::Atom, def_var::DefVar, top_level_expr::TopLevelExpr, ParseError},
        lexer::{Lexer, LexicalError},
    };

    #[test]
    fn missing_name() {
        let (errs, ast) = test(r#"(defvar)"#);

        assert_eq!(ast, Ok(TopLevelExpr::Err));
        assert_eq!(
            errs,
            vec![ParseError::ExpectedDefVarName {
                err_span: (0, 8).into()
            }]
        );
    }

    #[test]
    fn missing_value() {
        let (errs, ast) = test(r#"(defvar foo)"#);

        assert_eq!(ast, Ok(TopLevelExpr::Err));
        assert_eq!(
            errs,
            vec![ParseError::ExpectedDefVarValue {
                err_span: (0, 12).into()
            }]
        );
    }

    #[test]
    fn atom_value() {
        let (errs, ast) = test(r#"(defvar foo "some value")"#);

        assert_eq!(errs, Vec::<ParseError>::new(),);
        assert_eq!(ast, Ok(DefVar::new("foo", Atom::from("some value")).into()));
    }

    fn test(inp: &str) -> (Vec<ParseError>, Result<TopLevelExpr, LexicalError>) {
        let _ = env_logger::builder().is_test(true).try_init();

        let lexer = Lexer::new(inp);
        let parser = crate::grammar::DefVarParser::new();

        let mut errors = Vec::new();

        let result = parser.parse(&mut errors, lexer).map_err(|e| match e {
            lalrpop_util::ParseError::User { error } => error,
            e => unimplemented!("this error should not happen: {:?}", e),
        });

        (errors, result)
    }
}
