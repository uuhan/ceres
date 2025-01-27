//! Derive traits
pub use crate::{
    builder::Builder,
    func::HostFuncType,
    instance::Instance,
    memory::Memory,
    value::{ReturnValue, Type, Value},
};

/// Host function parcel
pub type HostParcel<M, F, T> = (M, F, HostFuncType<T>);
