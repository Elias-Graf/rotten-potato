#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Tok<'inp> {
    DelimiterComma,
    DelimiterLeftBracket,
    DelimiterLeftParen,
    DelimiterRightBracket,
    DelimiterRightParen,
    KeywordDefListen,
    KeywordDefPoll,
    KeywordDefVar,
    KeywordDefWidget,
    KeywordDefWindow,
    KeywordFalse,
    KeywordInclude,
    KeywordLiteral,
    KeywordTrue,
    LiteralNumber(&'inp str),
    LiteralString(&'inp str),
    LiteralSymbol(&'inp str),
    OperatorComparisonEqual,
    OperatorComparisonGreaterThan,
    OperatorComparisonGreaterThanOrEquals,
    OperatorComparisonLessThan,
    OperatorComparisonLessThanOrEquals,
    OperatorComparisonNotEqual,
    OperatorLogicalAnd,
    OperatorLogicalNot,
    OperatorLogicalOr,
    OperatorMathAddition,
    OperatorMathDivision,
    OperatorMathModulo,
    OperatorMathMultiplication,
    OperatorMathSubtraction,
    PunctuationColon,
    PunctuationQuestionMark,
}
