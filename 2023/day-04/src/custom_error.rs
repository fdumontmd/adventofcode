use miette::Diagnostic;
use nom_supreme::error::BaseErrorKind;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum AocError {
    #[error(transparent)]
    #[diagnostic(code(aoc::io_error))]
    IoError(#[from] std::io::Error),
    #[error("bad input")]
    BadInput {
        #[source_code]
        src: &'static str,

        #[label("{kind}")]
        bad_bits: miette::SourceSpan,

        kind: BaseErrorKind<&'static str, Box<dyn std::error::Error + Send + Sync>>,
    },
}
