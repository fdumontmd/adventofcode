use miette::Diagnostic;
use nom_supreme::error::{BaseErrorKind, ErrorTree};
use thiserror::Error;

use crate::parser::Span;

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
impl AocError {
    pub(crate) fn from_error_tree(input: &'static str, e: ErrorTree<Span>) -> Self {
        match e {
            nom_supreme::error::GenericErrorTree::Base { location, kind } => {
                let offset = location.location_offset().into();
                AocError::BadInput {
                    src: input,
                    bad_bits: miette::SourceSpan::new(offset, 0.into()),
                    kind,
                }
            }
            nom_supreme::error::GenericErrorTree::Stack { base, .. } => {
                AocError::from_error_tree(input, *base)
            }
            nom_supreme::error::GenericErrorTree::Alt(_) => todo!(),
        }
    }
}
