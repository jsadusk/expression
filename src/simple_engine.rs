use crate::error::*;
use crate::engine::*;
use crate::generator::*;
use worm_cell::{AtomicWormCell, AtomicWormCellReader};
use std::sync::Arc;

pub(crate) trait Expression<EvalErrorType>
{
    fn evaluated(&self) -> bool;
    fn upstream(&self) -> &IndexSet;
    fn eval(&self) -> Result<(), EvalErrorType>;
}


pub struct SimpleEngine<'a, ErrorType> {
    terms: Vec<Box<dyn Expression<ErrorType> + 'a>>,
}

#[derive(Clone)]
pub struct TermIndex(usize);

pub struct IndexSet(Vec<TermIndex>);

impl TermSet for IndexSet {
    type TermImpl = TermIndex;

    fn new() -> IndexSet {
        IndexSet(Vec::new())
    }

    fn add<'a, ValueType, TermType>(mut self, term: &'a TermType) -> Self
    where TermType: TermLike<'a, ValueType, TermIndex> {
        self.0.push(term.get_implementation().clone());
        self
    }
}

struct SimpleExpression<ValueType, FnType>
{
    result: Arc<AtomicWormCell<ValueType>>,
    func: FnType,
    upstream: IndexSet
}

impl<ValueType, FnType> SimpleExpression<ValueType, FnType>
    where FnType: Fn() -> ValueType
{
    fn new(func: FnType, upstream: IndexSet) -> Self {
        SimpleExpression {
            result: Arc::new(AtomicWormCell::new()),
            func: func,
            upstream: upstream
        }
    }
}

impl<ValueType, FnType, EvalErrorType> Expression<EvalErrorType> for SimpleExpression<ValueType, FnType>
where
    FnType: Fn() -> ValueType,
    EvalErrorType: std::error::Error + 'static
{
    fn evaluated(&self) -> bool {
        self.result.is_set()
    }

    fn upstream(&self) -> &IndexSet {
        &self.upstream
    }

    fn eval(&self) -> Result<(), EvalErrorType> {
        self.result.set((self.func)());
        Ok(())
    }
}

struct SimpleErrExpression<ValueType, FnType>
{
    result: Arc<AtomicWormCell<ValueType>>,
    func: FnType,
    upstream: IndexSet
}

impl<ValueType, ErrorType, FnType> SimpleErrExpression<ValueType, FnType>
    where FnType: Fn() -> Result<ValueType, ErrorType>
{
    fn new(func: FnType, upstream: IndexSet) -> Self {
        SimpleErrExpression {
            result: Arc::new(AtomicWormCell::new()),
            func: func,
            upstream: upstream
        }
    }
}

impl<ValueType, ErrorType, FnType, EvalErrorType> Expression<EvalErrorType> for SimpleErrExpression<ValueType, FnType>
where
    FnType: Fn() -> Result<ValueType, ErrorType>,
    EvalErrorType: std::error::Error + From<ErrorType>
{
    fn evaluated(&self) -> bool {
        self.result.is_set()
    }

    fn upstream(&self) -> &IndexSet {
        &self.upstream
    }

    fn eval(&self) -> Result<(), EvalErrorType> {
        self.result.set((self.func)()?);
        Ok(())
    }
}

impl<'a, ErrorType> SimpleEngine<'a, ErrorType>
where ErrorType: 'a + std::error::Error + 'static
{
    pub fn new() -> SimpleEngine<'a, ErrorType> {
        SimpleEngine { terms: Vec::new() }
    }
}

