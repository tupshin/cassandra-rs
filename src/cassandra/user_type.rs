

use cassandra::collection::Set;
use cassandra::data_type::ConstDataType;
use cassandra::error::CassError;
use cassandra::inet::Inet;
use cassandra::tuple::Tuple;
use cassandra::util::Protected;

use cassandra::uuid::Uuid;
use cassandra_sys::CassUserType as _UserType;
use cassandra_sys::cass_false;

use cassandra_sys::cass_true;
use cassandra_sys::cass_user_type_data_type;
use cassandra_sys::cass_user_type_free;
use cassandra_sys::cass_user_type_set_bool;
use cassandra_sys::cass_user_type_set_bool_by_name;
use cassandra_sys::cass_user_type_set_bytes;
use cassandra_sys::cass_user_type_set_bytes_by_name;
use cassandra_sys::cass_user_type_set_collection;
use cassandra_sys::cass_user_type_set_collection_by_name;
#[allow(unused_imports)]
use cassandra_sys::cass_user_type_set_decimal;
#[allow(unused_imports)]
use cassandra_sys::cass_user_type_set_decimal_by_name;
use cassandra_sys::cass_user_type_set_double;
use cassandra_sys::cass_user_type_set_double_by_name;
use cassandra_sys::cass_user_type_set_float;
use cassandra_sys::cass_user_type_set_float_by_name;
use cassandra_sys::cass_user_type_set_inet;
use cassandra_sys::cass_user_type_set_inet_by_name;
use cassandra_sys::cass_user_type_set_int16;
use cassandra_sys::cass_user_type_set_int16_by_name;
use cassandra_sys::cass_user_type_set_int32;
use cassandra_sys::cass_user_type_set_int32_by_name;
use cassandra_sys::cass_user_type_set_int64;
use cassandra_sys::cass_user_type_set_int64_by_name;
use cassandra_sys::cass_user_type_set_int8;
use cassandra_sys::cass_user_type_set_int8_by_name;
use cassandra_sys::cass_user_type_set_null;
use cassandra_sys::cass_user_type_set_null_by_name;
use cassandra_sys::cass_user_type_set_string;
use cassandra_sys::cass_user_type_set_string_by_name;
use cassandra_sys::cass_user_type_set_tuple;
use cassandra_sys::cass_user_type_set_tuple_by_name;
use cassandra_sys::cass_user_type_set_uint32;
use cassandra_sys::cass_user_type_set_uint32_by_name;
use cassandra_sys::cass_user_type_set_user_type;
use cassandra_sys::cass_user_type_set_user_type_by_name;
use cassandra_sys::cass_user_type_set_uuid;

use cassandra_sys::cass_user_type_set_uuid_by_name;
use errors::*;
use std::ffi::CString;
// use cassandra::iterator::FieldIterator;

/// A user defined type
#[derive(Debug)]
pub struct UserType(*mut _UserType);

impl Protected<*mut _UserType> for UserType {
    fn inner(&self) -> *mut _UserType { self.0 }
    fn build(inner: *mut _UserType) -> Self { UserType(inner) }
}


impl Drop for UserType {
    fn drop(&mut self) { unsafe { cass_user_type_free(self.0) } }
}

// impl Drop for UserType {
//    fn drop(&mut self) {unsafe{
//        cass_user_type_free(self.0)
//    }}
// }

impl UserType {
    /// Gets the data type of a user defined type.
    pub fn data_type(&self) -> ConstDataType { unsafe { ConstDataType(cass_user_type_data_type(self.0)) } }

    /// Sets a null in a user defined type at the specified index.
    pub fn set_null(&mut self, index: usize) -> Result<&mut Self> {
        unsafe {
            cass_user_type_set_null(self.0, index)
                .to_result(self)
                .chain_err(|| "")
        }
    }

    /// Sets a null in a user defined type at the specified name.
    pub fn set_null_by_name<S>(&mut self, name: S) -> Result<&mut Self>
        where S: Into<String> {
        unsafe {
            cass_user_type_set_null_by_name(self.0,
                                            CString::new(name.into()).expect("must be utf8").as_ptr())
                .to_result(self)
                .chain_err(|| "")
        }
    }

