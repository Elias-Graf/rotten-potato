use super::atom::Atom;
use super::atom::Symbol;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WidgetCall {
    pub name: Symbol,
    pub args: Vec<WidgetCallArg>,
    pub children: Vec<WidgetCallChild>,
}

impl WidgetCall {
    pub fn new(
        name: impl Into<Symbol>,
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
    pub name: Symbol,
    pub value: WidgetCallArgValue,
}

impl WidgetCallArg {
    pub fn new(name: impl Into<Symbol>, value: impl Into<WidgetCallArgValue>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum WidgetCallArgValue {
    Atom(Atom),
}

impl From<Atom> for WidgetCallArgValue {
    fn from(value: Atom) -> Self {
        Self::Atom(value)
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum WidgetCallChild {
    Atom(Atom),
    WidgetCall(WidgetCall),
}

impl From<Atom> for WidgetCallChild {
    fn from(value: Atom) -> Self {
        Self::Atom(value)
    }
}

impl From<WidgetCall> for WidgetCallChild {
    fn from(value: WidgetCall) -> Self {
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
                "container",
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
                "labeled-container",
                vec![
                    WidgetCallArg::new("x", Atom::new_number("0")),
                    WidgetCallArg::new("name", Atom::from("foo"))
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
                "labeled-container",
                Vec::<WidgetCallArg>::new(),
                vec![Atom::from("content")]
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
                "labeled-container",
                Vec::<WidgetCallArg>::new(),
                vec![WidgetCall::new(
                    "button",
                    Vec::<WidgetCallArg>::new(),
                    Vec::<WidgetCallChild>::new(),
                )]
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
                "labeled-container",
                vec![WidgetCallArg::new("name", Atom::from("foo"))],
                vec![WidgetCall::new(
                    "button",
                    vec![WidgetCallArg::new(
                        "onclick",
                        Atom::from("notify-send hey ho")
                    )],
                    vec![Atom::from("click me")],
                )],
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

        (errors, result)
    }
}
