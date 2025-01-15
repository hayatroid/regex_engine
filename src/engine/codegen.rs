use thiserror::Error;

#[derive(Error, Debug)]
pub enum CodeGenError {
    #[error("CodeGenError: PCOverFlow")]
    PCOverFlow,
    #[error("CodeGenError: FailStar")]
    FailStar,
    #[error("CodeGenError: FailOr")]
    FailOr,
    #[error("CodeGenError: FailQuestion")]
    FailQuestion,
}