    /// Sets a "tinyint" in a user defined type at the specified index.
    pub fn set_int8(&mut self, index: usize, value: i8) -> Result<&mut Self> {
        unsafe {
            cass_user_type_set_int8(self.0, index, value)
                .to_result(self)
                .chain_err(|| "")
        }
    }

    /// Sets a "tinyint" in a user defined type at the specified name.
    pub fn set_int8_by_name<S>(&mut self, name: S, value: i8) -> Result<&mut Self>
        where S: Into<String> {
        unsafe {
            match CString::new(name.into()) {
                Ok(name) => {
                    cass_user_type_set_int8_by_name(self.0, name.as_ptr(), value)
                        .to_result(self)
                        .chain_err(|| "")
                }
                Err(err) => panic!("error: {}", err),
            }

        }
    }

    /// Sets an "smallint" in a user defined type at the specified index.
    pub fn set_int16(&mut self, index: usize, value: i16) -> Result<&mut Self> {
        unsafe {
            cass_user_type_set_int16(self.0, index, value)
                .to_result(self)
                .chain_err(|| "")
        }
    }

    /// Sets an "smallint" in a user defined type at the specified name.
    pub fn set_int16_by_name<S>(&mut self, name: S, value: i16) -> Result<&mut Self>
        where S: Into<String> {
        unsafe {
            match CString::new(name.into()) {
                Ok(name) => {
                    cass_user_type_set_int16_by_name(self.0, name.as_ptr(), value)
                        .to_result(self)
                        .chain_err(|| "")
                }
                Err(err) => panic!("error: {}", err),
            }

        }
    }

    /// Sets an "int" in a user defined type at the specified index.
    pub fn set_int32(&mut self, index: usize, value: i32) -> Result<&mut Self> {
        unsafe {
            cass_user_type_set_int32(self.0, index, value)
                .to_result(self)
                .chain_err(|| "")
        }
    }

    /// Sets an "int" in a user defined type at the specified name.
    pub fn set_int32_by_name<S>(&mut self, name: S, value: i32) -> Result<&mut Self>
        where S: Into<String> {
        unsafe {
            match CString::new(name.into()) {
                Ok(name) => {
                    cass_user_type_set_int32_by_name(self.0, name.as_ptr(), value)
                        .to_result(self)
                        .chain_err(|| "")
                }
                Err(err) => panic!("error: {}", err),
            }

        }
    }

    /// Sets a "date" in a user defined type at the specified index.
    pub fn set_uint32(&mut self, index: usize, value: u32) -> Result<&mut Self> {
        unsafe {
            cass_user_type_set_uint32(self.0, index, value)
                .to_result(self)
                .chain_err(|| "")
        }
    }

    /// Sets a "date" in a user defined type at the specified name.
    pub fn set_uint32_by_name<S>(&mut self, name: S, value: u32) -> Result<&mut Self>
        where S: Into<String> {
        unsafe {
            match CString::new(name.into()) {
                Ok(name) => {
                    cass_user_type_set_uint32_by_name(self.0, name.as_ptr(), value)
                        .to_result(self)
                        .chain_err(|| "")
                }
                Err(err) => panic!("error: {}", err),
            }

        }
    }


    /// Sets an "bigint", "counter", "timestamp" or "time" in a
    /// user defined type at the specified index.
    pub fn set_int64(&mut self, index: usize, value: i64) -> Result<&mut Self> {
        unsafe {
            cass_user_type_set_int64(self.0, index, value)
                .to_result(self)
                .chain_err(|| "")
        }
    }

    /// Sets an "bigint", "counter", "timestamp" or "time" in a
    /// user defined type at the specified name.
    pub fn set_int64_by_name<S>(&mut self, name: S, value: i64) -> Result<&mut Self>
        where S: Into<String> {
        unsafe {
            cass_user_type_set_int64_by_name(self.0,
                                             CString::new(name.into())
                                                 .expect("must be utf8")
                                                 .as_ptr(),
                                             value)
                .to_result(self)
                .chain_err(|| "")
        }
    }

