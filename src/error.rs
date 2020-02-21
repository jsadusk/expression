use std::error;
use std::fmt;
use worm_cell::error::WormCellError;

#[derive(Debug)]
pub enum EngineError {
    GetNotCalculated,
    DoubleCalc
}

impl error::Error for EngineError {}

impl fmt::Display for EngineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EngineError::GetNotCalculated => write!(f, "Tried to get() a result that has not been calculated"),
            EngineError::DoubleCalc => write!(f, "Tried to calculate a result that has already been calculated")
        }
    }
}

#[derive(Debug)]
pub enum ExpressionError<EvalError>
where EvalError: error::Error + 'static {
    Engine(EngineError),
    Eval(EvalError)
}

impl<EvalError: error::Error + 'static > error::Error for ExpressionError<EvalError> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            ExpressionError::<EvalError>::Engine(engine) => Some(engine),
            ExpressionError::<EvalError>::Eval(eval) => Some(eval)
        }
    }
}

impl<EvalError: error::Error + 'static > fmt::Display for ExpressionError<EvalError> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Engine(engine) => write!(f, "Engine error: {}", engine),
            Self::Eval(eval) => write!(f, "Eval error: {}", eval)
        }
    }
}

impl<EvalError: error::Error + 'static> From<EngineError> for ExpressionError<EvalError> {
    fn from(orig: EngineError) -> Self {
        Self::Engine(orig)
    }
}

impl<EvalError: error::Error + 'static> From<WormCellError> for ExpressionError<EvalError> {
    fn from(orig: WormCellError) -> Self {
        Self::Engine(orig.into())
    }
}

#[derive(Debug)]
pub enum OpError {
    NeverError
}

impl error::Error for OpError {}

impl fmt::Display for OpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Should never error")
    }
}

impl From<()> for OpError {
    fn from(_empty: ()) -> OpError {
        OpError::NeverError
    }
}

impl From<WormCellError> for EngineError {
    fn from(cause: WormCellError) -> EngineError {
        match cause {
            WormCellError::ReadNotSet => EngineError::GetNotCalculated,
            WormCellError::DoubleSet => EngineError::DoubleCalc
        }
    }
}

    
pub type EngineResult<T> = Result<T, EngineError>;
pub type ExpressionResult<T, E> = Result<T, ExpressionError<E>>;
