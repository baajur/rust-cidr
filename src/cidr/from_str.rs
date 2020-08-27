use crate::local_addr_parser::ParseableAddress;
use std::str::FromStr;

use super::super::errors::*;
use super::super::traits::*;

pub fn cidr_from_str<C>(s: &str) -> Result<C, NetworkParseError>
where
	C: Cidr,
	C::Address: ParseableAddress,
{
	match s.rfind('/') {
		None => Ok(C::new_host(C::Address::address_from_str(s)?)),
		Some(pos) => {
			C::new(C::Address::address_from_str(&s[0..pos])?, u8::from_str(&s[pos + 1..])?)
		},
	}
}
