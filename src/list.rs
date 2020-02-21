use crate::expression::*;
use rayon::prelude::*;

pub trait ListExpression {
    type ElementType;
    type ErrorType;

    fn terms(&self) -> Terms;
    fn eval(&self) -> Result<Vec<Self::ElementType>, Self::ErrorType>;
}

pub(crate) struct ListExpressionWrapper<Expr: ListExpression>(pub(crate) Expr);

impl<Expr> Expression for ListExpressionWrapper<Expr>
where Expr: ListExpression
{
    type ValueType = Vec<Expr::ElementType>;
    type ErrorType = Expr::ErrorType;

    fn terms(&self) -> Terms {
        self.0.terms()
    }

    fn eval(&self) -> Result<Self::ValueType, Self::ErrorType> {
        self.0.eval()
    }
}

pub trait RandomListExpression {
    type ElementType;
    type ErrorType;
    type ElementSetup;

    fn terms(&self) -> Terms;
    fn len(&self) -> usize;
    fn setup_element(&self, index: usize) ->
        Result<Self::ElementSetup,Self::ErrorType>;
    fn eval_element(&self, setup: &Self::ElementSetup) ->
        Result<Self::ElementType, Self::ErrorType>;
}

pub(crate) struct RandomListExpressionWrapper<Expr: RandomListExpression>(
    pub(crate) Expr);

impl<Expr> ListExpression for RandomListExpressionWrapper<Expr>
where
    Expr: RandomListExpression + Sync,
    Expr::ElementSetup: Sync,
    Expr::ElementType: Send,
    Expr::ErrorType: Send
{
    type ElementType = Expr::ElementType;
    type ErrorType = Expr::ErrorType;

    fn terms(&self) -> Terms {
        self.0.terms()
    }

    fn eval(&self) -> Result<Vec<Self::ElementType>, Self::ErrorType> {
        let indices = 0..self.0.len();

        let setups: Result<Vec<Expr::ElementSetup>, Self::ErrorType> = indices
            .into_iter()
            .map(|i| self.0.setup_element(i))
            .collect();
        let setups = setups?;
        
        setups
            .into_par_iter()
            .map(|s| self.0.eval_element(s))
            .collect()
    }
}

pub trait SequentialListExpression {
    type ElementType;
    type ErrorType;

    fn terms(&self) -> Terms;
    fn eval_next(&self, prev: &Vec<Self::ElementType>) -> Result<Option<Self::ElementType>, Self::ErrorType>;
}

pub(crate) struct SequentialListExpressionWrapper<Expr: SequentialListExpression>(
    pub(crate) Expr);

impl<Expr> Expression for SequentialListExpressionWrapper<Expr>
where Expr: SequentialListExpression
{
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

