use crate::spanned::Spanned;

use super::{symbol::Symbol, widget_call::WidgetCall};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DefWidget {
    pub keyword: Spanned<()>,
    pub name: Spanned<Symbol>,
    pub params: Vec<DefWidgetParam>,
    pub children: Vec<DefWidgetChild>,
}

impl DefWidget {
    pub fn new(
        keyword: impl Into<Spanned<()>>,
        name: impl Into<Spanned<Symbol>>,
        params: Vec<impl Into<DefWidgetParam>>,
        children: Vec<impl Into<DefWidgetChild>>,
    ) -> Self {
        Self {
            keyword: keyword.into(),
            name: name.into(),
            params: params.into_iter().map(|a| a.into()).collect(),
            children: children.into_iter().map(|a| a.into()).collect(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DefWidgetParam {
    pub name: Spanned<Symbol>,
    pub is_optional: bool,
}

impl DefWidgetParam {
    pub fn new(name: impl Into<Spanned<Symbol>>, is_optional: bool) -> Self {
        Self {
            name: name.into(),
            is_optional,
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum DefWidgetChild {
    WidgetCall(Spanned<WidgetCall>),
}

impl From<Spanned<WidgetCall>> for DefWidgetChild {
    fn from(value: Spanned<WidgetCall>) -> Self {
        Self::WidgetCall(value)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{
        ast::{
            atom::Atom,
            def_widget::{DefWidget, DefWidgetChild, DefWidgetParam},
            top_level_expr::TopLevelExpr,
            widget_call::{WidgetCall, WidgetCallArg, WidgetCallChild},
            ParseError,
        },
        lexer::{Lexer, LexicalError},
        spanned::Spanned,
    };

    #[test]
    fn missing_name() {
        // TODO: Restuls in broken parser
        // r#"(defwidget [])"#
        let (errs, ast) = test(r#"(defwidget)"#);

        assert_eq!(ast, Ok((0, TopLevelExpr::Err, 11).into()));
        assert_eq!(
            errs,
            vec![ParseError::ExpectedDefWidgetName {
                err_span: (0, 11).into()
            }]
        );
    }

    #[test]
    fn missing_params() {
        let (errs, ast) = test(r#"(defwidget bar)"#);

        assert_eq!(ast, Ok((0, TopLevelExpr::Err, 15).into()));
        assert_eq!(
            errs,
            vec![ParseError::ExpectedDefWidgetParams {
                err_span: (0, 15).into()
            }]
        );
    }

    #[test]
    fn required_param() {
        let (errs, ast) = test(r#"(defwidget bar [name])"#);

        assert_eq!(errs, Vec::new());
        assert_eq!(
            ast,
            Ok((
                0,
                DefWidget::new(
                    (1, (), 10),
                    (11, "bar".into(), 14),
                    vec![DefWidgetParam::new((16, "name".into(), 20), false)],
                    Vec::<DefWidgetChild>::new()
                )
                .into(),
                22
            )
                .into())
        );
    }

    #[test]
    fn optional_param() {
        let (errs, ast) = test(r#"(defwidget bar [?name])"#);

        assert_eq!(errs, Vec::new());
        assert_eq!(
            ast,
            Ok((
                0,
                DefWidget::new(
                    (1, (), 10),
                    (11, "bar".into(), 14),
                    vec![DefWidgetParam::new((17, "name".into(), 21), true)],
                    Vec::<DefWidgetChild>::new()
                )
                .into(),
                23
            )
                .into())
        );
    }

    #[test]
    fn params() {
        let (errs, ast) = test(r#"(defwidget bar [foo ?name bar])"#);

        assert_eq!(errs, Vec::new());
        assert_eq!(
            ast,
            Ok((
                0,
                DefWidget::new(
                    (1, (), 10),
                    (11, "bar".into(), 14),
                    vec![
                        DefWidgetParam::new((16, "foo".into(), 19), false),
                        DefWidgetParam::new((21, "name".into(), 25), true),
                        DefWidgetParam::new((26, "bar".into(), 29), false),
                    ],
                    Vec::<DefWidgetChild>::new()
                )
                .into(),
                31
            )
                .into())
        );
    }

    #[test]
    fn children() {
        let (errs, ast) = test(
            r#"(defwidget bottombar [width]
                (centerbox :orientation "h"
                    (box :halign "start" :orientation "h" :space-evenly false)
                    (box :halign "end" :orientation "h" :space-evenly false)
                )
            )"#,
        );

        assert_eq!(errs, Vec::new());
        assert_eq!(
            ast,
            Ok((
                0,
                DefWidget::new(
                    (1, (), 10),
                    (11, "bottombar".into(), 20),
                    vec![DefWidgetParam::new((22, "width".into(), 27), false)],
                    vec![Spanned::from((
                        45,
                        WidgetCall::new(
                            (46, "centerbox".into(), 55),
                            vec![WidgetCallArg::new(
                                Spanned(57, "orientation".into(), 68),
                                Spanned(69, Atom::from("h"), 72)
                            )],
                            vec![
                                Spanned::from((
                                    93,
                                    WidgetCall::new(
                                        (94, "box".into(), 97),
                                        vec![
                                            WidgetCallArg::new(
                                                Spanned(99, "halign".into(), 105),
                                                Spanned(106, Atom::from("start"), 113)
                                            ),
                                            WidgetCallArg::new(
                                                Spanned(115, "orientation".into(), 126),
                                                Spanned(127, Atom::from("h"), 130)
                                            ),
                                            WidgetCallArg::new(
                                                Spanned(132, "space-evenly".into(), 144),
                                                Spanned(145, Atom::from(false), 150)
                                            )
                                        ],
                                        Vec::<WidgetCallChild>::new(),
                                    ),
                                    151
                                ))
                                .into(),
                                Spanned::from((
                                    172,
                                    WidgetCall::new(
                                        (173, "box".into(), 176),
                                        vec![
                                            WidgetCallArg::new(
                                                Spanned(178, "halign".into(), 184),
                                                Spanned(185, Atom::from("end"), 190)
                                            ),
                                            WidgetCallArg::new(
                                                Spanned(192, "orientation".into(), 203),
                                                Spanned(204, Atom::from("h"), 207)
                                            ),
                                            WidgetCallArg::new(
                                                Spanned(209, "space-evenly".into(), 221),
                                                Spanned(222, Atom::from(false), 227)
                                            )
                                        ],
                                        Vec::<WidgetCallChild>::new(),
                                    ),
                                    228
                                ))
                                .into()
                            ]
                        ),
                        246
                    ))],
                )
                .into(),
                260
            )
                .into())
        );
    }

    fn test(inp: &str) -> (Vec<ParseError>, Result<Spanned<TopLevelExpr>, LexicalError>) {
        let _ = env_logger::builder().is_test(true).try_init();

        let lexer = Lexer::new(inp);
        let parser = crate::grammar::DefWidgetParser::new();

        let mut errors = Vec::new();

        let result = parser.parse(&mut errors, lexer).map_err(|e| match e {
            lalrpop_util::ParseError::User { error } => error,
            e => unimplemented!("this error should not happen: {:?}", e),
        });

        (errors, result)
    }
}
