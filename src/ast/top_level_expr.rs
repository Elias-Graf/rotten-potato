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
            def_window::{DefWindow, DefWindowArg, DefWindowContent},
            include::Include,
            literal::{Literal, LiteralArg},
        },
        grammar, lexer,
    };

    #[test]
    fn test() {
        let _ = env_logger::builder().is_test(true).try_init();

        let inp = r#"
            (include "listeners.yuck")

            (defvar foo "bar")
            (defpoll volume :interval "1s" "scripts/getvol")
            (deflisten music :initial ""
              "playerctl --follow metadata --format '{{ artist }} - {{ title }}' || true")
            (defwidget bar [])
            (defwidget sidestuff [])
            (defwidget metric [label value onchange])
            (defwindow bar)
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
                (13, Include::new("listeners.yuck").into(), 39).into(),
                (53, DefVar::new("foo", Atom::from("bar")).into(), 71).into(),
                (
                    84,
                    DefPoll::new(
                        "volume",
                        vec![DefPollArg::new("interval", Atom::from("1s"))],
                        "scripts/getvol".to_owned()
                    )
                    .into(),
                    132
                )
                    .into(),
                (
                    145,
                    DefListen::new(
                        "music",
                        vec![DefListenArg::new("initial", Atom::from(""))],
                        "playerctl --follow metadata --format '{{ artist }} - {{ title }}' || true"
                            .to_owned()
                    )
                    .into(),
                    264
                )
                    .into(),
                (
                    277,
                    DefWidget::new(
                        "bar",
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
                        "sidestuff",
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
                        "metric",
                        vec![
                            DefWidgetParam::new("label", false),
                            DefWidgetParam::new("value", false),
                            DefWidgetParam::new("onchange", false)
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
                        "bar",
                        Vec::<DefWindowArg>::new(),
                        Vec::<DefWindowContent>::new()
                    )
                    .into(),
                    414
                )
                    .into(),
                (
                    427,
                    Literal::new(vec![LiteralArg::new(
                        "content",
                        "(button 'click')".to_owned()
                    )])
                    .into(),
                    464
                )
                    .into(),
            ])
        );
    }
}
