use cassandra::util::Protected;
use cassandra::error::*;

use cassandra_sys::CassSslVerifyFlags;
use cassandra_sys::CassSsl as _Ssl;
use cassandra_sys::cass_ssl_add_trusted_cert;
use cassandra_sys::cass_ssl_free;
use cassandra_sys::cass_ssl_new;
use cassandra_sys::cass_ssl_set_cert;
use cassandra_sys::cass_ssl_set_private_key;
use cassandra_sys::cass_ssl_set_verify_flags;

use std::ffi::CString;

/// The individual SSL verification levels.
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
#[allow(missing_docs)] // Meanings are defined in CQL documentation.
#[allow(non_camel_case_types)] // Names are traditional.
pub enum SslVerifyFlag {
    NONE,
    PEER_CERT,
    PEER_IDENTITY,
    PEER_IDENTITY_DNS,
}

enhance_nullary_enum!(SslVerifyFlag, CassSslVerifyFlags, {
    (NONE, CASS_SSL_VERIFY_NONE, "NONE"),
    (PEER_CERT, CASS_SSL_VERIFY_PEER_CERT, "PEER_CERT"),
    (PEER_IDENTITY, CASS_SSL_VERIFY_PEER_IDENTITY, "PEER_IDENTITY"),
    (PEER_IDENTITY_DNS, CASS_SSL_VERIFY_PEER_IDENTITY_DNS, "PEER_IDENTITY_DNS"),
});

fn to_bitset(flags: &[SslVerifyFlag]) -> i32 {
    let mut res = 0;
    for f in flags.iter() {
        res = res | f.inner() as u32;
    }
    res as i32
}

/// Describes the SSL configuration of a cluster.
#[derive(Debug)]
pub struct Ssl(*mut _Ssl);

// The underlying C type has no thread-local state, but does not support access
// from multiple threads: https://datastax.github.io/cpp-driver/topics/#thread-safety
unsafe impl Send for Ssl {}

impl Protected<*mut _Ssl> for Ssl {
    fn inner(&self) -> *mut _Ssl { self.0 }
    fn build(inner: *mut _Ssl) -> Self { Ssl(inner) }
}


impl Drop for Ssl {
    /// Frees a SSL context instance.
    fn drop(&mut self) { unsafe { cass_ssl_free(self.0) } }
}

impl Default for Ssl {
    /// Creates a new SSL context.
    fn default() -> Ssl { unsafe { Ssl(cass_ssl_new()) } }
}

impl Ssl {
    /// Adds a trusted certificate. This is used to verify
    /// the peer's certificate.
    pub fn add_trusted_cert(&mut self, cert: &str) -> Result<&mut Self> {
        unsafe {
            cass_ssl_add_trusted_cert(self.0, CString::new(cert)?.as_ptr())
                .to_result(self)
        }
    }

    /// Sets verification performed on the peer's certificate.
    ///
    /// CASS_SSL_VERIFY_NONE - No verification is performed
    ///
    /// CASS_SSL_VERIFY_PEER_CERT - Certificate is present and valid
    ///
    /// CASS_SSL_VERIFY_PEER_IDENTITY - IP address matches the certificate's
    /// common name or one of its subject alternative names. This implies the
    /// certificate is also present.
    ///
    /// <b>Default:</b> CASS_SSL_VERIFY_PEER_CERT
    pub fn set_verify_flags(&mut self, flags: &[SslVerifyFlag]) { unsafe { cass_ssl_set_verify_flags(self.0, to_bitset(flags)) } }

    /// Set client-side certificate chain. This is used to authenticate
    /// the client on the server-side. This should contain the entire
    /// Certificate chain starting with the certificate itself.
    pub fn set_cert(&mut self, cert: &str) -> Result<&mut Self> {
        unsafe {
            cass_ssl_set_cert(self.0, CString::new(cert)?.as_ptr())
                .to_result(self)
        }
    }

    /// Set client-side private key. This is used to authenticate
    /// the client on the server-side.
    pub fn set_private_key(&mut self, key: &str, password: &str) -> Result<&mut Self> {
        unsafe {
            cass_ssl_set_private_key(self.0,
                                     CString::new(key)?.as_ptr(),
                                     password.as_ptr() as *const i8)
                .to_result(self)
        }
    }
}
