use crate::error::*;
use worm_cell::{WormCell, WormCellReader};

use std::ops::Deref;
use std::convert::Into;

#[derive(Debug, Copy, Clone)]
pub struct Term(pub(crate) usize);

#[derive(Debug, Copy, Clone)]
pub struct TypedTerm<ResultType> {
    pub(crate) term: Term,
    pub(crate) result: WormCellReader<ResultType>
}

pub trait Expression {
    type ValueType;
    type ErrorType;

    fn terms(&self) -> Terms;
    fn eval(&self) -> Result<Self::ValueType, Self::ErrorType>;
}

pub type Terms = Vec<Term>;

pub(crate)struct TypedExpressionCache<Expr: Expression> {
    pub expr: Expr,
    pub result: WormCell<Expr::ValueType>,
}

impl<Expr: Expression> TypedExpressionCache<Expr> {
    pub fn new(expr: Expr) -> Self {
        TypedExpressionCache::<Expr> {
            expr: expr,
            result: WormCell::<Expr::ValueType>::new() }
    }
}

pub(crate) trait ExpressionCache<EvalErrorType>
where EvalErrorType: std::error::Error + 'static
{
    fn evaluated(&self) -> bool;
    fn terms(&self) -> Terms;
    fn eval(&mut self) -> ExpressionResult<(), EvalErrorType>;
}

impl<Expr, EvalErrorType> ExpressionCache<EvalErrorType> for TypedExpressionCache<Expr>
where
    Expr: Expression,
    EvalErrorType: std::error::Error + From<Expr::ErrorType> + 'static,
{
    fn terms(&self) -> Terms {
        self.expr.terms()
    }

    fn eval(&mut self) -> Result<(), ExpressionError<EvalErrorType>> {
        match self.expr.eval() {
            Ok(val) => self.result.set(val).map_err(|e| ExpressionError::<EvalErrorType>::Engine(EngineError::DoubleCalc(e))),
            Err(e) => Err(ExpressionError::<EvalErrorType>::Eval(e.into()))
        }
    }

    fn evaluated(&self) -> bool {
        self.result.is_set()
    }
}


impl<ResultType> TypedTerm<ResultType> {
    pub fn get(&self) -> EngineResult<&ResultType> {
        self.result.get().map_err(EngineError::GetNotCalculated)
    }

    pub fn term(&self) -> Term {
        self.term
    }
}

impl<ResultType> Deref for TypedTerm<ResultType> {
    type Target = ResultType;

    fn deref(&self) -> &Self::Target {
        self.get().unwrap()
    }
}
