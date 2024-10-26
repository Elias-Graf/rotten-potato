use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

pub mod atom;
pub mod binary_operation;
pub mod comparison_operation;
pub mod def_listen;
pub mod def_poll;
pub mod def_var;
pub mod def_widget;
pub mod def_window;
pub mod expr;
pub mod function_call;
pub mod include;
pub mod literal;
pub mod symbol;
pub mod ternary_operation;
pub mod top_level_expr;
pub mod unary_operation;
pub mod widget_call;

// TODO: move
#[derive(Clone, Error, Debug, Diagnostic, PartialEq)]
pub enum ParseError {
    #[error("expected a name")]
    ExpectedDefListenName {
        #[label("try providing a name here")]
        err_span: SourceSpan,
    },
    #[error("expected a script")]
    ExpectedDefListenScript {
        #[label("try providing a script here")]
        err_span: SourceSpan,
    },
    #[error("expected a name")]
    ExpectedDefPollName {
        #[label("try providing a name here")]
        err_span: SourceSpan,
    },
    #[error("expected a script")]
    ExpectedDefPollScript {
        #[label("try providing a script here")]
        err_span: SourceSpan,
    },
    #[error("expected a name")]
    ExpectedDefVarName {
        #[label("try providing a name here")]
        err_span: SourceSpan,
    },
    #[error("expected a value")]
    ExpectedDefVarValue {
        #[label("try providing a value here")]
        err_span: SourceSpan,
    },
    #[error("expected a name")]
    ExpectedDefWidgetName {
        #[label("try providing a name here")]
        err_span: SourceSpan,
    },
    #[error("expected parameters")]
    ExpectedDefWidgetParams {
        #[label("try providing parameters here")]
        err_span: SourceSpan,
    },
    #[error("expected a name")]
    ExpectedDefWindowName {
        #[label("try providing a name here")]
        err_span: SourceSpan,
    },
    #[error("expected a path")]
    ExpectedIncludePath {
        #[label("try providing a path here")]
        err_span: SourceSpan,
    },
}
