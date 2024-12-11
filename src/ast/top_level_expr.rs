use super::{
    def_listen::DefListen, def_poll::DefPoll, def_var::DefVar, def_widget::DefWidget,
    def_window::DefWindow, include::Include, literal::Literal,
};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum TopLevelExpr {
    DefListen(DefListen),
    DefPoll(DefPoll),
    DefVar(DefVar),
    DefWidget(DefWidget),
    DefWindow(DefWindow),
    Err,
    Include(Include),
    Literal(Literal),
}

impl From<DefListen> for TopLevelExpr {
    fn from(value: DefListen) -> Self {
        Self::DefListen(value)
    }
}

impl From<DefPoll> for TopLevelExpr {
    fn from(value: DefPoll) -> Self {
        Self::DefPoll(value)
    }
}

impl From<DefVar> for TopLevelExpr {
    fn from(value: DefVar) -> Self {
        Self::DefVar(value)
    }
}

impl From<DefWidget> for TopLevelExpr {
    fn from(value: DefWidget) -> Self {
        Self::DefWidget(value)
    }
}

impl From<DefWindow> for TopLevelExpr {
    fn from(value: DefWindow) -> Self {
        Self::DefWindow(value)
    }
}

impl From<Include> for TopLevelExpr {
    fn from(value: Include) -> Self {
        Self::Include(value)
    }
}

