use crate::error::*;
use worm_cell::{WormCell, WormCellReader};

use std::ops::Deref;

#[derive(Debug, Copy, Clone)]
pub struct Term(pub(crate) usize);

#[derive(Debug, Copy, Clone)]
pub struct TypedTerm<ResultType> {
    pub(crate) term: Term,
    pub(crate) result: WormCellReader<ResultType>
}

pub trait Expression<ValueType> {
    fn terms(&self) -> Terms;
    fn eval(&self) -> ExpressionResult<ValueType>;
}

pub type Terms = Vec<Term>;

pub(crate)struct TypedExpressionCache<ResultType, Expr: Expression<ResultType>> {
    pub expr: Expr,
    pub result: WormCell<ResultType>
}

pub(crate) trait ExpressionCache {
    fn evaluated(&self) -> bool;
    fn terms(&self) -> Terms;
    fn eval(&mut self) -> ExpressionResult<()>;
}

impl<ResultType, Expr: Expression<ResultType>> ExpressionCache for TypedExpressionCache<ResultType, Expr> {
    fn terms(&self) -> Terms {
        self.expr.terms()
    }

    fn eval(&mut self) -> ExpressionResult<()> {
        self.result.set(self.expr.eval()?).map_err(ExpressionError::DoubleCalc)
    }

    fn evaluated(&self) -> bool {
        self.result.is_set()
    }
 }


impl<ResultType> TypedTerm<ResultType> {
    pub fn get(&self) -> ExpressionResult<&ResultType> {
        self.result.get().map_err(ExpressionError::GetNotCalculated)
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
