#![no_std]

use gstd::{msg, exec, ActorId};

// for feedback from handle_signal
static mut USER_ID: ActorId = ActorId::zero();

#[no_mangle]
unsafe extern "C" fn init(){
    USER_ID = msg::source();
}

#[no_mangle]
unsafe extern "C" fn handle(){
    msg::send(USER_ID, b"send in handle in contract-2", 0).expect("Error in send in contract-2 in handle");
    // msg::reply(b"some reply", 0).expect("Error in reply in contract-2 in handle");

    exec::system_reserve_gas(1_000_000_000).expect("Error in system_reserve_gas in handle"); // more than enough
    panic!("Intended panic in contract-2");
}

#[no_mangle]
unsafe extern "C" fn handle_signal(){
    msg::send(USER_ID, b"send in handle_signal in contract-2", 0).expect("Error in send in contract-2 in handle_signal");

    // check what does reply in handle_signal
    msg::reply(b"some payload", 0).expect("Error in reply in contract-2 in handle_signal");
}


// just in case
#[no_mangle]
unsafe extern "C" fn handle_reply(){
    msg::send(USER_ID, b"send in handle_reply in contract-2", 0).expect("Error in send in contract-2 in handle_reply");
    
    // you'll get infinite replies loop (contract-1 reply contract-2, and then contract-2 reply contract-1)
    // msg::reply(b"some not important payload", 0).expect("Error in reply in contract-2 in handle_reply"); 
}

// just in case
#[no_mangle]
unsafe extern "C" fn my_handle_signal(){
    msg::send(USER_ID, b"send in handle_signal in contract-2", 0).expect("Error in send in contract-2 in handle_signal");

    // check what does reply in handle_signal
    msg::reply(b"some payload", 0).expect("Error in reply in contract-2 in handle_signal");
}