    /// Sets a "float" in a user defined type at the specified index.
    pub fn set_float(&mut self, index: usize, value: f32) -> Result<&mut Self> {
        unsafe {
            cass_user_type_set_float(self.0, index, value)
                .to_result(self)
                .chain_err(|| "")
        }
    }

    /// Sets a "float" in a user defined type at the specified name.
    pub fn set_float_by_name<S>(&mut self, name: S, value: f32) -> Result<&mut Self>
        where S: Into<String> {
        unsafe {
            cass_user_type_set_float_by_name(self.0,
                                             CString::new(name.into())
                                                 .expect("must be utf8")
                                                 .as_ptr(),
                                             value)
                .to_result(self)
                .chain_err(|| "")
        }
    }

    /// Sets an "double" in a user defined type at the specified index.
    pub fn set_double(&mut self, index: usize, value: f64) -> Result<&mut Self> {
        unsafe {
            cass_user_type_set_double(self.0, index, value)
                .to_result(self)
                .chain_err(|| "")
        }
    }

    /// Sets an "double" in a user defined type at the specified name.

    pub fn set_double_by_name<S>(&mut self, name: S, value: f64) -> Result<&mut Self>
        where S: Into<String> {
        unsafe {
            cass_user_type_set_double_by_name(self.0,
                                              CString::new(name.into())
                                                  .expect("must be utf8")
                                                  .as_ptr(),
                                              value)
                .to_result(self)
                .chain_err(|| "")
        }
    }

    /// Sets a "boolean" in a user defined type at the specified index.
    pub fn set_bool(&mut self, index: usize, value: bool) -> Result<&mut Self> {
        unsafe {
            cass_user_type_set_bool(self.0, index, if value { cass_true } else { cass_false })
                .to_result(self)
                .chain_err(|| "")
        }
    }

    /// Sets a "boolean" in a user defined type at the specified name.
    pub fn set_bool_by_name<S>(&mut self, name: S, value: bool) -> Result<&mut Self>
        where S: Into<String> {
        unsafe {
            cass_user_type_set_bool_by_name(self.0,
                                            CString::new(name.into()).expect("must be utf8").as_ptr(),
                                            if value { cass_true } else { cass_false })
                .to_result(self)
                .chain_err(|| "")
        }
    }

    /// Sets an "ascii", "text" or "varchar" in a user defined type at the
    /// specified index.
    pub fn set_stringl<S>(&mut self, index: usize, value: S) -> Result<&mut Self>
        where S: Into<String> {
        unsafe {
            cass_user_type_set_string(self.0,
                                      index,
                                      CString::new(value.into()).expect("must be utf8").as_ptr())
                .to_result(self)
                .chain_err(|| "")
        }
    }

    /// Sets an "ascii", "text" or "varchar" in a user defined type at the
    /// specified name.
    pub fn set_string_by_name<S>(&mut self, name: S, value: S) -> Result<&mut Self>
        where S: Into<String> {
        unsafe {
            cass_user_type_set_string_by_name(self.0,
                                              CString::new(name.into())
                                                  .expect("must be utf8")
                                                  .as_ptr(),
                                              CString::new(value.into())
                                                  .expect("must be utf8")
                                                  .as_ptr())
                .to_result(self)
                .chain_err(|| "")
        }
    }

    // FIXME. right way to pass the vec?
    /// Sets a "blob" "varint" or "custom" in a user defined type at the specified index.
    pub fn set_bytes(&mut self, index: usize, value: Vec<u8>) -> Result<&mut Self> {
        unsafe {
            cass_user_type_set_bytes(self.0, index, value.as_ptr(), value.len())
                .to_result(self)
                .chain_err(|| "")
        }
    }

    /// Sets a "blob", "varint" or "custom" in a user defined type at the specified name.
    pub fn set_bytes_by_name<S>(&mut self, name: S, value: Vec<u8>) -> Result<&mut Self>
        where S: Into<String> {
        unsafe {
            cass_user_type_set_bytes_by_name(self.0,
                                             CString::new(name.into())
                                                 .expect("must be utf8")
                                                 .as_ptr(),
                                             value.as_ptr(),
                                             value.len())
                .to_result(self)
                .chain_err(|| "")
        }
    }

