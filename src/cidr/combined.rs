use std::fmt;
use std::net::IpAddr;
use std::str::FromStr;

use super::super::errors::*;
use super::super::family::Family;
use super::super::inet::*;
use super::super::traits::*;
use super::from_str::cidr_from_str;
use super::{IpCidr, Ipv4Cidr, Ipv6Cidr};

impl IpCidr {
	/// Whether representing an IPv4 network
	pub fn is_ipv4(&self) -> bool {
		match *self {
			IpCidr::V4(_) => true,
			IpCidr::V6(_) => false,
		}
	}

	/// Whether representing an IPv6 network
	pub fn is_ipv6(&self) -> bool {
		match *self {
			IpCidr::V4(_) => false,
			IpCidr::V6(_) => true,
		}
	}
}

impl Cidr for IpCidr {
	type Address = IpAddr;
	type Inet = IpInet;

	fn new(addr: Self::Address, len: u8) -> Result<Self, NetworkParseError> {
		Ok(match addr {
			IpAddr::V4(a) => IpCidr::V4(Ipv4Cidr::new(a, len)?),
			IpAddr::V6(a) => IpCidr::V6(Ipv6Cidr::new(a, len)?),
		})
	}

	fn new_host(addr: Self::Address) -> Self {
		match addr {
			IpAddr::V4(a) => IpCidr::V4(Ipv4Cidr::new_host(a)),
			IpAddr::V6(a) => IpCidr::V6(Ipv6Cidr::new_host(a)),
		}
	}

	fn first_address(&self) -> Self::Address {
		match *self {
			IpCidr::V4(ref c) => IpAddr::V4(c.first_address()),
			IpCidr::V6(ref c) => IpAddr::V6(c.first_address()),
		}
	}

	fn first(&self) -> Self::Inet {
		match *self {
			IpCidr::V4(ref c) => IpInet::V4(c.first()),
			IpCidr::V6(ref c) => IpInet::V6(c.first()),
		}
	}

	fn last_address(&self) -> Self::Address {
		match *self {
			IpCidr::V4(ref c) => IpAddr::V4(c.last_address()),
			IpCidr::V6(ref c) => IpAddr::V6(c.last_address()),
		}
	}

	fn last(&self) -> Self::Inet {
		match *self {
			IpCidr::V4(ref c) => IpInet::V4(c.last()),
			IpCidr::V6(ref c) => IpInet::V6(c.last()),
		}
	}

	fn network_length(&self) -> u8 {
		match *self {
			IpCidr::V4(ref c) => c.network_length(),
			IpCidr::V6(ref c) => c.network_length(),
		}
	}

	fn family(&self) -> Family {
		match *self {
			IpCidr::V4(_) => Family::Ipv4,
			IpCidr::V6(_) => Family::Ipv6,
		}
	}

	fn mask(&self) -> Self::Address {
		match *self {
			IpCidr::V4(ref c) => IpAddr::V4(c.mask()),
			IpCidr::V6(ref c) => IpAddr::V6(c.mask()),
		}
	}

	fn contains(&self, addr: &Self::Address) -> bool {
		match *self {
			IpCidr::V4(ref c) => match *addr {
				IpAddr::V4(ref a) => c.contains(a),
				IpAddr::V6(_) => false,
			},
			IpCidr::V6(ref c) => match *addr {
				IpAddr::V4(_) => false,
				IpAddr::V6(ref a) => c.contains(a),
			},
		}
	}
}

impl fmt::Display for IpCidr {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			IpCidr::V4(ref c) => fmt::Display::fmt(c, f),
			IpCidr::V6(ref c) => fmt::Display::fmt(c, f),
		}
	}
}

impl FromStr for IpCidr {
	type Err = NetworkParseError;
	fn from_str(s: &str) -> Result<IpCidr, NetworkParseError> {
		cidr_from_str(s)
	}
}

impl From<Ipv4Cidr> for IpCidr {
	fn from(c: Ipv4Cidr) -> Self {
		IpCidr::V4(c)
	}
}

impl From<Ipv6Cidr> for IpCidr {
	fn from(c: Ipv6Cidr) -> Self {
		IpCidr::V6(c)
	}
}
