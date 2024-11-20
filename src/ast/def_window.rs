use crate::spanned::Spanned;

use super::{atom::Atom, symbol::Symbol, widget_call::WidgetCall};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DefWindow {
    pub keyword: Spanned<()>,
    pub name: Spanned<Symbol>,
    pub args: Vec<DefWindowArg>,
    pub content: Vec<DefWindowContent>,
}

impl DefWindow {
    pub fn new(
        keyword: impl Into<Spanned<()>>,
        name: impl Into<Spanned<Symbol>>,
        args: Vec<impl Into<DefWindowArg>>,
        content: Vec<impl Into<DefWindowContent>>,
    ) -> Self {
        Self {
            keyword: keyword.into(),
            name: name.into(),
            args: args.into_iter().map(|a| a.into()).collect(),
            content: content.into_iter().map(|c| c.into()).collect(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DefWindowArg {
    pub name: Spanned<Symbol>,
    pub value: DefWindowArgValue,
}

impl DefWindowArg {
    pub fn new(name: impl Into<Spanned<Symbol>>, value: impl Into<DefWindowArgValue>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum DefWindowArgValue {
    Atom(Spanned<Atom>),
    WidgetCall(Spanned<WidgetCall>),
}

impl From<Spanned<Atom>> for DefWindowArgValue {
    fn from(value: Spanned<Atom>) -> Self {
        Self::Atom(value)
    }
}

impl From<Spanned<WidgetCall>> for DefWindowArgValue {
    fn from(value: Spanned<WidgetCall>) -> Self {
        Self::WidgetCall(value)
    }
}

// TODO: Rename to `DefWindowChild` for consistency
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum DefWindowContent {
    Atom(Spanned<Atom>),
    WidgetCall(Spanned<WidgetCall>),
}

impl From<Spanned<Atom>> for DefWindowContent {
    fn from(value: Spanned<Atom>) -> Self {
        Self::Atom(value)
    }
}

impl From<Spanned<WidgetCall>> for DefWindowContent {
    fn from(value: Spanned<WidgetCall>) -> Self {
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
    use crate::spanned::Spanned;

    #[test]
    fn missing_name() {
        let (errs, ast) = test(r#"(defwindow :type "dock")"#);

        assert_eq!(ast, Ok((0, TopLevelExpr::Err, 24).into()));
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
            Ok((
                0,
                DefWindow::new(
                    (1, 10),
                    (11, "bar".into(), 14),
                    Vec::<DefWindowArg>::new(),
                    Vec::<DefWindowContent>::new()
                )
                .into(),
                15
            )
                .into())
        )
    }

    #[test]
    fn atom_args() {
        let (errs, ast) = test(
            r#"(defwindow top
                :monitor 0
                :windowtype "dock")"#,
        );

        assert_eq!(errs, Vec::new());
        assert_eq!(
            ast,
            Ok((
                0,
                DefWindow::new(
                    (1, 10),
                    (11, "top".into(), 14),
                    vec![
                        DefWindowArg::new(
                            (32, "monitor".into(), 39),
                            Spanned::from((40, Atom::new_number("0"), 41))
                        ),
                        DefWindowArg::new(
                            (59, "windowtype".into(), 69),
                            Spanned::from((70, Atom::from("dock"), 76))
                        ),
                    ],
                    Vec::<DefWindowContent>::new()
                )
                .into(),
                77
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
            r#"(defwindow bar
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
            Ok((
                0,
                DefWindow::new(
                    (1, 10),
                    (11, "bar".into(), 14),
                    vec![DefWindowArg::new(
                        (32, "geometry".into(), 40),
                        Spanned::from((
                            41,
                            WidgetCall::new(
                                (42, "geometry".into(), 50),
                                vec![
                                    WidgetCallArg::new(
                                        (72, "x".into(), 73),
                                        Spanned::from((74, Atom::from("0%"), 78))
                                    ),
                                    WidgetCallArg::new(
                                        (100, "y".into(), 101),
                                        Spanned::from((102, Atom::from("0%"), 106))
                                    ),
                                    WidgetCallArg::new(
                                        (128, "width".into(), 133),
                                        Spanned::from((134, Atom::from("90%"), 139))
                                    ),
                                    WidgetCallArg::new(
                                        (161, "height".into(), 167),
                                        Spanned::from((168, Atom::from("10px"), 174))
                                    ),
                                    WidgetCallArg::new(
                                        (196, "anchor".into(), 202),
                                        Spanned::from((203, Atom::from("top center"), 215))
                                    ),
                                ],
                                Vec::<WidgetCallChild>::new()
                            ),
                            216
                        ))
                    )],
                    Vec::<DefWindowContent>::new()
                )
                .into(),
                217
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
            Ok((
                0,
                DefWindow::new(
                    (1, 10),
                    (11, "frame".into(), 16),
                    Vec::<DefWindowArg>::new(),
                    vec![Spanned::from((17, Atom::from("canvas"), 25))]
                )
                .into(),
                26
            )
                .into())
        );
    }

    #[test]
    fn widget_call_content() {
        let (errs, ast) = test(
            r#"(defwindow bar
                :geometry (geometry :anchor "top center")
                (bar))"#,
        );

        assert_eq!(errs, Vec::new());
        assert_eq!(
            ast,
            Ok((
                0,
                DefWindow::new(
                    (1, 10),
                    (11, "bar".into(), 14),
                    vec![DefWindowArg::new(
                        (32, "geometry".into(), 40),
                        Spanned::from((
                            41,
                            WidgetCall::new(
                                (42, "geometry".into(), 50),
                                vec![WidgetCallArg::new(
                                    (52, "anchor".into(), 58),
                                    Spanned::from((59, Atom::from("top center"), 71))
                                )],
                                Vec::<WidgetCallChild>::new(),
                            ),
                            72
                        ))
                    )],
                    vec![Spanned::from((
                        89,
                        WidgetCall::new(
                            (90, "bar".into(), 93),
                            Vec::<WidgetCallArg>::new(),
                            Vec::<WidgetCallChild>::new()
                        ),
                        94
                    ))],
                )
                .into(),
                95
            )
                .into())
        );
    }

    fn test(inp: &str) -> (Vec<ParseError>, Result<Spanned<TopLevelExpr>, LexicalError>) {
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
