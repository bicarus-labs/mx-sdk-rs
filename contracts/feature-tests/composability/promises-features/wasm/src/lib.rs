// Code generated by the mx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            7
// Async Callback (empty):               1
// Promise callbacks:                    2
// Total number of exported functions:  11

#![no_std]
#![feature(alloc_error_handler, lang_items)]

mx_sc_wasm_adapter::allocator_declaration!();
mx_sc_wasm_adapter::panic_handler_declaration!();

mx_sc_wasm_adapter::endpoints! {
    promises_features
    (
        forward_promise_accept_funds
        forward_promise_retrieve_funds
        callback_data
        callback_data_at_index
        clear_callback_data
        promise_raw_single_token
        promise_raw_multi_transfer
        retrieve_funds_callback
        the_one_callback
    )
}

mx_sc_wasm_adapter::empty_callback! {}
