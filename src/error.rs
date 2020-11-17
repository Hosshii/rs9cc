use crate::ast::Error as AstError;
use crate::token::Error as TokenError;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
pub enum Error {
    AstError(AstError),
    TokenError(TokenError),
}

impl From<AstError> for Error {
    fn from(error: AstError) -> Self {
        Error::AstError(error)
    }
}

// このエラー必要ないかも
impl From<TokenError> for Error {
    fn from(error: TokenError) -> Self {
        Error::TokenError(error)
    }
}
