use crate::error::*;
use crate::generator::*;
use worm_cell::AtomicWormCellReader;
use std::ops::Deref;
use std::marker::PhantomData;

#[derive(Clone)]
pub struct Term<'a, ValueType, ImplType> {
    pub(crate) result: AtomicWormCellReader<ValueType>,
    pub(crate) implementation: ImplType,
    phantom: PhantomData<&'a ValueType>
}

impl<'a, ValueType, ImplType> Term<'a, ValueType, ImplType> {
    pub fn new(result: AtomicWormCellReader<ValueType>, implementation: ImplType) -> Self {
        Self {
            result,
            implementation,
            phantom: PhantomData
        }
    }
}

#[derive(Clone)]
pub struct ListTerm<'a,  ElementType, ImplType> {
    pub(crate) result: AtomicWormCellReader<Vec<ElementType>>,
    pub(crate) implementation: ImplType,
    phantom: PhantomData<&'a ElementType>
}

impl<'a, ElementType, ImplType> ListTerm<'a, ElementType, ImplType> {
    pub fn new(result: AtomicWormCellReader<Vec<ElementType>>, implementation: ImplType) -> Self {
        Self {
            result,
            implementation,
            phantom: PhantomData
        }
    }
}

pub trait TermLike<'a, ValueType, ImplType> {
    fn try_get(&'a self) -> Result<&'a ValueType, EngineError>;
    fn get_implementation(&'a self) -> &'a ImplType;
}

impl<'a, ValueType, ImplType> TermLike<'a, ValueType, ImplType> for Term<'a, ValueType, ImplType> {
    fn try_get(&'a self) -> Result<&'a ValueType, EngineError> {
        Ok(self.result.try_get()?)
    }
    
    fn get_implementation(&'a self) -> &'a ImplType {
        &self.implementation
    }
}

impl<'a, ElementType, ImplType> TermLike<'a, Vec<ElementType>, ImplType> for ListTerm<'a, ElementType, ImplType> {
    fn try_get(&'a self) -> Result<&'a Vec<ElementType>, EngineError> {
        Ok(self.result.try_get()?)
    }
    
    fn get_implementation(&'a self) -> &'a ImplType {
        &self.implementation
    }
}

impl<'a, ElementType, ImplType> ListTerm<'a, ElementType, ImplType> {
    pub fn iter(&'a self) -> std::slice::Iter<'a, ElementType> {
        self.result.try_get().unwrap().iter()
    }
}

impl<'a, ValueType, ImplType> Deref for Term<'a, ValueType, ImplType> {
    type Target = ValueType;

    fn deref(&self) -> &Self::Target {
        self.try_get().unwrap()
    }
}

pub trait TermSet {
    type TermImpl;

    fn new() -> Self;
    fn add<'a, ValueType, TermType>(self, term: &'a TermType) -> Self
        where TermType: TermLike<'a, ValueType, Self::TermImpl>;
}

pub trait Engine<'a> {
    type ErrorType: std::error::Error;
    type UpstreamSet: TermSet;
    type TermImpl: 'a;

    fn eval<'t, ValueType, TermType>(&mut self, term: &'t TermType) -> Result<&'t ValueType, ExpressionError<Self::ErrorType>>
    where TermType: TermLike<'t, ValueType, Self::TermImpl>,
    'a: 't {
        self.eval_impl(&term.get_implementation())?;
        term.try_get().map_err(|e| ExpressionError::<Self::ErrorType>::Engine(e))
    }

    fn eval_impl(&self, term: &Self::TermImpl) -> Result<(), ExpressionError<Self::ErrorType>>;

    fn scalar<'t, ValueType, FnType>(&mut self, func: FnType, upstream: Self::UpstreamSet) -> Term<'t, ValueType, Self::TermImpl>
    where
        ValueType: 'a,
        FnType: Fn() -> ValueType + 'a,
        'a: 't;
        
    fn scalar_err<ValueType, ErrorType, FnType>(&mut self, func: FnType, upstream: Self::UpstreamSet) -> Term<ValueType, Self::TermImpl>
    where
        ValueType: 'a,
        FnType: Fn() -> Result<ValueType, ErrorType> + 'a,
        Self::ErrorType: From<ErrorType>;

    fn list<ElementType, FnType>(&mut self, func: FnType, upstream: Self::UpstreamSet) -> ListTerm<ElementType, Self::TermImpl>
    where
        ElementType: 'a,
        FnType: Fn() -> Vec<ElementType> + 'a;


    fn list_err<ElementType, ErrorType, FnType>(&mut self, func: FnType, upstream: Self::UpstreamSet) -> ListTerm<ElementType, Self::TermImpl>
    where
        ElementType: 'a,
        FnType: Fn() -> Result<Vec<ElementType>, ErrorType> + 'a,
        Self::ErrorType: From<ErrorType>;

    fn generator<'t, ElementType, GeneratorType>(&mut self, generator: GeneratorType, upstream: Self::UpstreamSet) -> ListTerm<'t, ElementType, Self::TermImpl>
    where
        ElementType: 'a,
        GeneratorType: Generator<Item=ElementType> + 'a,
        'a: 't;

    fn generator_err<ElementType, ErrorType, GeneratorType>(&mut self, generator: GeneratorType, upstream: Self::UpstreamSet) -> ListTerm<ElementType, Self::TermImpl>
    where
        ElementType: 'a,
        GeneratorType: Generator<Item=Result<ElementType, ErrorType>> + 'a,
        Self::ErrorType: From<ErrorType>;

    fn map<'t, SetupType, ElementType, GeneratorType, MapFnType>(&mut self, generator: GeneratorType, map_fn: MapFnType, upstream: Self::UpstreamSet) -> ListTerm<'t, ElementType, Self::TermImpl>
    where
        ElementType: 'a,
        GeneratorType: Generator<Item=SetupType> + 'a,
        MapFnType: Fn(SetupType) -> ElementType + 'a,
        'a: 't;

    fn map_err<SetupType, ElementType, ErrorType, GeneratorType, MapFnType>(&mut self, generator: GeneratorType, map_fn: MapFnType, upstream: Self::UpstreamSet) -> ListTerm<ElementType, Self::TermImpl>
    where
        ElementType: 'a,
        GeneratorType: Generator<Item=Result<SetupType, ErrorType>> + 'a,
        MapFnType: Fn(SetupType) -> Result<ElementType, ErrorType> + 'a,
        Self::ErrorType: From<ErrorType>;

    fn upstream(&self) -> Self::UpstreamSet {
        Self::UpstreamSet::new()
    }
}
