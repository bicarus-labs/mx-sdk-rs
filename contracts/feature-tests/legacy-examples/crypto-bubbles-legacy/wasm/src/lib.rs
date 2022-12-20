// Code generated by the mx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            6
// Async Callback (empty):               1
// Total number of exported functions:   8

#![no_std]
#![feature(alloc_error_handler, lang_items)]

mx_sc_wasm_adapter::allocator_declaration!();
mx_sc_wasm_adapter::panic_handler_declaration!();

mx_sc_wasm_adapter::endpoints! {
    crypto_bubbles_legacy
    (
        topUp
        withdraw
        joinGame
        rewardWinner
        rewardAndSendToWallet
        balanceOf
    )
}

mx_sc_wasm_adapter::empty_callback! {}
