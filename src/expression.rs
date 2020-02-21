use crate::error::*;

use std::sync::Arc;
use std::convert::Into;
use std::ops::{Deref, Index};
use worm_cell::{AtomicWormCell, AtomicWormCellReader};

#[derive(Debug, Copy, Clone)]
pub struct Term(pub(crate) usize);

#[derive(Debug, Clone)]
pub struct TypedTermImpl<ResultType> {
    pub(crate) term: Term,
    pub(crate) result: AtomicWormCellReader<ResultType>,
}

pub trait Expression {
    type ValueType;
    type ErrorType;

    fn terms(&self) -> Terms;
    fn eval(&self) -> Result<Self::ValueType, Self::ErrorType>;
}

pub type Terms = Vec<Term>;

pub(crate) struct TypedExpressionCache<Expr: Expression>
{
    pub expr: Expr,
    pub result: Arc<AtomicWormCell<Expr::ValueType>>
}

impl<Expr: Expression> TypedExpressionCache<Expr>
{
    pub fn new(expr: Expr) -> Self {
        TypedExpressionCache::<Expr> {
            expr: expr,
            result: Arc::new(AtomicWormCell::<Expr::ValueType>::new())
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
            Ok(val) => Ok(self.result.try_set(val)?),
            Err(e) => Err(ExpressionError::<EvalErrorType>::Eval(e.into())),
        }
    }

    fn evaluated(&self) -> bool {
        self.result.is_set()
    }
}

pub trait TypedTerm {
    type ValueType;
    fn get(&self) -> &Self::ValueType;
    fn try_get(&self) -> EngineResult<&Self::ValueType>;
    fn term(&self) -> Term;
}

impl<ResultType> TypedTerm for TypedTermImpl<ResultType> {
    type ValueType = ResultType;

    fn get(&self) -> &Self::ValueType {
        self.result.get()
    }

    fn try_get(&self) -> EngineResult<&Self::ValueType> {
        Ok(self.result.try_get()?)
    }

    fn term(&self) -> Term {
        self.term
    }
}

#[derive(Debug, Clone)]
pub struct ListTermImpl<ElementType>(pub(crate) TypedTermImpl<Vec<ElementType>>);

pub trait ListTerm : TypedTerm {
    type ElementType;
    fn try_len(&self) -> EngineResult<usize>;
    fn len(&self) -> usize;
    fn iter(&self) -> std::slice::Iter<Self::ElementType>;
}

impl<ElementType> TypedTerm for ListTermImpl<ElementType> {
    type ValueType = Vec<ElementType>;
    
    fn get(&self) -> &Self::ValueType {
        self.0.get()
    }

    fn try_get(&self) -> EngineResult<&Self::ValueType> {
        self.0.try_get()
    }

    fn term(&self) -> Term {
        self.0.term()
    }
}

impl<ElementType> ListTerm for ListTermImpl<ElementType> {
    type ElementType = ElementType;

    fn len(&self) -> usize {
        self.get().len()
    }

    fn try_len(&self) -> EngineResult<usize> {
        Ok(self.try_get()?.len())
    }

    fn iter(&self) -> std::slice::Iter<Self::ElementType> {
        self.get().iter()
    }
}

pub struct TermResult<TermImpl>(pub(crate) TermImpl);

impl<TermImpl> TermResult<TermImpl>
where
    TermImpl: TypedTerm,
{
    pub fn get(&self) -> &TermImpl::ValueType {
        self.0.get()
    }

    pub fn try_get(&self) -> EngineResult<&TermImpl::ValueType> {
        self.0.try_get()
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
        self.get()
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
    pub fn try_get(&self) -> EngineResult<&TermImpl::ValueType> {
        self.0.try_get()
    }

    pub fn get(&self) -> &TermImpl::ValueType {
        self.0.get()
    }

    pub fn term(&self) -> Term {
        self.0.term()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn try_len(&self) -> EngineResult<usize> {
        self.0.try_len()
    }

    pub fn iter(&self) -> std::slice::Iter<TermImpl::ElementType> {
        self.0.iter()
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
        self.get()
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
        &self.get()[i]
    }
}
