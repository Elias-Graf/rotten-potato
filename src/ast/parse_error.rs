use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Clone, Error, Debug, Diagnostic, PartialEq)]
pub enum ParseError {
    #[error("Token was not expected at this point")]
    UnexpectedToken { err_span: SourceSpan },
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
