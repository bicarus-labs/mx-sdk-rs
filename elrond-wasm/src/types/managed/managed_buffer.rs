use elrond_codec::{NestedEncodeOutput, TryStaticCast};

use crate::{
    api::{Handle, ManagedTypeApi},
    types::BoxedBytes,
};

/// A byte buffer managed by an external API.
pub struct ManagedBuffer<M: ManagedTypeApi> {
    pub(crate) handle: Handle,
    pub(crate) api: M,
}

impl<M: ManagedTypeApi> ManagedBuffer<M> {
    pub fn new_empty(api: M) -> Self {
        ManagedBuffer {
            handle: api.mb_new_empty(),
            api: api.clone(),
        }
    }

    pub fn new_from_bytes(api: M, bytes: &[u8]) -> Self {
        ManagedBuffer {
            handle: api.mb_new_from_bytes(bytes),
            api: api.clone(),
        }
    }

    pub(crate) fn new_from_raw_handle(api: M, handle: Handle) -> Self {
        ManagedBuffer { handle, api }
    }

    pub fn len(&self) -> usize {
        self.api.mb_len(self.handle)
    }

    pub fn overwrite(&self, value: &[u8]) {
        self.api.mb_overwrite(self.handle, value);
    }

    pub fn append(&mut self, other: &ManagedBuffer<M>) {
        self.api.mb_append(self.handle, other.handle);
    }

    pub fn append_bytes(&mut self, slice: &[u8]) {
        self.api.mb_append_bytes(self.handle, slice);
    }

    pub fn to_boxed_bytes(&self) -> BoxedBytes {
        self.api.mb_to_boxed_bytes(self.handle)
    }
}

impl<M: ManagedTypeApi> Clone for ManagedBuffer<M> {
    fn clone(&self) -> Self {
        // TODO: Optimize!!!
        ManagedBuffer {
            handle: self.api.mb_new_from_bytes(self.to_boxed_bytes().as_slice()),
            api: self.api.clone(),
        }
    }
}

impl<M: ManagedTypeApi> TryStaticCast for ManagedBuffer<M> {}

impl<M: ManagedTypeApi> NestedEncodeOutput for ManagedBuffer<M> {
    fn write(&mut self, bytes: &[u8]) {
        self.append_bytes(bytes);
    }

    fn push_specialized<T: TryStaticCast>(&mut self, value: &T) -> bool {
        if let Some(managed_buffer) = value.try_cast_ref::<ManagedBuffer<M>>() {
            self.append(managed_buffer);
            true
        } else {
            false
        }
    }
}
