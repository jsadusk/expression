#[derive(Fail, Debug)]
pub enum ExpressionError {
    #[fail(display = "Tried to get() a result that has not been calculated {}", _0)]
    GetNotCalculated(#[fail(cause)] worm_cell::WormCellError),
    #[fail(display = "Tried to calculate a result that has already been calculated {}", _0)]
    DoubleCalc(#[fail(cause)] worm_cell::WormCellError),
}

pub type ExpressionResult<T> = Result<T, ExpressionError>;
