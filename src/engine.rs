use worm_cell::WormCell;

use crate::error::*;
use crate::expression::*;
use crate::list::*;

use std::marker::PhantomData;

pub struct Engine<'a, ErrorType> {
    terms: Vec<Box<dyn ExpressionCache<ErrorType> + 'a>>,
    _e: PhantomData<ErrorType>
}

impl<'a, ErrorType: 'a + std::error::Error + 'static> Engine<'a, ErrorType> {
    pub fn new() -> Engine<'a, ErrorType> {
        Engine { terms: Vec::new(), _e: PhantomData::<ErrorType>::default() }
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

    pub fn term<ValueType: 'a, Expr: Expression<ValueType, ErrorType> + 'a>(&mut self, expr: Expr) -> TypedTerm<ValueType> {
        let expr_cache = Box::new(TypedExpressionCache {
            expr: expr,
            result: WormCell::<ValueType>::new(),
            _e: PhantomData::<ErrorType>::default()
        });

        let term_result = expr_cache.result.reader();

        self.terms.push(expr_cache);

        TypedTerm { term: Term(self.terms.len() - 1),
                    result: term_result}
    }

    pub fn random_list_term<ElementType: 'a, ListExpr: RandomListExpression<ElementType, ErrorType> + 'a>(&mut self, expr: ListExpr) -> TypedTerm<Vec<ElementType>> {
        self.term(RandomListExpressionWrapper::<ElementType, ErrorType, ListExpr>(expr, PhantomData::<ElementType>::default(), PhantomData::<ErrorType>::default()))
    }

    pub fn sequential_list_term<ElementType: 'a, ListExpr: SequentialListExpression<ElementType, ErrorType> + 'a>(&mut self, expr: ListExpr) -> TypedTerm<Vec<ElementType>> {
        self.term(SequentialListExpressionWrapper::<ElementType, ErrorType, ListExpr>(expr, PhantomData::<ElementType>::default(), PhantomData::<ErrorType>::default()))
    }
}
