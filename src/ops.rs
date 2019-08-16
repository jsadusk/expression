use std::ops::Mul;

use crate::error::*;
use crate::expression::*;
use crate::list::*;

pub struct Value<ValueType: Clone> {
    pub val : ValueType
}

impl<ValueType: Clone> Expression<ValueType> for Value<ValueType> {
    fn eval(&self) -> ExpressionResult<ValueType> {
        Ok(self.val.clone())
    }

    fn terms(&self) -> Vec<Term> { Vec::<Term>::new() }
}

pub struct Coefficient<ValueType: Mul + Copy> {
    pub operand: TypedTerm<ValueType>,
    pub factor: ValueType
}

impl<ValueType: Mul + Copy> Expression<ValueType::Output> for Coefficient<ValueType> {
    fn terms(&self) -> Terms {
        vec!(self.operand.term())
    }
    
    fn eval(&self) -> ExpressionResult<ValueType::Output> {
        let result = *self.operand.get()? * self.factor;
        Ok(result)
    }
}

pub struct Multiply<ValueType: Mul + Copy> {
    pub a: TypedTerm<ValueType>,
    pub b: TypedTerm<ValueType>
}

impl<ValueType: Mul + Copy> Expression<ValueType::Output> for Multiply<ValueType> {
    fn terms(&self) -> Terms {
        vec!(self.a.term(), self.b.term())
    }

    fn eval(&self) -> ExpressionResult<ValueType::Output> {
        Ok(*self.a.get()? * *self.b.get()?)
    }
}

pub struct MultiplyListScalar<ElementType: Mul + Copy> {
    pub l: TypedTerm<Vec<ElementType>>,
    pub c: TypedTerm<ElementType>
}

impl<ElementType: Mul + Copy> RandomListExpression<ElementType::Output> for MultiplyListScalar<ElementType> {
    fn terms(&self) -> Terms {
        vec!(self.l.term(), self.c.term())
    }

    fn len(&self) -> ExpressionResult<usize> {
        Ok(self.l.get()?.len())
    }

    fn eval_element(&self, index: usize) -> ExpressionResult<ElementType::Output> {
        Ok(self.l.get()?[index] * *self.c.get()?)
    }
}
