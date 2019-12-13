use crate::error::*;
use worm_cell::{WormCell, WormCellReader};

use std::convert::Into;
use std::ops::{Deref, Index};

#[derive(Debug, Copy, Clone)]
pub struct Term(pub(crate) usize);

#[derive(Debug, Clone)]
pub struct TypedTermImpl<ResultType> {
    pub(crate) term: Term,
    pub(crate) result: WormCellReader<ResultType>,
}

pub trait Expression {
    type ValueType;
    type ErrorType;

    fn terms(&self) -> Terms;
    fn eval(&self) -> Result<Self::ValueType, Self::ErrorType>;
}

pub type Terms = Vec<Term>;

pub(crate) struct TypedExpressionCache<Expr: Expression> {
    pub expr: Expr,
    pub result: WormCell<Expr::ValueType>,
}

impl<Expr: Expression> TypedExpressionCache<Expr> {
    pub fn new(expr: Expr) -> Self {
        TypedExpressionCache::<Expr> {
            expr: expr,
            result: WormCell::<Expr::ValueType>::new(),
        }
    }
}

pub(crate) trait ExpressionCache<EvalErrorType>
where
    EvalErrorType: std::error::Error + 'static,
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
            Ok(val) => self
                .result
                .set(val)
                .map_err(|e| ExpressionError::<EvalErrorType>::Engine(EngineError::DoubleCalc(e))),
            Err(e) => Err(ExpressionError::<EvalErrorType>::Eval(e.into())),
        }
    }

    fn evaluated(&self) -> bool {
        self.result.is_set()
    }
}

pub trait TypedTerm {
    type ValueType;
    fn get(&self) -> EngineResult<&Self::ValueType>;
    fn term(&self) -> Term;
}

impl<ResultType> TypedTerm for TypedTermImpl<ResultType> {
    type ValueType = ResultType;

    fn get(&self) -> EngineResult<&ResultType> {
        self.result.get().map_err(EngineError::GetNotCalculated)
    }

    fn term(&self) -> Term {
        self.term
    }
}

#[derive(Debug, Clone)]
pub struct ListTermImpl<ElementType>(pub(crate) TypedTermImpl<Vec<ElementType>>);

pub trait ListTerm : TypedTerm {
    type ElementType;
    fn len(&self) -> EngineResult<usize>;
}

impl<ElementType> TypedTerm for ListTermImpl<ElementType> {
    type ValueType = Vec<ElementType>;
    fn get(&self) -> EngineResult<&Vec<ElementType>> {
        self.0.get()
    }

    fn term(&self) -> Term {
        self.0.term()
    }
}

impl<ElementType> ListTerm for ListTermImpl<ElementType> {
    type ElementType = ElementType;
    fn len(&self) -> EngineResult<usize> {
        Ok(self.get()?.len())
    }
}

pub struct TermResult<TermImpl>(pub(crate) TermImpl);

impl<TermImpl> TermResult<TermImpl>
where
    TermImpl: TypedTerm,
{
    pub fn get(&self) -> EngineResult<&TermImpl::ValueType> {
        self.0.get()
    }

    pub fn term(&self) -> Term {
        self.0.term()
    }
}

impl<TermImpl> Deref for TermResult<TermImpl>
where
    TermImpl: TypedTerm,
{
    type Target = TermImpl::ValueType;

    fn deref(&self) -> &Self::Target {
        self.get().unwrap()
    }
}

impl<TermImpl> From<TermImpl> for TermResult<TermImpl> {
    fn from(other: TermImpl) -> Self {
        Self(other)
    }
}

pub struct TermListResult<TermImpl>(pub(crate) TermImpl);

impl<TermImpl> TermListResult<TermImpl>
where
    TermImpl: ListTerm,
{
    pub fn get(&self) -> EngineResult<&TermImpl::ValueType> {
        self.0.get()
    }

    pub fn term(&self) -> Term {
        self.0.term()
    }

    pub fn len(&self) -> usize {
        self.0.len().unwrap()
    }
}

impl<TermImpl> From<TermImpl> for TermListResult<TermImpl> {
    fn from(other: TermImpl) -> Self {
        Self(other)
    }
}

impl<TermImpl> Deref for TermListResult<TermImpl>
where
    TermImpl: ListTerm,
{
    type Target = TermImpl::ValueType;

    fn deref(&self) -> &Self::Target {
        self.get().unwrap()
    }
}

impl<TermImpl> Index<usize> for TermListResult<TermImpl>
where
    TermImpl: ListTerm,
    TermImpl::ValueType: Index<usize>
    // need quality constraints
    //TermImpl::ElementType = TermImpl::ValueType::Output
{
    type Output = <TermImpl::ValueType as Index<usize>>::Output;

    fn index(&self, i: usize) -> &Self::Output {
        &self.get().unwrap()[i]
    }
}
