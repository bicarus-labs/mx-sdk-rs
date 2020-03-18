#![no_std]

extern crate alloc;
pub use alloc::boxed::Box;
pub use alloc::vec::Vec;
pub use alloc::string::String;

mod address;
mod err;
pub mod str_util;
pub mod serialization;

pub use address::*;
pub use err::*;
pub use serialization::*;

use core::ops::{Add, Sub, Mul, Div, Rem, Neg};
use core::ops::{AddAssign, SubAssign, MulAssign, DivAssign, RemAssign};
use core::ops::{BitAnd, BitOr, BitXor, Shr, Shl};
use core::ops::{BitAndAssign, BitOrAssign, BitXorAssign, ShrAssign, ShlAssign};

/// Interface to be used by the actual smart contract code.
/// 
/// Note: contracts and the api are not mutable.
/// They simply pass on/retrieve data to/from the protocol.
/// When mocking the blockchain state, we use the Rc/RefCell pattern 
/// to isolate mock state mutability from the contract interface.
pub trait ContractHookApi<BigInt, BigUint> {

    fn get_owner(&self) -> Address;

    fn get_caller(&self) -> Address;

    fn get_balance(&self, address: &Address) -> BigUint;

    fn get_own_balance(&self) -> BigUint {
        self.get_balance(&self.get_owner())
    }
    
    fn storage_store(&self, key: &StorageKey, value: &Vec<u8>);

    fn storage_load(&self, key: &StorageKey) -> Vec<u8>;

    fn storage_store_bytes32(&self, key: &StorageKey, value: &[u8; 32]);
    
    fn storage_load_bytes32(&self, key: &StorageKey) -> [u8; 32];

    fn storage_store_big_uint(&self, key: &StorageKey, value: &BigUint);
    
    fn storage_load_big_uint(&self, key: &StorageKey) -> BigUint;

    fn storage_store_big_int(&self, key: &StorageKey, value: &BigInt);
    
    fn storage_load_big_int(&self, key: &StorageKey) -> BigInt;

    fn storage_store_i64(&self, key: &StorageKey, value: i64);
    
    fn storage_load_i64(&self, key: &StorageKey) -> Option<i64>;
    
    fn get_call_value_big_uint(&self) -> BigUint;

    fn send_tx(&self, to: &Address, amount: &BigUint, message: &str);

    fn async_call(&self, to: &Address, amount: &BigUint, data: &str);

    fn get_gas_left(&self) -> i64;

    fn sha256(&self, data: &Vec<u8>) -> [u8; 32];

    fn keccak256(&self, data: &Vec<u8>) -> [u8; 32];
}

/// Interface to only be used by code generated by the macros.
/// The smart contract code doesn't have access to these methods directly.
pub trait ContractIOApi<BigInt, BigUint> {

    fn get_num_arguments(&self) -> i32;

    fn check_num_arguments(&self, expected: i32) -> bool {
        let nr_args = self.get_num_arguments();
        if nr_args == expected + 1 {
            let callback_name_arg = self.get_argument_vec(nr_args - 1);
            self.finish_vec(&callback_name_arg); // callback method argument
            return true;
        }
        if nr_args != expected {
            self.signal_error("wrong number of arguments");
            return false;
        }
        return true;
    }

    fn check_not_payable(&self) -> bool;

    fn get_argument_vec(&self, arg_index: i32) -> Vec<u8>;

    fn get_argument_bytes32(&self, arg_index: i32) -> [u8; 32];
    
    fn get_argument_address(&self, arg_index: i32) -> Address {
        self.get_argument_bytes32(arg_index).into()
    }
    
    fn get_argument_big_int(&self, arg_id: i32) -> BigInt;

    fn get_argument_big_uint(&self, arg_id: i32) -> BigUint;
    
    fn get_argument_i64(&self, arg_id: i32) -> i64;
    
    fn finish_vec(&self, v: &Vec<u8>);

    fn finish_bytes32(&self, bytes: &[u8; 32]);

