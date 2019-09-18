use crate::error::*;
use worm_cell::{WormCell, WormCellReader};

use std::ops::Deref;
use std::marker::PhantomData;
use std::convert::Into;

#[derive(Debug, Copy, Clone)]
pub struct Term(pub(crate) usize);

#[derive(Debug, Copy, Clone)]
pub struct TypedTerm<ResultType> {
    pub(crate) term: Term,
    pub(crate) result: WormCellReader<ResultType>
}

pub trait Expression<ValueType, ErrorType> {
    fn terms(&self) -> Terms;
    fn eval(&self) -> Result<ValueType, ErrorType>;
}

pub type Terms = Vec<Term>;

pub(crate)struct TypedExpressionCache<ResultType, ExprErrorType, Expr: Expression<ResultType, ExprErrorType>> {
    pub expr: Expr,
    pub result: WormCell<ResultType>,
    pub _e: PhantomData<ExprErrorType>

}

pub(crate) trait ExpressionCache<EvalErrorType: std::error::Error + 'static> {
    fn evaluated(&self) -> bool;
    fn terms(&self) -> Terms;
    fn eval(&mut self) -> ExpressionResult<(), EvalErrorType>;
}

impl<ResultType, EvalErrorType: std::error::Error + 'static, ExprErrorType: Into<EvalErrorType> + 'static, Expr: Expression<ResultType, ExprErrorType>> ExpressionCache<EvalErrorType> for TypedExpressionCache<ResultType, ExprErrorType, Expr> {

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
