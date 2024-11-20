use crate::spanned::Spanned;

use super::{atom::Atom, symbol::Symbol};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WidgetCall {
    pub name: Spanned<Symbol>,
    pub args: Vec<WidgetCallArg>,
    pub children: Vec<WidgetCallChild>,
}

impl WidgetCall {
    pub fn new(
        name: impl Into<Spanned<Symbol>>,
        args: Vec<impl Into<WidgetCallArg>>,
        children: Vec<impl Into<WidgetCallChild>>,
    ) -> Self {
        Self {
            name: name.into(),
            args: args.into_iter().map(|a| a.into()).collect(),
            children: children.into_iter().map(|c| c.into()).collect(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WidgetCallArg {
    pub name: Spanned<Symbol>,
    pub value: WidgetCallArgValue,
}

impl WidgetCallArg {
    pub fn new(name: impl Into<Spanned<Symbol>>, value: impl Into<WidgetCallArgValue>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum WidgetCallArgValue {
    Atom(Spanned<Atom>),
}

impl From<Spanned<Atom>> for WidgetCallArgValue {
    fn from(value: Spanned<Atom>) -> Self {
        Self::Atom(value)
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum WidgetCallChild {
    Atom(Spanned<Atom>),
    WidgetCall(Spanned<WidgetCall>),
}

impl From<Spanned<Atom>> for WidgetCallChild {
    fn from(value: Spanned<Atom>) -> Self {
        Self::Atom(value)
    }
}

impl From<Spanned<WidgetCall>> for WidgetCallChild {
    fn from(value: Spanned<WidgetCall>) -> Self {
        Self::WidgetCall(value)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::{
        ast::ParseError,
        lexer::{Lexer, LexicalError},
    };

    #[test]
    fn no_args() {
        let (errs, ast) = test("(container)");

        assert_eq!(errs, Vec::new());
        assert_eq!(
            ast,
            Ok(WidgetCall::new(
                (1, "container".into(), 10),
                Vec::<WidgetCallArg>::new(),
                Vec::<WidgetCallChild>::new()
            ))
        );
    }
    #[test]
    fn args() {
        let (errs, ast) = test(r#"(labeled-container :x 0 :name "foo")"#);

        assert_eq!(errs, Vec::new());
        assert_eq!(
            ast,
            Ok(WidgetCall::new(
                (1, "labeled-container".into(), 18),
                vec![
                    WidgetCallArg::new(
                        (20, "x".into(), 21),
                        Spanned::from((22, Atom::new_number("0"), 23))
                    ),
                    WidgetCallArg::new(
                        (25, "name".into(), 29),
                        Spanned::from((30, Atom::from("foo"), 35))
                    )
                ],
                Vec::<WidgetCallChild>::new(),
            ))
        );
    }

    #[test]
    fn children_atom() {
        let (errs, ast) = test(r#"(labeled-container "content")"#);

        assert_eq!(errs, Vec::new());
        assert_eq!(
            ast,
            Ok(WidgetCall::new(
                (1, "labeled-container".into(), 18),
                Vec::<WidgetCallArg>::new(),
                vec![Spanned::from((19, Atom::from("content"), 28))]
            ))
        );
    }

    #[test]
    fn children_widget() {
        let (errs, ast) = test(r#"(labeled-container (button))"#);

        assert_eq!(errs, Vec::new());
        assert_eq!(
            ast,
            Ok(WidgetCall::new(
                (1, "labeled-container".into(), 18),
                Vec::<WidgetCallArg>::new(),
                vec![Spanned::from((
                    19,
                    WidgetCall::new(
                        (20, "button".into(), 26),
                        Vec::<WidgetCallArg>::new(),
                        Vec::<WidgetCallChild>::new(),
                    ),
                    27
                ))]
            ))
        );
    }

    #[test]
    fn args_and_children() {
        let (errs, ast) = test(
            r#"
            (labeled-container :name "foo"
                (button :onclick "notify-send hey ho"
                    "click me"))"#,
        );

        assert_eq!(errs, Vec::new());
        assert_eq!(
            ast,
            Ok(WidgetCall::new(
                (14, "labeled-container".into(), 31),
                vec![WidgetCallArg::new(
                    (33, "name".into(), 37),
                    Spanned::from((38, Atom::from("foo"), 43))
                )],
                vec![Spanned::from((
                    60,
                    WidgetCall::new(
                        (61, "button".into(), 67),
                        vec![WidgetCallArg::new(
                            (69, "onclick".into(), 76),
                            Spanned::from((77, Atom::from("notify-send hey ho"), 97))
                        )],
                        vec![Spanned::from((118, Atom::from("click me"), 128))],
                    ),
                    129
                ))],
            ))
        );
    }

    // TODO: Missing closing parenthesis
    // TODO: Missing opening parenthesis?
    // TODO: Malformed arguments
    //  - missing `:`
    //  - missing name
    //  - missing value

    fn test(inp: &str) -> (Vec<ParseError>, Result<WidgetCall, LexicalError>) {
        let _ = env_logger::builder().is_test(true).try_init();

        let lexer = Lexer::new(inp);
        let parser = crate::grammar::WidgetCallParser::new();

        let mut errors = Vec::new();

        let result = parser.parse(&mut errors, lexer).map_err(|e| match e {
            lalrpop_util::ParseError::User { error } => error,
            e => unimplemented!("this error should not happen: {:?}", e),
        });

        (errors, result.map(|r| r.1))
    }
}
