// Code generated by the multiversx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            3
// Async Callback (empty):               1
// Total number of exported functions:   5

#![no_std]
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!(leaking);
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    str_repeat
    (
        repeat
        getByteArrayLength
        getByteArray
    )
}

multiversx_sc_wasm_adapter::empty_callback! {}
