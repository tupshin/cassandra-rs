use std::mem;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt;
use std::str;
use std::slice;

use cassandra_sys::CASS_VALUE_TYPE_ASCII;
use cassandra_sys::CASS_VALUE_TYPE_VARCHAR;
use cassandra_sys::CASS_VALUE_TYPE_TEXT;
use cassandra_sys::CASS_OK;
use cassandra_sys::cass_value_get_int32;
use cassandra_sys::cass_value_get_int64;
use cassandra_sys::cass_value_get_float;
use cassandra_sys::cass_value_get_double;
use cassandra_sys::cass_value_get_bool;
use cassandra_sys::cass_value_get_uuid;
use cassandra_sys::cass_value_get_string;
use cassandra_sys::cass_value_get_inet;
use cassandra_sys::cass_value_get_uint32;
use cassandra_sys::cass_value_get_int8;
use cassandra_sys::cass_value_get_int16;

#[allow(unused_imports)]
use cassandra_sys::cass_value_get_decimal;
use cassandra_sys::cass_iterator_from_map;
use cassandra_sys::cass_iterator_from_collection;
use cassandra_sys::cass_value_type;
use cassandra_sys::CassValue as _Value;
use cassandra_sys::cass_iterator_fields_from_user_type;

use cassandra::uuid::Uuid;
use cassandra::value::ValueType;
use cassandra::iterator::SetIterator;
use cassandra::iterator::UserTypeIterator;
use cassandra::inet::Inet;
use cassandra::iterator::MapIterator;
use cassandra::error::CassErrorTypes;
use cassandra::error::CassError;

// use decimal::d128;

#[repr(C)]
#[derive(Copy,Debug,Clone)]
pub enum ColumnType {
    PARTITION_KEY = 0,
    CLUSTERING_KEY = 1,
    REGULAR = 2,
    COMPACT_VALUE = 3,
    STATIC = 4,
    UNKNOWN = 5,
}

impl ColumnType {
    pub fn build(type_num: u32) -> Result<ColumnType, u32> {

        use ColumnType::*;
        match type_num {
            0 => Ok(PARTITION_KEY),
            1 => Ok(CLUSTERING_KEY),
            2 => Ok(REGULAR),
            3 => Ok(COMPACT_VALUE),
            4 => Ok(STATIC),
            5 => Ok(UNKNOWN),
            err => Err(err),
        }
    }
}

pub struct Column(pub *const _Value);

impl Debug for Column {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.get_type() {
            ValueType::UNKNOWN => write!(f, "UNKNOWN Cassandra type"),
            ValueType::CUSTOM => write!(f, "CUSTOM Cassandra type"),
            ValueType::ASCII => write!(f, "ASCII Cassandra type"),
            ValueType::BIGINT => write!(f, "BIGINT Cassandra type"),
            ValueType::BLOB => write!(f, "BLOB Cassandra type"),
            ValueType::BOOLEAN => write!(f, "BOOLEAN Cassandra type"),
            ValueType::COUNTER => write!(f, "COUNTER Cassandra type"),
            ValueType::DECIMAL => write!(f, "DECIMAL Cassandra type"),
            ValueType::DOUBLE => write!(f, "DOUBLE Cassandra type"),
            ValueType::FLOAT => write!(f, "FLOAT Cassandra type"),
            ValueType::INT => write!(f, "INT Cassandra type"),
            ValueType::TEXT => write!(f, "TEXT Cassandra type"),
            ValueType::TIMESTAMP => write!(f, "TIMESTAMP Cassandra type"),
            ValueType::UUID => write!(f, "UUID Cassandra type"),
            ValueType::VARCHAR => write!(f, "VARCHAR: {:?}", self.get_string()),
            ValueType::VARINT => Ok(()),
            ValueType::TIMEUUID => write!(f, "TIMEUUID Cassandra type"),
            ValueType::INET => write!(f, "INET Cassandra type"),
            ValueType::LIST => {
                for item in self.set_iter().unwrap() {
                    try!(write!(f, "LIST {:?}", item))
                }
                Ok(())
            }
            ValueType::MAP => {
                for item in self.map_iter().unwrap() {
                    try!(write!(f, "LIST {:?}", item))
                }
                Ok(())
            }
            ValueType::SET => {
                for item in self.set_iter().unwrap() {
                    try!(write!(f, "SET {:?}", item))
                }
                Ok(())
            }
            ValueType::UDT => write!(f, "UDT Cassandra type"),
            ValueType::TUPLE => write!(f, "Tuple Cassandra type"),
            ValueType::LASTENTRY => write!(f, "LAST_ENTRY Cassandra type"),
        }
    }
}