impl From<Literal> for TopLevelExpr {
    fn from(value: Literal) -> Self {
        Self::Literal(value)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{
        ast::{
            atom::Atom,
            def_listen::{DefListen, DefListenArg},
            def_poll::{DefPoll, DefPollArg},
            def_var::DefVar,
            def_widget::{DefWidget, DefWidgetChild, DefWidgetParam},
            def_window::{DefWindow, DefWindowArg},
            include::Include,
            literal::{Literal, LiteralArg},
            widget_call::{WidgetCall, WidgetCallArg, WidgetCallChild},
            ParseError,
        },
        grammar,
        lexer::{self, LexicalError},
        spanned::Spanned,
    };

    use super::*;

    #[test]
    fn unexpected_token() {
        let inp = r#"
            (defvar foo "bar")
            abc
            (defpoll volume "scripts/getvol")
        "#;

        let (errors, ast) = test(inp);

        assert_eq!(
            errors,
            vec![ParseError::UnexpectedToken {
                err_span: (44, 3).into()
            }]
        );
        assert_eq!(
            ast,
            Ok(vec![
                (
                    13,
                    DefVar::new(
                        (14, 20),
                        (21, "foo".into(), 24),
                        Spanned::from((25, Atom::from("bar"), 30))
                    )
                    .into(),
                    31
                )
                    .into(),
                (44, TopLevelExpr::Err, 47).into(),
                (
                    60,
                    DefPoll::new(
                        (61, 68),
                        (69, "volume".into(), 75),
                        Vec::<DefPollArg>::new(),
                        (76, "scripts/getvol".to_owned(), 92)
                    )
                    .into(),
                    93
                )
                    .into()
            ])
        );
    }

    fn test(
        inp: &str,
    ) -> (
        Vec<ParseError>,
        Result<Vec<Spanned<TopLevelExpr>>, LexicalError>,
    ) {
        let mut errs = Vec::new();
        let lexer = lexer::Lexer::new(inp);
        let ast = grammar::TopLevelParser::new()
            .parse(&mut errs, lexer)
            .map_err(|e| match e {
                lalrpop_util::ParseError::User { error } => error,
                e => unimplemented!("this error should not happen: {:?}", e),
            });

        (errs, ast)
    }

    #[test]
    fn every_expr() {
        let _ = env_logger::builder().is_test(true).try_init();

        let inp = r#"
            (include "listeners.yuck")

            (defvar foo "bar")
            (defpoll volume :interval "1s" "scripts/getvol")
            (deflisten music :initial ""
              "playerctl --follow metadata --format '{{ artist }} - {{ title }}' || true")
            (defwidget baz [])
            (defwidget sidestuff [])
            (defwidget metric [label value onchange])
            (defwindow bar :type "dock"
                (metric :align "center"
                    "hey"
                    (sidestuff)))
            (literal :content "(button 'click')")"#;
        let mut errs = Vec::new();
        let lexer = lexer::Lexer::new(inp);
        let ast = grammar::TopLevelParser::new()
            .parse(&mut errs, lexer)
            .map_err(|e| match e {
                lalrpop_util::ParseError::User { error } => error,
                e => unimplemented!("this error should not happen: {:?}", e),
            });

        assert_eq!(errs, Vec::new());
        assert_eq!(
            ast,
            Ok(vec![
                (13, Include::new((14, 21), (22, "listeners.yuck".into(), 38)).into(), 39).into(),
                (
                    53,
                    DefVar::new(
                        (54, (), 60),
                        (61, "foo".into(), 64),
                        Spanned::from((65, Atom::from("bar").into(), 70))
                    )
                    .into(),
                    71
                )
                    .into(),
                (
                    84,
                    DefPoll::new(
                        (85, (), 92),
                        (93, "volume".into(), 99),
                        vec![DefPollArg::new((101, "interval".into(), 109), Spanned::from((110, Atom::from("1s"), 114)))],
                        (115, "scripts/getvol".into(), 131),
                    )
                    .into(),
                    132
                )
                    .into(),
                (
                    145,
                    DefListen::new(
                        (146, (), 155),
                        (156, "music".into(), 161),
                        vec![DefListenArg::new(
                            (163, "initial".into(), 170),
                            Spanned::from((171, Atom::from(""), 173))
                        )],
                        (188, "playerctl --follow metadata --format '{{ artist }} - {{ title }}' || true".into(), 263)
                    )
                    .into(),
                    264
                )
                    .into(),
                (
                    277,
                    DefWidget::new(
                        (278, (), 287),
                        (288, "baz".into(), 291),
                        Vec::<DefWidgetParam>::new(),
                        Vec::<DefWidgetChild>::new()
                    )
                    .into(),
                    295
                )
                    .into(),
                (
                    308,
                    DefWidget::new(
                        (309, (), 318),
                        (319, "sidestuff".into(), 328),
                        Vec::<DefWidgetParam>::new(),
                        Vec::<DefWidgetChild>::new()
                    )
                    .into(),
                    332
                )
                    .into(),
                (
                    345,
                    DefWidget::new(
                        (346, (), 355),
                        (356, "metric".into(), 362),
                        vec![
                            DefWidgetParam::new((364, "label".into(), 369), false),
                            DefWidgetParam::new((370, "value".into(), 375), false),
                            DefWidgetParam::new((376, "onchange".into(), 384), false)
                        ],
                        Vec::<DefWidgetChild>::new()
                    )
                    .into(),
                    386
                )
                    .into(),
                (
                    399,
                    DefWindow::new(
                        (400, 409),
                        (410, "bar".into(), 413),
                        vec![
                            DefWindowArg::new(
                                (415, "type".into(), 419),
                               Spanned::from((420, Atom::from("dock"), 426)),
                            )
                        ],
                        vec![
                            Spanned::from((
                                443,
                                WidgetCall::new(
                                    (444, "metric".into(), 450),
                                    vec![
                                        WidgetCallArg::new(
                                            Spanned(452, "align".into(), 457),
                                            Spanned(458, "center".into(), 466)
                                        )
                                    ],
                                    vec![
                                        Spanned::from((487, Atom::from("hey"), 492)).into(),
                                        Spanned::from((
                                            513,
                                            WidgetCall::new(
                                                (514, "sidestuff".into(), 523),
                                                Vec::<WidgetCallArg>::new(),
                                                Vec::<WidgetCallChild>::new(),
                                            ),
                                            524,
                                        ))
                                            .into()
                                    ],
                                ),
                                525
                            )),
                        ]
                    )
                    .into(),
                    526
                )
                    .into(),
                (
                    539,
                    Literal::new(
                        (540, 547),
                        vec![LiteralArg::new(
                            (549, "content".into(), 556),
                            Spanned::from((557, "(button 'click')".to_owned(), 575))
                    )])
                    .into(),
                    576
                )
                    .into(),
            ])
        );
    }
}
