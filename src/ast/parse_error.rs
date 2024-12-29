use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

// TODO: Create struct with common properties.
// Then remove workaround implementation
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

impl ParseError {
    pub fn span(&self) -> &SourceSpan {
        match self {
            ParseError::UnexpectedToken { err_span } => err_span,
            ParseError::ExpectedDefListenName { err_span } => err_span,
            ParseError::ExpectedDefListenScript { err_span } => err_span,
            ParseError::ExpectedDefPollName { err_span } => err_span,
            ParseError::ExpectedDefPollScript { err_span } => err_span,
            ParseError::ExpectedDefVarName { err_span } => err_span,
            ParseError::ExpectedDefVarValue { err_span } => err_span,
            ParseError::ExpectedDefWidgetName { err_span } => err_span,
            ParseError::ExpectedDefWidgetParams { err_span } => err_span,
            ParseError::ExpectedDefWindowName { err_span } => err_span,
            ParseError::ExpectedIncludePath { err_span } => err_span,
        }
    }
}
