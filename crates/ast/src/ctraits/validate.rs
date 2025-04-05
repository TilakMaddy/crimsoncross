pub trait Validate {
    type Result;
    fn validate_ciface(&self) -> Self::Result;
}
