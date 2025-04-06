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
