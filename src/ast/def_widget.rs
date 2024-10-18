use super::{atom::Symbol, widget_call::WidgetCall};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DefWidget {
    pub name: Symbol,
    pub params: Vec<DefWidgetParam>,
    pub children: Vec<DefWidgetChild>,
}

impl DefWidget {
    pub fn new(
        name: impl Into<Symbol>,
        params: Vec<impl Into<DefWidgetParam>>,
        children: Vec<impl Into<DefWidgetChild>>,
    ) -> Self {
        Self {
            name: name.into(),
            params: params.into_iter().map(|a| a.into()).collect(),
            children: children.into_iter().map(|a| a.into()).collect(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DefWidgetParam {
    pub name: Symbol,
    pub is_optional: bool,
}

impl DefWidgetParam {
    pub fn new(name: impl Into<Symbol>, is_optional: bool) -> Self {
        Self {
            name: name.into(),
            is_optional,
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum DefWidgetChild {
    WidgetCall(WidgetCall),
}

impl From<WidgetCall> for DefWidgetChild {
    fn from(value: WidgetCall) -> Self {
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
    };

    #[test]
    fn missing_name() {
        // TODO: Restuls in broken parser
        // r#"(defwidget [])"#
        let (errs, ast) = test(r#"(defwidget)"#);

        assert_eq!(ast, Ok(TopLevelExpr::Err));
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

        assert_eq!(ast, Ok(TopLevelExpr::Err));
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
            Ok(DefWidget::new(
                "bar",
                vec![DefWidgetParam::new("name", false)],
                Vec::<DefWidgetChild>::new()
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
            Ok(DefWidget::new(
                "bar",
                vec![DefWidgetParam::new("name", true)],
                Vec::<DefWidgetChild>::new()
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
            Ok(DefWidget::new(
                "bar",
                vec![
                    DefWidgetParam::new("foo", false),
                    DefWidgetParam::new("name", true),
                    DefWidgetParam::new("bar", false),
                ],
                Vec::<DefWidgetChild>::new()
            )
            .into())
        );
    }

    #[test]
    fn children() {
        let (errs, ast) = test(
            r#"
            (defwidget bottombar [width]
              (centerbox :orientation "h"
                (box :halign "start" :orientation "h" :space-evenly false)
                (box :halign "end" :orientation "h" :space-evenly false)
              )
            )"#,
        );

        assert_eq!(errs, Vec::new());
        assert_eq!(
            ast,
            Ok(DefWidget::new(
                "bottombar",
                vec![DefWidgetParam::new("width", false)],
                vec![WidgetCall::new(
                    "centerbox",
                    vec![WidgetCallArg::new("orientation", Atom::from("h"))],
                    vec![
                        WidgetCall::new(
                            "box",
                            vec![
                                WidgetCallArg::new("halign", Atom::from("start")),
                                WidgetCallArg::new("orientation", Atom::from("h")),
                                WidgetCallArg::new("space-evenly", Atom::from(false))
                            ],
                            Vec::<WidgetCallChild>::new(),
                        ),
                        WidgetCall::new(
                            "box",
                            vec![
                                WidgetCallArg::new("halign", Atom::from("end")),
                                WidgetCallArg::new("orientation", Atom::from("h")),
                                WidgetCallArg::new("space-evenly", Atom::from(false))
                            ],
                            Vec::<WidgetCallChild>::new(),
                        )
                    ]
                )],
            )
            .into())
        );
    }

    fn test(inp: &str) -> (Vec<ParseError>, Result<TopLevelExpr, LexicalError>) {
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
