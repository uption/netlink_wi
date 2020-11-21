//! This module contains the error types used by `netlink-wi`.
use std::error;
use std::fmt;

#[derive(Debug)]
pub struct AttrParseError {
    msg: String,
    attr: String,
}

impl AttrParseError {
    pub fn new(msg: impl fmt::Display, attr: impl fmt::Debug) -> Self {
        Self {
            msg: format!("{}", msg),
            attr: format!("{:?}", attr),
        }
    }
}

impl fmt::Display for AttrParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to parse attribute {}: {}", self.attr, self.msg)
    }
}

impl error::Error for AttrParseError {}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::attributes::Attribute;
    #[test]
    fn display_error() {
        let err = AttrParseError::new("test", Attribute::WiphyName);
        assert_eq!(
            "Failed to parse attribute WiphyName: test",
            format!("{}", err)
        );
    }
}