impl Display for Column {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.get_type() {
            ValueType::UNKNOWN => write!(f, "UNKNOWN Cassandra type"),
            ValueType::CUSTOM => write!(f, "CUSTOM Cassandra type"),
            ValueType::ASCII => write!(f, "ASCII Cassandra type"),
            ValueType::BIGINT => write!(f, "BIGINT Cassandra type"),
            ValueType::BLOB => write!(f, "BLOB Cassandra type"),
            ValueType::BOOLEAN => write!(f, "BOOLEAN Cassandra type"),
            ValueType::COUNTER => write!(f, "COUNTER Cassandra type"),
            ValueType::DECIMAL => write!(f, "DECIMAL Cassandra type"),
            ValueType::DOUBLE => write!(f, "DOUBLE Cassandra type"),
            ValueType::FLOAT => write!(f, "FLOAT Cassandra type"),
            ValueType::INT => write!(f, "INT Cassandra type"),
            ValueType::TEXT => write!(f, "TEXT Cassandra type"),
            ValueType::TIMESTAMP => write!(f, "TIMESTAMP Cassandra type"),
            ValueType::UUID => write!(f, "UUID Cassandra type"),
            ValueType::VARCHAR => write!(f, "{}", self.get_string().unwrap()),
            ValueType::VARINT => Ok(()),
            ValueType::TIMEUUID => write!(f, "TIMEUUID Cassandra type"),
            ValueType::INET => write!(f, "INET Cassandra type"),
            ValueType::LIST => {
                for item in self.set_iter().unwrap() {
                    try!(write!(f, "LIST {:?}", item))
                }
                Ok(())
            }
            ValueType::MAP => {
                for item in self.map_iter().unwrap() {
                    try!(write!(f, "LIST {:?}", item))
                }
                Ok(())
            }
            ValueType::SET => {
                for item in self.set_iter().unwrap() {
                    try!(write!(f, "SET {:?}", item))
                }
                Ok(())
            }
            ValueType::UDT => write!(f, "UDT Cassandra type"),
            ValueType::TUPLE => write!(f, "Tuple Cassandra type"),
            ValueType::LASTENTRY => write!(f, "LAST_ENTRY Cassandra type"),
        }
    }
}

trait AsTypedColumn {
    type T;
    fn get(col: Column) -> Result<Self::T, CassError>;
}

impl AsTypedColumn for bool {
    type T = Self;
    fn get(col: Column) -> Result<Self, CassError> {
        col.get_bool()
    }
}

impl Column {
    ///Gets the type of this column.
    pub fn get_type(&self) -> ValueType {
        unsafe { ValueType::build(cass_value_type(self.0)) }
    }

    ///Gets the inet from this column or errors if you ask for the wrong type
    pub fn get_inet(&self, mut output: Inet) -> Result<Inet, CassError> {
        unsafe { CassError::build(cass_value_get_inet(self.0, &mut output.0)).wrap(output) }
    }

    ///Gets the u32 from this column or errors if you ask for the wrong type
    pub fn get_u32(&self, mut output: u32) -> Result<u32, CassError> {
        unsafe { CassError::build(cass_value_get_uint32(self.0, &mut output)).wrap(output) }
    }

    ///Gets the i8 from this column or errors if you ask for the wrong type
    pub fn get_i8(&self, mut output: i8) -> Result<i8, CassError> {
        unsafe { CassError::build(cass_value_get_int8(self.0, &mut output)).wrap(output) }
    }

    ///Gets the i16 from this column or errors if you ask for the wrong type
    pub fn get_i16(&self, mut output: i16) -> Result<i16, CassError> {
        unsafe { CassError::build(cass_value_get_int16(self.0, &mut output)).wrap(output) }
    }

