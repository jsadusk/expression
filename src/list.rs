use crate::error::*;
use crate::expression::*;

use std::marker::PhantomData;

pub trait RandomListExpression<ElementType> {
    fn terms(&self) -> Terms;
    fn len(&self) -> ExpressionResult<usize>;
    fn eval_element(&self, index: usize) -> ExpressionResult<ElementType>;
}

pub(crate) struct RandomListExpressionWrapper<ElementType, Expr: RandomListExpression<ElementType>>(pub(crate) Expr, pub(crate) PhantomData<ElementType>);

impl<ElementType, Expr: RandomListExpression<ElementType>> Expression<Vec<ElementType>> for RandomListExpressionWrapper<ElementType, Expr> {
    fn terms(&self) -> Terms {
        self.0.terms()
    }

    fn eval(&self) -> ExpressionResult<Vec<ElementType>> {
        let mut result =  Vec::<ElementType>::new();

        for i in 0..self.0.len()? {
            result.push(self.0.eval_element(i)?);
        }

        Ok(result)
    }
}

pub trait SequentialListExpression<ElementType> {
    fn terms(&self) -> Terms;
    fn eval_next(&self, prev: &Vec<ElementType>) -> ExpressionResult<Option<ElementType>>;
}

pub(crate) struct SequentialListExpressionWrapper<ElementType, Expr: SequentialListExpression<ElementType>>(pub(crate) Expr, pub(crate) PhantomData<ElementType>);

impl<ElementType, Expr: SequentialListExpression<ElementType>> Expression<Vec<ElementType>> for SequentialListExpressionWrapper<ElementType, Expr> {
    fn terms(&self) -> Terms {
        self.0.terms()
    }

    fn eval(&self) -> ExpressionResult<Vec<ElementType>> {
        let mut result =  Vec::<ElementType>::new();

        let mut maybe_elem = self.0.eval_next(&result)?;

        while let Some(elem) = maybe_elem {
            result.push(elem);
            maybe_elem = self.0.eval_next(&result)?;
        }

        Ok(result)
    }
}
