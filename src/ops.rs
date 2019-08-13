use std::ops::Mul;

use crate::error::*;
use crate::expression::*;

pub struct Value<ValueType: Clone> {
    pub val : ValueType
}

impl<ValueType: Clone> Expression<ValueType> for Value<ValueType> {
    fn eval(&self) -> ExpressionResult<ValueType> {
        Ok(self.val.clone())
    }

    fn terms(&self) -> Vec<Term> { Vec::<Term>::new() }
}

pub struct Multiply<ValueType: Mul + Copy> {
    pub operand: TypedTerm<ValueType>,
    pub factor: ValueType
}

impl<ValueType: Mul + Copy> Expression<ValueType::Output> for Multiply<ValueType> {
    fn terms(&self) -> Terms {
        vec!(self.operand.term())
    }
    
    fn eval(&self) -> ExpressionResult<ValueType::Output> {
        let result = *self.operand.get()? * self.factor;
        Ok(result)
    }
}