    fn finish_big_int(&self, b: &BigInt);

    fn finish_big_uint(&self, b: &BigUint);

    fn finish_i64(&self, value: i64);

    #[inline]
    fn signal_error(&self, message: &str) {
        self.signal_error_raw(message.as_ptr(), message.len());
    }

    fn signal_error_raw(&self, message_ptr: *const u8, message_len: usize);

    fn write_log(&self, topics: &[[u8;32]], data: &[u8]);
}

/// Definition of the BigUint type required by the API.
/// The API doesn't care about the actual BigInt implementation.
/// The Arwen VM provides an implementation directly in the protocol.
/// For debugging we use a different implementation, based on Rust's BigInt.
/// 
/// Since most values in smart contracts will not be signed, as well as for safety,
/// most of the functionality if provided for unsigned integers.
pub trait BigUintApi: 
    Sized +
    From<i64> +
    From<i32> +
    Clone +
    Add<Output=Self> + 
    AddAssign + 
    Sub<Output=Self> + 
    SubAssign +
    Mul<Output=Self> +
    MulAssign +
    Div<Output=Self> +
    DivAssign +
    Rem<Output=Self> +
    RemAssign +
    BitAnd<Output=Self> +
    BitAndAssign +
    BitOr<Output=Self> +
    BitOrAssign +
    BitXor<Output=Self> +
    BitXorAssign +
    Shr<i32, Output=Self> +
    ShrAssign<i32> +
    Shl<i32, Output=Self> +
    ShlAssign<i32> +
    PartialEq<Self> +
    Eq +
    PartialOrd<Self> +
    Ord +
    PartialEq<i64> +
    PartialOrd<i64> +
{
    fn byte_length(&self) -> i32;

    fn copy_to_slice_big_endian(&self, slice: &mut [u8]) -> i32;

    fn copy_to_array_big_endian_pad_right(&self, target: &mut [u8; 32]) {
        let byte_len = self.byte_length() as usize;
        if byte_len > 32 {
            panic!();
        }
        self.copy_to_slice_big_endian(&mut target[32 - byte_len ..]);
    }

    fn to_bytes_be(&self) -> Vec<u8>;

    fn to_bytes_be_pad_right(&self, nr_bytes: usize) -> Vec<u8>;

    // only needed at compilation, value will never be used
    fn phantom() -> Self;
}

// BigInt sign.
pub enum Sign {
    Minus,
    NoSign,
    Plus,
}

/// Definition of the BigInt type required by the API.
pub trait BigIntApi<BigUint>: 
        Sized +
        From<BigUint> +
        From<i64> +
        From<i32> +
        Clone +
        Add<Output=Self> + 
        AddAssign + 
        Sub<Output=Self> + 
        SubAssign +
        Mul<Output=Self> +
        MulAssign +
        Div<Output=Self> +
        DivAssign +
        Rem<Output=Self> +
        RemAssign +
        Neg +
        PartialEq<Self> +
        Eq +
        PartialOrd<Self> +
        Ord +
        PartialEq<i64> +
        PartialOrd<i64> +
{
    fn abs_uint(&self) -> BigUint;

    fn sign(&self) -> Sign;

    fn to_signed_bytes_be(&self) -> Vec<u8>;

    // only needed at compilation, value will never be used
    fn phantom() -> Self;
}

/// CallableContract is the means by which the debugger calls methods in the contract.
pub trait CallableContract {
    fn call(&self, fn_name: &'static str);

    fn clone_contract(&self) -> Box<dyn CallableContract>;
}

/// Handy way of casting to a contract proxy trait.
/// Would make more sense to be in elrond-wasm-derive, but Rust "cannot export macro_rules! macros from a `proc-macro` crate type currently".
#[macro_export]
macro_rules! contract_proxy {
    ($s:expr, $address:expr, $proxy_trait:ident) => {
      $s.contract_proxy($address) as Box<dyn $proxy_trait<BigInt, BigUint>>
  };
}
