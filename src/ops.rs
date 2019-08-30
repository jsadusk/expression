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
        let result = *self.operand * self.factor;
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
        Ok(*self.a * *self.b)
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
        Ok(self.l.len())
    }

    fn eval_element(&self, index: usize) -> ExpressionResult<ElementType::Output> {
        Ok(self.l[index] * *self.c)
    }
}

pub struct CountList {
    pub start: TypedTerm<i32>,
    pub end: TypedTerm<i32>,
    pub inc: TypedTerm<i32>
}

impl SequentialListExpression<i32> for CountList {
    fn terms(&self) -> Terms {
        vec!(self.start.term(), self.end.term(), self.inc.term())
    }

    fn eval_next(&self, prev: &Vec<i32>) -> ExpressionResult<Option<i32>> {
        if prev.len() == 0 {
            Ok(Some(*self.start))
        } else if (prev.len() as i32) <= (*self.end - *self.start) / *self.inc {
            Ok(Some(prev[prev.len() - 1] + *self.inc))
        } else {
            Ok(None)
        }
    }
}
