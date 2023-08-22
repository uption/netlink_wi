use std::{fmt::Display, result};

use neli::{
    consts::nl::Nlmsg,
    err::{DeError, Nlmsgerr, NlmsghdrErr, RouterError},
    genl::Genlmsghdr,
};
use nix::errno::Errno;

use crate::{attributes::Attribute, commands::Command};

pub type Result<T> = result::Result<T, NlError>;

#[derive(Clone, Debug)]
pub struct NlError {
    pub msg: String,
}

impl NlError {
    pub fn new<T: Display>(msg: T) -> NlError {
        NlError {
            msg: msg.to_string(),
        }
    }
}

impl std::fmt::Display for NlError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Netlink error: {}", self.msg)
    }
}

impl std::error::Error for NlError {}

impl<T, P> From<RouterError<T, P>> for NlError
where
    T: std::fmt::Debug,
    P: std::fmt::Debug,
{
    fn from(value: RouterError<T, P>) -> Self {
        match &value {
            RouterError::Nlmsgerr(err) => NlError::new(Errno::from_i32(-*err.error())),
            _ => NlError::new(value),
        }
    }
}

impl From<DeError> for NlError {
    fn from(value: DeError) -> Self {
        NlError::new(value)
    }
}

type Nl80211Msgerr = Nlmsgerr<NlmsghdrErr<Nlmsg, Genlmsghdr<Command, Attribute>>>;

impl From<Nl80211Msgerr> for NlError {
    fn from(value: Nl80211Msgerr) -> Self {
        NlError::new(Errno::from_i32(-value.error()))
    }
}
