use worm_cell::WormCell;

use crate::error::*;
use crate::expression::*;

pub struct Engine<'a> {
     terms: Vec<Box<dyn ExpressionCache + 'a>>
}

impl<'a> Engine<'a> {

    pub fn new() -> Engine<'a> {
        Engine { terms: Vec::new() }
    }
    
    fn eval_term(&mut self, term: Term) -> ExpressionResult<()> {
        if !self.terms[term.0].evaluated() {
            for subterm in self.terms[term.0].terms() {
                self.eval_term(subterm)?;
            }

            self.terms[term.0].eval()
        } else { Ok(()) }
        
    }

    pub fn eval<'b, ValueType>(&mut self, term: &'b TypedTerm<ValueType>) -> ExpressionResult<&'b ValueType> {
        self.eval_term(term.term())?;
        term.get()
    }
    
    pub fn term<ValueType: 'a, Expr: Expression<ValueType> + 'a>(&mut self, expr: Expr) -> TypedTerm<ValueType> {
        let expr_cache = Box::new(TypedExpressionCache {
            expr: expr,
            result: WormCell::<ValueType>::new()
        });
        
        let term_result = expr_cache.result.reader();
        
        self.terms.push(expr_cache);

        TypedTerm { term: Term(self.terms.len() - 1),
                    result: term_result}
    }
}