    //    pub fn get_decimal(&self, mut output: d128) -> Result<d128, CassError> {
    //        let _ = output;
    //        unimplemented!()
    //        // unsafe { CassError::build(cass_value_get_decimal(self.0, &mut output)).wrap(output) }
    //    }

    ///Gets the string from this column or errors if you ask for the wrong type
    pub fn get_string(&self) -> Result<String, CassError> {
        unsafe {
            match cass_value_type(self.0) {
                CASS_VALUE_TYPE_ASCII | CASS_VALUE_TYPE_TEXT | CASS_VALUE_TYPE_VARCHAR => {
                    let mut message = mem::zeroed();
                    let mut message_length = mem::zeroed();
                    match cass_value_get_string(self.0, &mut message, &mut message_length) {
                        CASS_OK => {
                            let slice = slice::from_raw_parts(message as *const u8,
                                                              message_length as usize);
                            Ok(str::from_utf8(slice).unwrap().to_owned())
                        }
                        err => Err(CassError::build(err)),
                    }


                }
                err => Err(CassError::build(err)),
            }
        }
    }

    ///Gets the i32 from this column or errors if you ask for the wrong type
    pub fn get_i32(&self) -> Result<i32, CassError> {
        unsafe {
            let mut output = mem::zeroed();
            CassError::build(cass_value_get_int32(self.0, &mut output)).wrap(output)
        }
    }

    ///Gets the i64 from this column or errors if you ask for the wrong type
    pub fn get_i64(&self) -> Result<i64, CassError> {
        unsafe {
            let mut output = mem::zeroed();
            CassError::build(cass_value_get_int64(self.0, &mut output)).wrap(output)
        }
    }

    ///Gets the float from this column or errors if you ask for the wrong type
    pub fn get_float(&self) -> Result<f32, CassError> {
        unsafe {
            let mut output = mem::zeroed();
            CassError::build(cass_value_get_float(self.0, &mut output)).wrap(output)
        }
    }

    ///Gets the double from this column or errors if you ask for the wrong type
    pub fn get_double(&self) -> Result<f64, CassError> {
        unsafe {
            let mut output = mem::zeroed();
            CassError::build(cass_value_get_double(self.0, &mut output)).wrap(output)
        }
    }

    ///Gets the bool from this column or errors if you ask for the wrong type
    pub fn get_bool(&self) -> Result<bool, CassError> {
        unsafe {
            let mut output = mem::zeroed();
            CassError::build(cass_value_get_bool(self.0, &mut output)).wrap(output > 0)
        }
    }

    ///Gets the uuid from this column or errors if you ask for the wrong type
    pub fn get_uuid(&self) -> Result<Uuid, CassError> {
        unsafe {
            let mut output: Uuid = mem::zeroed();
            CassError::build(cass_value_get_uuid(self.0, &mut output.0)).wrap(output)
        }
    }

    ///Gets an iterator over the map in this column or errors if you ask for the wrong type
    pub fn map_iter(&self) -> Result<MapIterator, CassError> {
        unsafe {
            match self.get_type() {
                ValueType::MAP => Ok(MapIterator(cass_iterator_from_map(self.0))),
                _ => Err(CassError::build(CassErrorTypes::LIB_INVALID_VALUE_TYPE as u32)),
            }
        }
    }

    ///Gets an iterator over the set in this column or errors if you ask for the wrong type
    pub fn set_iter(&self) -> Result<SetIterator, CassError> {
        unsafe {
            match self.get_type() {
                ValueType::SET => Ok(SetIterator(cass_iterator_from_collection(self.0))),
                _ => Err(CassError::build(1)),
            }
        }
    }

    ///Gets an iterator over the fields of the user type in this column or errors if you ask for the wrong type
    pub fn use_type_iter(&self) -> Result<UserTypeIterator, CassError> {
        unsafe {
            match self.get_type() {
                ValueType::UDT => Ok(UserTypeIterator(cass_iterator_fields_from_user_type(self.0))),
                _ => Err(CassError::build(1)),
            }
        }
    }
}
