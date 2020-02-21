use crate::error::*;
use crate::expression::*;
use crate::list::*;

pub trait Engine<'a> {
    type ErrorType: std::error::Error;

    fn eval<'b, TermType>(&mut self, term: &'b TermType) -> Result<&'b TermType::ValueType, ExpressionError<Self::ErrorType>>
    where TermType: TypedTerm;

    fn term<Expr>(&mut self, expr: Expr) -> TypedTermImpl<Expr::ValueType>
    where
        Expr: Expression + 'a,
        Self::ErrorType: From<Expr::ErrorType>;

    fn list_term<ListExpr>(&mut self, expr: ListExpr) ->
        ListTermImpl<ListExpr::ElementType>
    where
        ListExpr: ListExpression + 'a,
        Self::ErrorType: From<ListExpr::ErrorType>;

    fn random_list_term<ListExpr>(&mut self, expr: ListExpr) ->
        ListTermImpl<ListExpr::ElementType>
    where
        ListExpr: RandomListExpression + Sync + 'a,
        ListExpr::ElementSetup: Sync,
        ListExpr::ElementType: Send,
        ListExpr::ErrorType: Send,
        Self::ErrorType: From<ListExpr::ErrorType> + Send;

    fn sequential_list_term<ListExpr>(&mut self, expr: ListExpr) ->
        TypedTermImpl<Vec<ListExpr::ElementType>>
    where
        ListExpr: SequentialListExpression + 'a,
        Self::ErrorType: From<ListExpr::ErrorType>;
}
