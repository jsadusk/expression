use crate::expression::*;

use std::marker::PhantomData;

pub trait RandomListExpression<ElementType, ErrorType=()> {
    fn terms(&self) -> Terms;
    fn len(&self) -> usize;
    fn eval_element(&self, index: usize) -> Result<ElementType, ErrorType>;
}

pub(crate) struct RandomListExpressionWrapper<ElementType, ErrorType, Expr: RandomListExpression<ElementType, ErrorType>>(pub(crate) Expr, pub(crate) PhantomData<ElementType>, pub(crate) PhantomData<ErrorType>);

impl<ElementType, ErrorType, Expr: RandomListExpression<ElementType, ErrorType>> Expression<Vec<ElementType>, ErrorType> for RandomListExpressionWrapper<ElementType, ErrorType, Expr> {
    fn terms(&self) -> Terms {
        self.0.terms()
    }

    fn eval(&self) -> Result<Vec<ElementType>, ErrorType> {
        let mut result =  Vec::<ElementType>::new();

        for i in 0..self.0.len() {
            result.push(self.0.eval_element(i)?);
        }

        Ok(result)
    }
}

pub trait SequentialListExpression<ElementType, ErrorType=()> {
    fn terms(&self) -> Terms;
    fn eval_next(&self, prev: &Vec<ElementType>) -> Result<Option<ElementType>, ErrorType>;
}

pub(crate) struct SequentialListExpressionWrapper<ElementType, ErrorType, Expr: SequentialListExpression<ElementType, ErrorType>>(pub(crate) Expr, pub(crate) PhantomData<ElementType>, pub(crate) PhantomData<ErrorType>);

impl<ElementType, ErrorType, Expr: SequentialListExpression<ElementType, ErrorType>> Expression<Vec<ElementType>, ErrorType> for SequentialListExpressionWrapper<ElementType, ErrorType, Expr> {
    fn terms(&self) -> Terms {
        self.0.terms()
    }

    fn eval(&self) -> Result<Vec<ElementType>, ErrorType> {
        let mut result =  Vec::<ElementType>::new();

        let mut maybe_elem = self.0.eval_next(&result)?;

        while let Some(elem) = maybe_elem {
            result.push(elem);
            maybe_elem = self.0.eval_next(&result)?;
        }

        Ok(result)
    }
}