    /// Sets a "uuid" or "timeuuid" in a user defined type at the specified index.
    pub fn set_uuid<S>(&mut self, index: usize, value: S) -> Result<&mut Self>
        where S: Into<Uuid> {
        unsafe {
            cass_user_type_set_uuid(self.0, index, value.into().inner())
                .to_result(self)
                .chain_err(|| "")
        }
    }

    /// Sets a "uuid" or "timeuuid" in a user defined type at the specified name.
    pub fn set_uuid_by_name<S, U>(&mut self, name: S, value: U) -> Result<&mut Self>
        where S: Into<String>, U: Into<Uuid> {
        unsafe {
            cass_user_type_set_uuid_by_name(self.0,
                                            CString::new(name.into()).expect("must be utf8").as_ptr(),
                                            value.into().inner())
                .to_result(self)
                .chain_err(|| "")
        }
    }

    /// Sets a "inet" in a user defined type at the specified index.
    pub fn set_inet<S>(&mut self, index: usize, value: S) -> Result<&mut Self>
        where S: Into<Inet> {
        unsafe {
            cass_user_type_set_inet(self.0, index, value.into().inner())
                .to_result(self)
                .chain_err(|| "")
        }
    }

    /// Sets a "inet" in a user defined type at the specified name.
    pub fn set_inet_by_name<S, U>(&mut self, name: S, value: U) -> Result<&mut Self>
        where S: Into<String>, U: Into<Inet> {
        unsafe {
            cass_user_type_set_inet_by_name(self.0,
                                            CString::new(name.into()).expect("must be utf8").as_ptr(),
                                            value.into().inner())
                .to_result(self)
                .chain_err(|| "")
        }
    }

    /// Sets a "list", "map" or "set" in a user defined type at the specified index.
    pub fn set_collection<S>(&mut self, index: usize, value: S) -> Result<&mut Self>
        where S: Into<Set> {
        unsafe {
            cass_user_type_set_collection(self.0, index, value.into().inner())
                .to_result(self)
                .chain_err(|| "")
        }
    }

    /// Sets a "list", "map" or "set" in a user defined type at the
    /// specified name.
    pub fn set_collection_by_name<S>(&mut self, name: S, value: Set) -> Result<&mut Self>
        where S: Into<String> {
        unsafe {
            cass_user_type_set_collection_by_name(self.0,
                                                  CString::new(name.into())
                                                      .expect("must be utf8")
                                                      .as_ptr(),
                                                  value.inner())
                .to_result(self)
                .chain_err(|| "")
        }
    }

    /// Sets a "tuple" in a user defined type at the specified index.
    pub fn set_tuple(&mut self, index: usize, value: Tuple) -> Result<&mut Self> {
        unsafe {
            cass_user_type_set_tuple(self.0, index, value.inner())
                .to_result(self)
                .chain_err(|| "")
        }
    }

    /// Sets a "tuple" in a user defined type at the specified name.
    pub fn set_tuple_by_name<S>(&mut self, name: S, value: Tuple) -> Result<&mut Self>
        where S: Into<String> {
        unsafe {
            cass_user_type_set_tuple_by_name(self.0,
                                             CString::new(name.into())
                                                 .expect("must be utf8")
                                                 .as_ptr(),
                                             value.inner())
                .to_result(self)
                .chain_err(|| "")
        }
    }

    /// Sets a user defined type in a user defined type at the specified index.
    pub fn set_user_type(&mut self, index: usize, value: UserType) -> Result<&mut Self> {
        unsafe {
            cass_user_type_set_user_type(self.0, index, value.0)
                .to_result(self)
                .chain_err(|| "")
        }
    }

    /// Sets a user defined type in a user defined type at the specified name.
    pub fn set_user_type_by_name<S>(&mut self, name: S, value: UserType) -> Result<&mut Self>
        where S: Into<String> {
        unsafe {
            cass_user_type_set_user_type_by_name(self.0,
                                                 CString::new(name.into())
                                                     .expect("must be utf8")
                                                     .as_ptr(),
                                                 value.0)
                .to_result(self)
                .chain_err(|| "")
        }
    }
}
