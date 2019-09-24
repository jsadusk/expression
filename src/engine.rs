use crate::error::*;
use crate::expression::*;
use crate::list::*;

pub struct Engine<'a, ErrorType> {
    terms: Vec<Box<dyn ExpressionCache<ErrorType> + 'a>>,
}

impl<'a, ErrorType> Engine<'a, ErrorType>
where ErrorType: 'a + std::error::Error + 'static
{
    pub fn new() -> Engine<'a, ErrorType> {
        Engine { terms: Vec::new() }
    }

    fn eval_term(&mut self, term: Term) -> Result<(), ExpressionError<ErrorType>> {
        if !self.terms[term.0].evaluated() {
            for subterm in self.terms[term.0].terms() {
                self.eval_term(subterm)?;
            }

            self.terms[term.0].eval()
        } else {
            Ok(())
        }
    }

    pub fn eval<'b, ValueType>(&mut self, term: &'b TypedTerm<ValueType>) -> Result<&'b ValueType, ExpressionError<ErrorType>> {
        self.eval_term(term.term())?;
        term.get().map_err(|e| ExpressionError::<ErrorType>::Engine(e))
    }

    pub fn term<Expr>(&mut self, expr: Expr) -> TypedTerm<Expr::ValueType>
    where
        Expr: Expression + 'a,
        ErrorType: From<Expr::ErrorType>
    {
        let expr_cache = Box::new(TypedExpressionCache::new(expr));
        let term_result = expr_cache.result.reader();

        self.terms.push(expr_cache);

        TypedTerm { term: Term(self.terms.len() - 1),
                    result: term_result}
    }

    pub fn random_list_term<ListExpr>(&mut self, expr: ListExpr) ->
        TypedTerm<Vec::<ListExpr::ElementType>>
    where
        ListExpr: RandomListExpression + 'a,
        ErrorType: From<ListExpr::ErrorType>
    {
        self.term(RandomListExpressionWrapper::<ListExpr>(expr))
    }

    pub fn sequential_list_term<ListExpr>(&mut self, expr: ListExpr) ->
        TypedTerm<Vec<ListExpr::ElementType>>
    where
        ListExpr: SequentialListExpression + 'a,
        ErrorType: From<ListExpr::ErrorType>
    {
        self.term(SequentialListExpressionWrapper::<ListExpr>(expr))
    }
}
