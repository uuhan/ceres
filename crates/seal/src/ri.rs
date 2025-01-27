//! std runtime interfaces
use ceres_executor::{
    derive::{ReturnValue, Value},
    Result,
};
use ceres_sandbox::Sandbox;
use ceres_std::{vec, Vec};

type ParcelResult = Result<ReturnValue>;

/// std runtime interfaces
pub trait RuntimeInterfaces: Sized {
    /// Check if enabled
    fn enabled(&self) -> bool;

    /// Println
    fn seal_println(sandbox: &mut Sandbox, args: &[Value]) -> ParcelResult;

    /// Generate random value
    fn seal_random(sandbox: &mut Sandbox, args: &[Value]) -> ParcelResult;

    /// sha2 256
    fn seal_hash_sha2_256(sandbox: &mut Sandbox, args: &[Value]) -> ParcelResult;

    /// keccak 256
    fn seal_hash_keccak_256(sandbox: &mut Sandbox, args: &[Value]) -> ParcelResult;

    /// blake2 256
    fn seal_hash_blake2_256(sandbox: &mut Sandbox, args: &[Value]) -> ParcelResult;

    /// blake2 128
    fn seal_hash_blake2_128(sandbox: &mut Sandbox, args: &[Value]) -> ParcelResult;

    /// pack functions
    fn pack(
        &self,
    ) -> Vec<ceres_executor::derive::HostParcel<&'static str, &'static str, ceres_sandbox::Sandbox>>
    {
        vec![
            ("seal0", "seal_println", Self::seal_println),
            ("seal0", "seal_random", Self::seal_random),
            ("seal0", "seal_hash_blake2_128", Self::seal_hash_blake2_128),
            ("seal0", "seal_hash_blake2_256", Self::seal_hash_blake2_256),
            ("seal0", "seal_hash_keccak_256", Self::seal_hash_keccak_256),
            ("seal0", "seal_hash_sha2_256", Self::seal_hash_sha2_256),
        ]
    }
}

/// A dimmy runtime interfaces
pub struct NoRuntimeInterfaces;

impl RuntimeInterfaces for NoRuntimeInterfaces {
    fn enabled(&self) -> bool {
        false
    }

    fn seal_println(_sandbox: &mut Sandbox, _args: &[Value]) -> ParcelResult {
        todo!()
    }

    fn seal_random(_sandbox: &mut Sandbox, _args: &[Value]) -> ParcelResult {
        todo!()
    }

    fn seal_hash_sha2_256(_sandbox: &mut Sandbox, _args: &[Value]) -> ParcelResult {
        todo!()
    }

    fn seal_hash_keccak_256(_sandbox: &mut Sandbox, _args: &[Value]) -> ParcelResult {
        todo!()
    }

    fn seal_hash_blake2_256(_sandbox: &mut Sandbox, _args: &[Value]) -> ParcelResult {
        todo!()
    }

    fn seal_hash_blake2_128(_sandbox: &mut Sandbox, _args: &[Value]) -> ParcelResult {
        todo!()
    }
}
