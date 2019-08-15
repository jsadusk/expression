use crate::error::*;
use crate::expression::*;

use std::marker::PhantomData;

pub trait RandomListExpression<ElementType> {
    fn terms(&self) -> Terms;
    fn size(&self) -> usize;
    fn eval_element(&self, index: usize) -> ExpressionResult<ElementType>;
}

pub(crate) struct RandomListExpressionWrapper<ElementType, Expr: RandomListExpression<ElementType>>(Expr, PhantomData<ElementType>);

impl<ElementType, Expr: RandomListExpression<ElementType>> Expression<Vec<ElementType>> for RandomListExpressionWrapper<ElementType, Expr> {
    fn terms(&self) -> Terms {
        self.0.terms()
    }

    fn eval(&self) -> ExpressionResult<Vec<ElementType>> {
        let mut result =  Vec::<ElementType>::new();
        
        for i in 0..self.0.size() {
            result.push(self.0.eval_element(i)?);
        }

        Ok(result)
    }
}
