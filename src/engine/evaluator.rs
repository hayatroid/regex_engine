use thiserror::Error;

#[derive(Error, Debug)]
pub enum EvalError {
    #[error("CodeGenError: PCOverFlow")]
    PCOverFlow,
    #[error("CodeGenError: SPOverFlow")]
    SPOverFlow,
    #[error("CodeGenError: InvalidPC")]
    InvalidPC,
    #[error("CodeGenError: InvalidContext")]
    InvalidContext,
}
