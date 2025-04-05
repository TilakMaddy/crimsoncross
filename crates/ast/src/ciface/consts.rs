use crate::ctraits::Validate;

/// Const (Literal)
pub enum CConst {
    /// Allowed: 0-9
    /// Constraints: -2^256 to 2^256 - 1
    SignedInt { sign: CConstSign, value: String },
    /// Must not contain the initial `0x` part
    /// Allowed: 0-9 a-f A-F
    /// Constraints: -2^256 to 2^256 - 1
    HexDigit(String),
}

pub enum CConstSign {
    Plus,
    Minus,
}

// Semantic analysis
// Should be done in multiple passes/steps
// Errors should return a format that can be displayed
// That means each node in CIFace must contain a SourceLocation
// - source-unit (id)
// - byte offset
// - char offset
// - help message
// (ariadne crate requirements)
// rename to ciface to cnode
// create a new crate sema - for semantic analysis of cnode
impl Validate for CConst {
    type Result = super::Result<()>;

    fn validate_ciface(&self) -> Self::Result {
        type Error = super::Error;
        match self {
            CConst::SignedInt { value, .. } => {
                if !value.chars().all(|c| c.is_ascii_digit()) {
                    let error_message = format!("non ascii digits detected in {}", value);
                    return Err(Error::CConst(error_message));
                }
                return Ok(());
            }
            CConst::HexDigit(value) => {
                if !value.chars().all(|c| c.is_ascii_hexdigit()) {
                    let error_message = format!("non ascii hex-digits detected in {}", value);
                    return Err(Error::CConst(error_message));
                }
                return Ok(());
            }
        }
    }
}
