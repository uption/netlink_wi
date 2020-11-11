use std::convert::TryFrom;
use std::convert::TryInto;
use std::fmt;
#[derive(Debug, Copy, Clone)]
pub struct MacAddress {
    address_bytes: [u8; 6],
}

impl MacAddress {
    fn as_bytes(&self) -> [u8; 6] {
        self.address_bytes
    }
}

impl fmt::Display for MacAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let foo: Vec<String> = self
            .address_bytes
            .iter()
            .map(|x| format!("{:X?}", x))
            .collect();
        let foo = foo.join(":");
        write!(f, "{}", foo)
    }
}

impl TryFrom<&[u8]> for MacAddress {
    type Error = std::array::TryFromSliceError;
    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        Ok(MacAddress {
            address_bytes: slice.try_into()?,
        })
    }
}
