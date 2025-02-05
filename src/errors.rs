use std::fmt;

#[derive(Debug)]
pub enum SolResolverError {
    InvalidDidFormat,
    InvalidSolanaAddress,
}

impl fmt::Display for SolResolverError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SolResolverError::InvalidDidFormat => write!(f, "Invalid DID format"),
            SolResolverError::InvalidSolanaAddress =>
                write!(f, "Invalid DID format - Missing Solana address"),
        }
    }
}
