use super::{
    atom::{Atom, Symbol},
    binary_operation::BinaryOperation,
    comparison_operation::ComparisonOperation,
    function_call::FunctionCall,
    ternary_operation::TernaryOperation,
    unary_operation::UnaryOperation,
};

// TODO: Rename to "Expression"
#[derive(Debug, PartialEq, PartialOrd)]
pub enum PrimitiveExpr {
    Atom(Atom),
    BinaryOperation(BinaryOperation),
    ComparisonOperation(ComparisonOperation),
    FunctionCall(FunctionCall),
    Symbol(Symbol),
    TernaryOperation(TernaryOperation),
    UnaryOperation(UnaryOperation),
    // TODO: Evaluate if necessary.
    Err,
}

impl From<bool> for PrimitiveExpr {
    fn from(value: bool) -> Self {
        Atom::from(value).into()
    }
}

impl From<&str> for PrimitiveExpr {
    fn from(value: &str) -> Self {
        Self::Atom(Atom::from(value))
    }
}

impl From<Atom> for PrimitiveExpr {
    fn from(value: Atom) -> Self {
        Self::Atom(value)
    }
}

impl From<BinaryOperation> for PrimitiveExpr {
    fn from(value: BinaryOperation) -> Self {
        Self::BinaryOperation(value)
    }
}

impl From<ComparisonOperation> for PrimitiveExpr {
    fn from(value: ComparisonOperation) -> Self {
        Self::ComparisonOperation(value)
    }
}

impl From<FunctionCall> for PrimitiveExpr {
    fn from(value: FunctionCall) -> Self {
        Self::FunctionCall(value)
    }
}

impl From<TernaryOperation> for PrimitiveExpr {
    fn from(value: TernaryOperation) -> Self {
        Self::TernaryOperation(value)
    }
}

impl From<UnaryOperation> for PrimitiveExpr {
    fn from(value: UnaryOperation) -> Self {
        Self::UnaryOperation(value)
    }
}
