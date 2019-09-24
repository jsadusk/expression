use std::ops::Mul;

use crate::expression::*;
use crate::list::*;
use crate::error::OpError;

pub struct Value<ValueType: Clone> {
    pub val : ValueType
}

impl<ValueType: Clone> Expression for Value<ValueType> {
    type ValueType = ValueType;
    type ErrorType = ();

    fn eval(&self) -> Result<Self::ValueType, Self::ErrorType> {
        Ok(self.val.clone())
    }

    fn terms(&self) -> Vec<Term> { Vec::<Term>::new() }
}

pub struct Coefficient<ValueType: Mul + Copy> {
    pub operand: TypedTerm<ValueType>,
    pub factor: ValueType
}

impl<ValueType: Mul + Copy> Expression for Coefficient<ValueType> {
    type ValueType = ValueType::Output;
    type ErrorType = OpError;

    fn terms(&self) -> Terms {
        vec!(self.operand.term())
    }

    fn eval(&self) -> Result<Self::ValueType, OpError> {
        let result = *self.operand * self.factor;
        Ok(result)
    }
}

pub struct Multiply<ValueType: Mul + Copy> {
    pub a: TypedTerm<ValueType>,
    pub b: TypedTerm<ValueType>
}

impl<ValueType: Mul + Copy> Expression for Multiply<ValueType> {
    type ValueType = ValueType::Output;
    type ErrorType = OpError;

    fn terms(&self) -> Terms {
        vec!(self.a.term(), self.b.term())
    }

    fn eval(&self) -> Result<Self::ValueType, OpError> {
        Ok(*self.a * *self.b)
    }
}

pub struct MultiplyListScalar<ElementType: Mul + Copy> {
    pub l: TypedTerm<Vec<ElementType>>,
    pub c: TypedTerm<ElementType>
}

impl<ElementType: Mul + Copy> RandomListExpression for MultiplyListScalar<ElementType> {
    type ElementType = ElementType::Output;
    type ErrorType = OpError;

    fn terms(&self) -> Terms {
        vec!(self.l.term(), self.c.term())
    }

    fn len(&self) -> usize {
        self.l.len()
    }

    fn eval_element(&self, index: usize) -> Result<<ElementType as Mul>::Output, OpError> {
        Ok(self.l[index] * *self.c)
    }
}

pub struct CountList {
    pub start: TypedTerm<i32>,
    pub end: TypedTerm<i32>,
    pub inc: TypedTerm<i32>
}

impl SequentialListExpression for CountList {
    type ElementType = i32;
    type ErrorType = OpError;

    fn terms(&self) -> Terms {
        vec!(self.start.term(), self.end.term(), self.inc.term())
    }

    fn eval_next(&self, prev: &Vec<i32>) -> Result<Option<i32>, OpError> {
        if prev.len() == 0 {
            Ok(Some(*self.start))
        } else if (prev.len() as i32) <= (*self.end - *self.start) / *self.inc {
            Ok(Some(prev[prev.len() - 1] + *self.inc))
        } else {
            Ok(None)
        }
    }
}
