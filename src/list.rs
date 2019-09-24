use crate::expression::*;

pub trait RandomListExpression {
    type ElementType;
    type ErrorType;

    fn terms(&self) -> Terms;
    fn len(&self) -> usize;
    fn eval_element(&self, index: usize) -> Result<Self::ElementType, Self::ErrorType>;
}

pub(crate) struct RandomListExpressionWrapper<Expr: RandomListExpression>(pub(crate) Expr);

impl<Expr: RandomListExpression> Expression for RandomListExpressionWrapper<Expr> {
    type ValueType = Vec<Expr::ElementType>;
    type ErrorType = Expr::ErrorType;

    fn terms(&self) -> Terms {
        self.0.terms()
    }

    fn eval(&self) -> Result<Self::ValueType, Self::ErrorType> {
        let mut result =  Self::ValueType::new();

        for i in 0..self.0.len() {
            result.push(self.0.eval_element(i)?);
        }

        Ok(result)
    }
}

pub trait SequentialListExpression {
    type ElementType;
    type ErrorType;

    fn terms(&self) -> Terms;
    fn eval_next(&self, prev: &Vec<Self::ElementType>) -> Result<Option<Self::ElementType>, Self::ErrorType>;
}

pub(crate) struct SequentialListExpressionWrapper<Expr: SequentialListExpression>(pub(crate) Expr);

impl<Expr: SequentialListExpression> Expression for SequentialListExpressionWrapper<Expr> {
    type ValueType = Vec<Expr::ElementType>;
    type ErrorType = Expr::ErrorType;

    fn terms(&self) -> Terms {
        self.0.terms()
    }

    fn eval(&self) -> Result<Self::ValueType, Self::ErrorType> {
        let mut result =  Self::ValueType::new();

        let mut maybe_elem = self.0.eval_next(&result)?;

        while let Some(elem) = maybe_elem {
            result.push(elem);
            maybe_elem = self.0.eval_next(&result)?;
        }

        Ok(result)
    }
}
