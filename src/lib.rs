
#[macro_use]
extern crate failure;
extern crate worm_cell;
use std::ops::Mul;
use worm_cell::{WormCell, WormCellReader};

#[derive(Fail, Debug)]
pub enum BuilderError {
    #[fail(display = "Tried to get() a result that has not been calculated {}", _0)]
    GetNotCalculated(#[fail(cause)] worm_cell::WormCellError),
    #[fail(display = "Tried to calculate a result that has already been calculated {}", _0)]
    DoubleCalc(#[fail(cause)] worm_cell::WormCellError),
}

pub type BuilderResult<T> = Result<T, BuilderError>;

#[derive(Debug, Copy, Clone)]
pub struct Term(usize);

pub struct TypedTerm<ResultType> {
    term: Term,
    result: WormCellReader<ResultType>
}

impl<'a, ResultType> TypedTerm<ResultType> {
    fn get(&self) -> BuilderResult<&ResultType> {
        self.result.get().map_err(BuilderError::GetNotCalculated)
    }

    fn term(&self) -> Term {
        self.term
    }
}

pub type Terms = Vec<Term>;

pub trait Expression<ValueType> {
    fn terms(&self) -> Terms;
    fn eval(&self) -> BuilderResult<ValueType>;
}

struct TypedExpressionResult<ResultType, Expr: Expression<ResultType>> {
    expr: Expr,
    result: WormCell<ResultType>
}

trait ExpressionResult {
    fn evaluated(&self) -> bool;
    fn terms(&self) -> Terms;
    fn eval(&mut self) -> BuilderResult<()>;
}

impl<ResultType, Expr: Expression<ResultType>> ExpressionResult for TypedExpressionResult<ResultType, Expr> {
    fn terms(&self) -> Terms {
        self.expr.terms()
    }

    fn eval(&mut self) -> BuilderResult<()> {
        self.result.set(self.expr.eval()?).map_err(BuilderError::DoubleCalc)
    }

    fn evaluated(&self) -> bool {
        self.result.is_set()
    }
 }

struct Builder<'a> {
     terms: Vec<Box<dyn ExpressionResult + 'a>>
}

impl<'a> Builder<'a> {

    fn new() -> Builder<'a> {
        Builder { terms: Vec::new() }
    }
    
    fn eval_term(&mut self, term: Term) {
        if !self.terms[term.0].evaluated() {
            for subterm in self.terms[term.0].terms() {
                self.eval_term(subterm);
            }

            self.terms[term.0].eval();
        }
    }

    fn eval<'b, ValueType>(&mut self, term: &'b TypedTerm<ValueType>) -> BuilderResult<&'b ValueType> {
        self.eval_term(term.term());
        term.get()
    }
    
    fn term<ValueType: 'a, Expr: Expression<ValueType> + 'a>(&mut self, expr: Expr) -> TypedTerm<ValueType> {
        let expr_result = Box::new(TypedExpressionResult {
            expr: expr,
            result: WormCell::<ValueType>::new()
        });
        
        let term_result = expr_result.result.reader();
        
        self.terms.push(expr_result);

        TypedTerm { term: Term(self.terms.len() - 1),
                    result: term_result}
    }
}

struct Value<ValueType: Clone> {
    val : ValueType
}

impl<ValueType: Clone> Expression<ValueType> for Value<ValueType> {
    fn eval(&self) -> BuilderResult<ValueType> {
        Ok(self.val.clone())
    }

    fn terms(&self) -> Vec<Term> { Vec::<Term>::new() }
}

struct Multiply<ValueType: Mul + Copy> {
    operand: TypedTerm<ValueType>,
    factor: ValueType
}

impl<ValueType: Mul + Copy> Expression<ValueType::Output> for Multiply<ValueType> {
    fn terms(&self) -> Terms {
        vec!(self.operand.term())
    }
    
    fn eval(&self) -> BuilderResult<ValueType::Output> {
        let result = *self.operand.get()? * self.factor;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_term() {
        let mut builder = Builder::new();
        
        let term1 = builder.term(Value::<i32>{ val: 5 });
        let term2 = builder.term(Multiply::<i32>{ operand: term1, factor: 2 });
        assert_eq!(*builder.eval(&term2).unwrap(), 10);

        println!("OK");
    }
}

