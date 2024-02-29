use std::ops::{Mul, Index};

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

pub struct ListValue<ElementType> {
    pub val : Vec<ElementType>
}

impl<ElementType: Clone> ListExpression for ListValue<ElementType> {
    type ElementType = ElementType;
    type ErrorType = ();

    fn eval(&self) -> Result<Vec<Self::ElementType>, Self::ErrorType> {
        Ok(self.val.clone())
    }

    fn terms(&self) -> Vec<Term> { Vec::<Term>::new() }
}

pub struct Coefficient<T>
    where T: TypedTerm
{
    pub operand: TermResult<T>,
    pub factor: T::ValueType
}

impl<T> Expression for Coefficient<T>
where
    T: TypedTerm,
    T::ValueType: Copy + Mul,
{
    type ValueType = <T::ValueType as Mul>::Output;
    type ErrorType = OpError;

    fn terms(&self) -> Terms {
        vec!(self.operand.term())
    }

    fn eval(&self) -> Result<Self::ValueType, OpError> {
        let result = *self.operand * self.factor;
        Ok(result)
    }
}

pub struct Multiply<T, U> {
    pub a: TermResult<T>,
    pub b: TermResult<U>
}

impl<T, U> Expression for Multiply<T, U>
where
    T: TypedTerm,
    U: TypedTerm,
    T::ValueType: Mul + Copy + Mul<U::ValueType>,
    U::ValueType: Copy
{
    type ValueType = <T::ValueType as std::ops::Mul<U::ValueType>>::Output;
    type ErrorType = OpError;

    fn terms(&self) -> Terms {
        vec!(self.a.term(), self.b.term())
    }

    fn eval(&self) -> Result<Self::ValueType, OpError> {
        Ok(*self.a * *self.b)
    }
}

pub struct MultiplyListScalar<'a, L, T>
{
    pub l: TermListResult<L>,
    pub c: TermResult<T>,
}

impl<'a, L, T> MapExpression for MultiplyListScalar<'a, L, T>
where
    L: ListTerm,
    T: TypedTerm<ValueType=L::ElementType>,
    L::ElementType: Mul + Copy + Mul<T::ValueType>,
    T::ValueType: Copy,
    L::ValueType: Index<usize>,
    <<L as TypedTerm>::ValueType as Index<usize>>::Output: Copy + Mul + Mul<T::ValueType>
{
    type ElementType = <<<L as TypedTerm>::ValueType as Index<usize>>::Output as Mul<<L as ListTerm>::ElementType>>::Output;
    type ErrorType = OpError;
    type ElementSetup = &'a L::ElementType;

    fn terms(&self) -> Terms {
        vec!(self.l.term(), self.c.term())
    }

    fn setup(&self) -> Result<Vec<Self::ElementSetup>, OpError> {
        Ok(self.setup_iter(self.l.iter()))
    }

    fn eval_element(&self, list_elem: &Self::ElementSetup) -> Result<Self::ElementType, OpError> {
        Ok(*list_elem * *self.c)
    }
}

pub struct CountList<S, E, I>
{
    pub start: TermResult<S>,
    pub end: TermResult<E>,
    pub inc: TermResult<I>
}

impl<S, E, I> SequentialListExpression for CountList<S, E, I>
where
    S: TypedTerm<ValueType=i32>,
    E: TypedTerm<ValueType=i32>,
    I: TypedTerm<ValueType=i32>,
{
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

