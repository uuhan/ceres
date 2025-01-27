//! WASM value
use core::convert::Into;

/// Value types
#[derive(Clone)]
pub enum Type {
    I32,
    I64,
    F32,
    F64,
}

/// Custom value
#[derive(Clone, Copy)]
pub enum Value {
    I32(i32),
    I64(i64),
    F32(u32),
    F64(u64),
}

impl Value {
    /// try convert to i32
    pub fn as_i32(self) -> i32 {
        match self {
            Value::I32(v) => v,
            Value::I64(v) => v as i32,
            Value::F32(v) => v as i32,
            Value::F64(v) => v as i32,
        }
    }

    /// try convert to i64
    pub fn as_i64(self) -> i64 {
        match self {
            Value::I32(v) => v as i64,
            Value::I64(v) => v,
            Value::F32(v) => v as i64,
            Value::F64(v) => v as i64,
        }
    }

    /// try convert to u32
    pub fn as_u32(self) -> u32 {
        match self {
            Value::I32(v) => v as u32,
            Value::I64(v) => v as u32,
            Value::F32(v) => v,
            Value::F64(v) => v as u32,
        }
    }

    /// try convert to u64
    pub fn as_u64(self) -> u64 {
        match self {
            Value::I32(v) => v as u64,
            Value::I64(v) => v as u64,
            Value::F32(v) => v as u64,
            Value::F64(v) => v,
        }
    }
}

impl Into<i32> for Value {
    fn into(self) -> i32 {
        self.as_i32()
    }
}

impl Into<i64> for Value {
    fn into(self) -> i64 {
        self.as_i64()
    }
}

impl Into<u32> for Value {
    fn into(self) -> u32 {
        self.as_u32()
    }
}

impl Into<u64> for Value {
    fn into(self) -> u64 {
        self.as_u64()
    }
}

impl Into<ReturnValue> for Value {
    fn into(self) -> ReturnValue {
        ReturnValue::Value(self)
    }
}

/// Value for return
#[derive(Clone)]
pub enum ReturnValue {
    Unit,
    Value(Value),
}