impl<'a, ET> Engine<'a> for SimpleEngine<'a, ET>
where ET: 'a + std::error::Error + 'static
{
    type ErrorType = ET;
    type UpstreamSet = IndexSet;
    type TermImpl = TermIndex;
    

    fn eval_impl(&self, term: &TermIndex) -> Result<(), ExpressionError<Self::ErrorType>> {
        if !self.terms[term.0].evaluated() {
            for subterm in &self.terms[term.0].upstream().0 {
                self.eval_impl(&subterm)?;
            }

            self.terms[term.0].eval().map_err(ExpressionError::Eval)
        } else {
            Ok(())
        }
    }

    fn scalar<'t, ValueType, FnType>(&mut self, func: FnType, upstream: Self::UpstreamSet) -> Term<'t, ValueType, Self::TermImpl>
    where
        ValueType: 'a,
        FnType: Fn() -> ValueType + 'a,
        'a: 't {

        let expr = Box::new(SimpleExpression::new(func, upstream));
        let term_result = AtomicWormCellReader::new(expr.result.clone());
        self.terms.push(expr);
        Term::new(term_result, TermIndex(self.terms.len() - 1))
    }

    fn scalar_err<ValueType, ErrorType, FnType>(&mut self, func: FnType, upstream: Self::UpstreamSet) -> Term<ValueType, Self::TermImpl>
    where
        ValueType: 'a,
        FnType: Fn() -> Result<ValueType, ErrorType> + 'a,
        Self::ErrorType: From<ErrorType> {

        let expr = Box::new(SimpleErrExpression::new(func, upstream));
        let term_result = AtomicWormCellReader::new(expr.result.clone());
        self.terms.push(expr);
        Term::new(term_result, TermIndex(self.terms.len() - 1))
    }

    fn list<ElementType, FnType>(&mut self, func: FnType, upstream: Self::UpstreamSet) -> ListTerm<ElementType, Self::TermImpl>
    where
        ElementType: 'a,
        FnType: Fn() -> Vec<ElementType> + 'a {

        let expr = Box::new(SimpleExpression::new(func, upstream));
        let term_result = AtomicWormCellReader::new(expr.result.clone());
        self.terms.push(expr);
        ListTerm::new(term_result, TermIndex(self.terms.len() - 1))
    }

    fn list_err<ElementType, ErrorType, FnType>(&mut self, func: FnType, upstream: Self::UpstreamSet) -> ListTerm<ElementType, Self::TermImpl>
    where
        ElementType: 'a,
        FnType: Fn() -> Result<Vec<ElementType>, ErrorType> + 'a,
        Self::ErrorType: From<ErrorType> {

        let expr = Box::new(SimpleErrExpression::new(func, upstream));
        let term_result = AtomicWormCellReader::new(expr.result.clone());
        self.terms.push(expr);
        ListTerm::new(term_result, TermIndex(self.terms.len() - 1))
    }

    fn generator<'t, ElementType, GeneratorType>(&mut self, generator: GeneratorType, upstream: Self::UpstreamSet) -> ListTerm<'t, ElementType, Self::TermImpl>
    where
        ElementType: 'a,
        GeneratorType: Generator<Item=ElementType> + 'a,
        'a: 't {

        let expr = Box::new(SimpleExpression::new(move || generator.iter().collect(), upstream));
        let term_result = AtomicWormCellReader::new(expr.result.clone());
        self.terms.push(expr);
        ListTerm::new(term_result, TermIndex(self.terms.len() - 1))
    }

    fn generator_err<ElementType, ErrorType, GeneratorType>(&mut self, generator: GeneratorType, upstream: Self::UpstreamSet) -> ListTerm<ElementType, Self::TermImpl>
    where
        ElementType: 'a,
        GeneratorType: Generator<Item=Result<ElementType, ErrorType>> + 'a,
        Self::ErrorType: From<ErrorType> {

        let expr = Box::new(SimpleErrExpression::new(move || generator.iter().collect(), upstream));
        let term_result = AtomicWormCellReader::new(expr.result.clone());
        self.terms.push(expr);
        ListTerm::new(term_result, TermIndex(self.terms.len() - 1))
    }

    fn map<'t, SetupType, ElementType, GeneratorType, MapFnType>(&mut self, generator: GeneratorType, map_fn: MapFnType, upstream: Self::UpstreamSet) -> ListTerm<'t, ElementType, Self::TermImpl>
    where
        ElementType: 'a,
        GeneratorType: Generator<Item=SetupType> + 'a,
        MapFnType: Fn(SetupType) -> ElementType + 'a,
        'a: 't {

        let expr = Box::new(SimpleExpression::new(move || generator.iter().map(|s| map_fn(s)).collect(), upstream));
        let term_result = AtomicWormCellReader::new(expr.result.clone());
        self.terms.push(expr);
        ListTerm::new(term_result, TermIndex(self.terms.len() - 1))
    }

    fn map_err<SetupType, ElementType, ErrorType, GeneratorType, MapFnType>(&mut self, generator: GeneratorType, map_fn: MapFnType, upstream: Self::UpstreamSet) -> ListTerm<ElementType, Self::TermImpl>
    where
        ElementType: 'a,
        GeneratorType: Generator<Item=Result<SetupType, ErrorType>> + 'a,
        MapFnType: Fn(SetupType) -> Result<ElementType, ErrorType> + 'a,
        Self::ErrorType: From<ErrorType> {

        let expr = Box::new(SimpleErrExpression::new(move || generator.iter().map(|e| map_fn(e?)).collect(), upstream));
        let term_result = AtomicWormCellReader::new(expr.result.clone());
        self.terms.push(expr);
        ListTerm::new(term_result, TermIndex(self.terms.len() - 1))
    }
}
