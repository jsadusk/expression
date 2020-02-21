use crate::error::*;
use crate::engine::*;
use crate::expression::*;
use crate::list::*;
use worm_cell::AtomicWormCellReader;

pub struct SimpleEngine<'a, ErrorType> {
    terms: Vec<Box<dyn ExpressionCache<ErrorType> + 'a>>,
}


impl<'a, ErrorType> SimpleEngine<'a, ErrorType>
where ErrorType: 'a + std::error::Error + 'static
{
    pub fn new() -> SimpleEngine<'a, ErrorType> {
        SimpleEngine { terms: Vec::new() }
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
}

impl<'a, ET> Engine<'a> for SimpleEngine<'a, ET>
where ET: 'a + std::error::Error + 'static
{
    type ErrorType = ET;
    
    fn eval<'b, TermType>(&mut self, term: &'b TermType) -> Result<&'b TermType::ValueType, ExpressionError<Self::ErrorType>>
    where TermType: TypedTerm {
        self.eval_term(term.term())?;
        term.try_get().map_err(|e| ExpressionError::<Self::ErrorType>::Engine(e))
    }

    fn term<Expr>(&mut self, expr: Expr) -> TypedTermImpl<Expr::ValueType>
    where
        Expr: Expression + 'a,
        Self::ErrorType: From<Expr::ErrorType>
    {
        let expr_cache = Box::new(TypedExpressionCache::new(expr));
        let term_result = AtomicWormCellReader::new(expr_cache.result.clone());

        self.terms.push(expr_cache);

        TypedTermImpl { term: Term(self.terms.len() - 1),
                        result: term_result}
    }

    fn list_term<ListExpr>(&mut self, expr: ListExpr) ->
        ListTermImpl<ListExpr::ElementType>
    where
        ListExpr: ListExpression + 'a,
        Self::ErrorType: From<ListExpr::ErrorType>
    {
        ListTermImpl::<ListExpr::ElementType>(self.term(ListExpressionWrapper::<ListExpr>(expr)))
    }

    fn random_list_term<ListExpr>(&mut self, expr: ListExpr) ->
        ListTermImpl<ListExpr::ElementType>
    where
        ListExpr: RandomListExpression + Sync + 'a,
        ListExpr::ElementSetup: Sync,
        ListExpr::ElementType: Send,
        ListExpr::ErrorType: Send,
        Self::ErrorType: From<ListExpr::ErrorType>
    {
        self.list_term(RandomListExpressionWrapper::<ListExpr>(expr))
    }

    fn sequential_list_term<ListExpr>(&mut self, expr: ListExpr) ->
        TypedTermImpl<Vec<ListExpr::ElementType>>
    where
        ListExpr: SequentialListExpression + 'a,
        Self::ErrorType: From<ListExpr::ErrorType>
    {
        self.term(SequentialListExpressionWrapper::<ListExpr>(expr))
    }
}
