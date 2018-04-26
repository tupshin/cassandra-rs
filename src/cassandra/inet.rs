use cassandra::util::Protected;
use cassandra::error::*;
use cassandra_sys::CassInet as _Inet;
use cassandra_sys::cass_inet_from_string;
use cassandra_sys::cass_inet_init_v4;
use cassandra_sys::cass_inet_init_v6;
use cassandra_sys::cass_inet_string;
use std::default::Default;
use std::ffi::CStr;
use std::ffi::CString;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::mem;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::str::FromStr;
use std::string::ToString;

#[repr(C)]
/// Cassandra's version of an IP address

pub struct Inet(_Inet);

impl Debug for Inet {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result { write!(f, "can't format an inet") }
}

impl Protected<_Inet> for Inet {
    fn inner(&self) -> _Inet { self.0 }
    fn build(inner: _Inet) -> Self { Inet(inner) }
}

impl Default for Inet {
    fn default() -> Inet { unsafe { ::std::mem::zeroed() } }
}

impl Inet {
    /// Constructs an inet v4 object.
    pub fn cass_inet_init_v4(address: &Ipv4Addr) -> Inet {
        unsafe { Inet(cass_inet_init_v4(address.octets().as_ptr())) }
    }

    /// Constructs an inet v6 object.
    pub fn cass_inet_init_v6(address: &Ipv6Addr) -> Inet {
        unsafe { Inet(cass_inet_init_v6(address.octets().as_ptr())) }
    }
}

impl<'a> From<&'a IpAddr> for Inet {
    fn from(ip_addr: &IpAddr) -> Inet {
        match *ip_addr {
            IpAddr::V4(ref ipv4_addr) => Inet::cass_inet_init_v4(ipv4_addr),
            IpAddr::V6(ref ipv6_addr) => Inet::cass_inet_init_v6(ipv6_addr),
        }
    }
}

/// The types of errors that can occur when trying to parse an Inet String
// pub enum InetParseError {
//    ///Don't put a null in a string, silly!
//    NulInString(NulError),
//    ///Not a valiid address
//    LibBadParams(CassLibError),
// }

impl FromStr for Inet {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        unsafe {
            let mut inet = mem::zeroed();

            let str = CString::new(s)?;
            cass_inet_from_string(str.as_ptr(), &mut inet).to_result(())
                .and_then(|_| Ok(Inet(inet)))
        }
    }
}

impl ToString for Inet {
    fn to_string(&self) -> String {
        unsafe {
            let mut inet_str = mem::zeroed();
            cass_inet_string(self.0, &mut inet_str);
            CStr::from_ptr(&inet_str).to_string_lossy().into_owned()
        }
    }
}

impl<'a> From<&'a Inet> for IpAddr {
    fn from(inet: &Inet) -> Self {
        match inet.0.address_length {
            4 => {
                let mut octets = [0u8; 4];
                octets.copy_from_slice(&inet.0.address[0..4]);
                IpAddr::from(octets)
            },
            16 => IpAddr::from(inet.0.address),
            unsupported => panic!("impossible inet type: {}", unsupported),
        }
    }
}

#[test]
fn ipv4_conversion() {
     let ipv4_in = Ipv4Addr::new(127, 0, 0, 1);
     let inet = Inet::cass_inet_init_v4(&ipv4_in);
     let ip_out = IpAddr::from(&inet);
     assert_eq!(IpAddr::V4(ipv4_in), ip_out);
}

#[test]
fn ipv6_conversion() {
     let ipv6_in = Ipv6Addr::new(0x2001, 0x0db8, 0, 0, 0, 0, 0, 1);
     let inet = Inet::cass_inet_init_v6(&ipv6_in);
     let ip_out = IpAddr::from(&inet);
     assert_eq!(IpAddr::V6(ipv6_in), ip_out);
}
