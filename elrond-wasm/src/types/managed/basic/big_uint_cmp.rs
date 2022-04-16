use core::cmp::Ordering;

use crate::api::{BigIntApi, ManagedTypeApi, StaticVarApiImpl};

use super::BigUint;

impl<M: ManagedTypeApi> PartialEq for BigUint<M> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        M::managed_type_impl()
            .bi_cmp(self.handle, other.handle)
            .is_eq()
    }
}

impl<M: ManagedTypeApi> Eq for BigUint<M> {}

impl<M: ManagedTypeApi> PartialOrd for BigUint<M> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<M: ManagedTypeApi> Ord for BigUint<M> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        M::managed_type_impl().bi_cmp(self.handle, other.handle)
    }
}

fn cmp_i64<M: ManagedTypeApi>(bi: &BigUint<M>, other: i64) -> Ordering {
    let api = M::managed_type_impl();
    if other == 0 {
        match api.bi_sign(bi.handle) {
            crate::api::Sign::Plus => Ordering::Greater,
            crate::api::Sign::NoSign => Ordering::Equal,
            crate::api::Sign::Minus => Ordering::Less,
        }
    } else {
        let handle = M::static_var_api_impl().next_handle();
        M::managed_type_impl().bi_set_int64(handle, other as i64);
        api.bi_cmp(bi.handle, handle)
    }
}

macro_rules! partial_eq_and_ord {
    ($small_int_type:ident) => {
        impl<M: ManagedTypeApi> PartialEq<$small_int_type> for BigUint<M> {
            #[inline]
            fn eq(&self, other: &$small_int_type) -> bool {
                cmp_i64(self, *other as i64).is_eq()
            }
        }

        impl<M: ManagedTypeApi> PartialOrd<$small_int_type> for BigUint<M> {
            #[inline]
            fn partial_cmp(&self, other: &$small_int_type) -> Option<Ordering> {
                Some(cmp_i64(self, *other as i64))
            }
        }
    };
}

partial_eq_and_ord! {i32}
partial_eq_and_ord! {i64}
partial_eq_and_ord! {u32}
partial_eq_and_ord! {u64}
