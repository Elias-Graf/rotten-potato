use super::{
    atom::{Atom, Symbol},
    widget_call::WidgetCall,
};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DefWindow {
    pub name: Symbol,
    pub args: Vec<DefWindowArg>,
    pub content: Vec<DefWindowContent>,
}

impl DefWindow {
    pub fn new(
        name: impl Into<Symbol>,
        args: Vec<impl Into<DefWindowArg>>,
        content: Vec<impl Into<DefWindowContent>>,
    ) -> Self {
        Self {
            name: name.into(),
            args: args.into_iter().map(|a| a.into()).collect(),
            content: content.into_iter().map(|c| c.into()).collect(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DefWindowArg {
    pub name: Symbol,
    pub value: DefWindowArgValue,
}

impl DefWindowArg {
    pub fn new(name: impl Into<Symbol>, value: impl Into<DefWindowArgValue>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum DefWindowArgValue {
    Atom(Atom),
    WidgetCall(WidgetCall),
}

impl From<Atom> for DefWindowArgValue {
    fn from(value: Atom) -> Self {
        Self::Atom(value)
    }
}

impl From<WidgetCall> for DefWindowArgValue {
    fn from(value: WidgetCall) -> Self {
        Self::WidgetCall(value)
    }
}

// TODO: Rename to `DefWindowChild` for consistency
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum DefWindowContent {
    Atom(Atom),
    WidgetCall(WidgetCall),
}

impl From<Atom> for DefWindowContent {
    fn from(value: Atom) -> Self {
        Self::Atom(value)
    }
}

impl From<WidgetCall> for DefWindowContent {
    fn from(value: WidgetCall) -> Self {
        Self::WidgetCall(value)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::ast::top_level_expr::TopLevelExpr;
    use crate::ast::widget_call::{WidgetCallArg, WidgetCallChild};
    use crate::ast::ParseError;
    use crate::lexer::{Lexer, LexicalError};

    #[test]
    fn missing_name() {
        let (errs, ast) = test(r#"(defwindow :type "dock")"#);

        assert_eq!(ast, Ok(TopLevelExpr::Err));
        assert_eq!(
            errs,
            vec![ParseError::ExpectedDefWindowName {
                err_span: (0, 24).into()
            }]
        );
    }

    #[test]
    fn no_args() {
        let (errs, ast) = test("(defwindow bar)");

        assert_eq!(errs, Vec::new());
        assert_eq!(
            ast,
            Ok(DefWindow::new(
                Symbol::new("bar"),
                Vec::<DefWindowArg>::new(),
                Vec::<DefWindowContent>::new()
            )
            .into())
        )
    }

    #[test]
    fn atom_args() {
        let (errs, ast) = test(
            r#"
            (defwindow top
                :monitor 0
                :windowtype "dock")"#,
        );

        assert_eq!(errs, Vec::new());
        assert_eq!(
            ast,
            Ok(DefWindow::new(
                "top",
                vec![
                    DefWindowArg::new("monitor", Atom::new_number("0")),
                    DefWindowArg::new("windowtype", Atom::from("dock")),
                ],
                Vec::<DefWindowContent>::new()
            )
            .into())
        );
    }

    #[test]
    fn widget_args() {
        // TODO: this leads to broken parser:
        // r#"
        //      (defwindow bar
        //          :geometry (geometry)
        // "#,

        let (errs, ast) = test(
            r#"
            (defwindow bar
                :geometry (geometry
                    :x "0%"
                    :y "0%"
                    :width "90%"
                    :height "10px"
                    :anchor "top center"))"#,
        );

        assert_eq!(errs, Vec::new());
        assert_eq!(
            ast,
            Ok(DefWindow::new(
                "bar",
                vec![DefWindowArg::new(
                    "geometry",
                    WidgetCall::new(
                        "geometry",
                        vec![
                            WidgetCallArg::new("x", Atom::from("0%")),
                            WidgetCallArg::new("y", Atom::from("0%")),
                            WidgetCallArg::new("width", Atom::from("90%")),
                            WidgetCallArg::new("height", Atom::from("10px")),
                            WidgetCallArg::new("anchor", Atom::from("top center")),
                        ],
                        Vec::<WidgetCallChild>::new()
                    )
                )],
                Vec::<DefWindowContent>::new()
            )
            .into())
        );
    }

    #[test]
    fn atom_content() {
        let (errs, ast) = test(r#"(defwindow frame "canvas")"#);

        assert_eq!(errs, Vec::new());
        assert_eq!(
            ast,
            Ok(DefWindow::new(
                "frame",
                Vec::<DefWindowArg>::new(),
                vec![Atom::from("canvas")]
            )
            .into())
        );
    }

    #[test]
    fn widget_call_content() {
        let (errs, ast) = test(
            r#"
            (defwindow bar
                :geometry (geometry :anchor "top center")
                (bar))"#,
        );

        assert_eq!(errs, Vec::new());
        assert_eq!(
            ast,
            Ok(DefWindow::new(
                Symbol::new("bar"),
                vec![DefWindowArg::new(
                    "geometry",
                    WidgetCall::new(
                        "geometry",
                        vec![WidgetCallArg::new("anchor", Atom::from("top center"))],
                        Vec::<WidgetCallChild>::new(),
                    )
                ),],
                vec![WidgetCall::new(
                    "bar",
                    Vec::<WidgetCallArg>::new(),
                    Vec::<WidgetCallChild>::new()
                )],
            )
            .into())
        );
    }

    fn test(inp: &str) -> (Vec<ParseError>, Result<TopLevelExpr, LexicalError>) {
        let _ = env_logger::builder().is_test(true).try_init();

        let lexer = Lexer::new(inp);
        let parser = crate::grammar::DefWindowParser::new();

        let mut errors = Vec::new();

        let result = parser.parse(&mut errors, lexer).map_err(|e| match e {
            lalrpop_util::ParseError::User { error } => error,
            e => unimplemented!("this error should not happen: {:?}", e),
        });

        (errors, result)
    }
}
