

use cassandra::error::CassError;
use cassandra::util::Protected;


use cassandra_sys::CASS_OK;
use cassandra_sys::CassUuid as _Uuid;
use cassandra_sys::CassUuidGen as _UuidGen;
use cassandra_sys::cass_uuid_from_string;
use cassandra_sys::cass_uuid_gen_free;
use cassandra_sys::cass_uuid_gen_from_time;
use cassandra_sys::cass_uuid_gen_new;
use cassandra_sys::cass_uuid_gen_new_with_node;
use cassandra_sys::cass_uuid_gen_random;
use cassandra_sys::cass_uuid_gen_time;
use cassandra_sys::cass_uuid_max_from_time;
use cassandra_sys::cass_uuid_min_from_time;

use cassandra_sys::cass_uuid_string;
use cassandra_sys::cass_uuid_timestamp;
use cassandra_sys::cass_uuid_version;
use errors::*;
use std::ffi::CString;
use std::fmt;
use std::fmt::{Debug, Display};
use std::fmt::Formatter;
use std::mem;
use std::str;

const CASS_UUID_STRING_LENGTH: usize = 37;


#[derive(Copy,Clone)]
/// Version 1 (time-based) or version 4 (random) UUID.
pub struct Uuid(_Uuid);

impl Protected<_Uuid> for Uuid {
    fn inner(&self) -> _Uuid { self.0 }
    fn build(inner: _Uuid) -> Self { Uuid(inner) }
}

impl Default for Uuid {
    fn default() -> Uuid { unsafe { ::std::mem::zeroed() } }
}

/// A UUID generator object.
///
/// Instances of the UUID generator object are thread-safe to generate UUIDs.
#[derive(Debug)]
pub struct UuidGen(*mut _UuidGen);
unsafe impl Sync for UuidGen {}
unsafe impl Send for UuidGen {}

impl Drop for UuidGen {
    fn drop(&mut self) { unsafe { cass_uuid_gen_free(self.0) } }
}

impl Debug for Uuid {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result { fmt::Display::fmt(self, f) }
}

impl Display for Uuid {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        unsafe {
            // Allocate a CString large enough for cass_uuid_string to write to.
            let mut buf = CString::from_vec_unchecked(vec!(0u8; CASS_UUID_STRING_LENGTH));
            let cstr = buf.into_raw(); // Convert to *mut c_char
            cass_uuid_string(self.0, cstr); // Write the UUID to *c_char
            buf = CString::from_raw(cstr); // Convert from *c_char back to a CString.
            let str = match buf.into_string() {
                Ok(s) => s,
                Err(_) => return Err(fmt::Error),
            };
            fmt::Display::fmt(&str, f)
        }
    }
}

impl Uuid {
    /// Generates a V1 (time) UUID for the specified time.
    pub fn min_from_time(&mut self, time: u64) { unsafe { cass_uuid_min_from_time(time, &mut self.0) } }

    /// Sets the UUID to the minimum V1 (time) value for the specified tim
    pub fn max_from_time(&mut self, time: u64) { unsafe { cass_uuid_max_from_time(time, &mut self.0) } }

    /// Gets the timestamp for a V1 UUID
    pub fn timestamp(&self) -> u64 { unsafe { cass_uuid_timestamp(self.0) } }

    /// Gets the version for a UUID
    pub fn version(&self) -> u8 { unsafe { cass_uuid_version(self.0) } }
}

impl str::FromStr for Uuid {
    type Err = Error;
    fn from_str(str: &str) -> Result<Uuid> {
        unsafe {
            let mut uuid = mem::zeroed();
            match cass_uuid_from_string(CString::new(str).expect("must be utf8").as_ptr(), &mut uuid) {
                CASS_OK => Ok(Uuid(uuid)),
                err => {
                    err.to_result(Uuid(uuid))
                        .chain_err(|| "")
                }
            }
        }
    }
}

impl Default for UuidGen {
    /// Creates a new thread-safe UUID generator
    fn default() -> Self { unsafe { UuidGen(cass_uuid_gen_new()) } }
}

impl UuidGen {
    /// Creates a new UUID generator with custom node information.
    /// <b>Note:</b> This object is thread-safe. It is best practice to create and reuse
    /// a single object per application.
    pub fn new_with_node(node: u64) -> UuidGen { unsafe { UuidGen(cass_uuid_gen_new_with_node(node)) } }

    /// Generates a V1 (time) UUID.
    pub fn gen_time(&self) -> Uuid {
        unsafe {
            let mut output: _Uuid = mem::zeroed();
            cass_uuid_gen_time(self.0, &mut output);
            Uuid(output)
        }
    }

    /// Generates a new V4 (random) UUID
    pub fn gen_random(&self) -> Uuid {
        unsafe {
            let mut output: _Uuid = mem::zeroed();
            cass_uuid_gen_random(self.0, &mut output);
            Uuid(output)
        }
    }

    /// Generates a V1 (time) UUID for the specified time.
    ///
    /// # Examples
    ///
    /// ```
    /// # use cassandra::{UuidGen, Uuid};
    /// # #[allow(dead_code)]
    /// # fn example() -> Uuid {
    /// let generator = UuidGen::default();
    /// let uuid = generator.gen_from_time(1457486866742u64);
    /// # uuid
    /// # }
    /// ```
    pub fn gen_from_time(&self, timestamp: u64) -> Uuid {
        unsafe {
            let mut output: _Uuid = mem::zeroed();
            cass_uuid_gen_from_time(self.0, timestamp, &mut output);
            Uuid(output)
        }
    }
}

#[test]
#[allow(unused_variables)]
fn test_uuid_display_gentime() {
    let generator = UuidGen::default();
    let uuid = generator.gen_from_time(1457486866742u64);
    assert_eq!(uuid.timestamp(), 1457486866742u64);
    let uuidstr = format!("{}", uuid); // Test Display trait
}

#[test]
#[allow(unused_variables)]
fn test_uuid_debug_genrand() {
    let generator = UuidGen::default();
    let uuid = generator.gen_random();
    let uuidstr = format!("{:?}", uuid); // Test Debug trait
}
