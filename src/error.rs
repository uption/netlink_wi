//! This module contains the error types used by `netlink-wi`.
use std::error;
use std::fmt;

use super::attributes::Attribute;

#[derive(Debug)]
pub struct AttrParseError {
    msg: String,
    attr: Attribute,
}

impl AttrParseError {
    pub fn new(msg: impl fmt::Display, attr: Attribute) -> Self {
        Self {
            msg: format!("{}", msg),
            attr,
        }
    }
}

impl fmt::Display for AttrParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to parse attribute {:?}: {}", self.attr, self.msg)
    }
}

impl error::Error for AttrParseError {}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn display_error() {
        let err = AttrParseError::new("test", Attribute::WiphyName);
        assert_eq!(
            "Failed to parse attribute WiphyName: test",
            format!("{}", err)
        );
    }
}
