// Code generated by the mx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           15
// Async Callback (empty):               1
// Total number of exported functions:  17

#![no_std]
#![feature(alloc_error_handler, lang_items)]

mx_sc_wasm_adapter::allocator_declaration!();
mx_sc_wasm_adapter::panic_handler_declaration!();

mx_sc_wasm_adapter::endpoints! {
    payable_features
    (
        echo_call_value
        payment_multiple
        payment_array_3
        payable_any_1
        payable_any_2
        payable_any_3
        payable_any_4
        payable_egld_1
        payable_egld_2
        payable_egld_3
        payable_egld_4
        payable_token_1
        payable_token_2
        payable_token_3
        payable_token_4
    )
}

mx_sc_wasm_adapter::empty_callback! {}
