#![no_std]

use gstd::{msg, exec, ActorId};


static mut USER_ID: ActorId = ActorId::zero();

#[no_mangle]
unsafe extern "C" fn init(){
    USER_ID = msg::source();
}

#[no_mangle]
unsafe extern "C" fn handle(){
    let contract2_id: ActorId = msg::load().expect("Error in decode ActorId");
    msg::send(USER_ID, b"send in handle in contract-1", 0).expect("Error in send in contract-1 in handle");

    exec::system_reserve_gas(1_000_000_000).expect("Error in system_reserve_gas in handle"); // just in case, but panic in contract-2
    msg::send(contract2_id, b"some payload", 0).expect("Error in send in contract-1 in handle"); 
}

#[no_mangle]
unsafe extern "C" fn handle_signal(){
    msg::send(USER_ID, b"send in handle_signal in contract-1", 0).expect("Error in send in contract-1 in handle_signal");

    // check what does reply in handle_signal
    msg::reply(b"reply in handle_signal in contract-1", 0).expect("Error in reply in contract-1 in handle_signal");
}



// just in case, but look what happens (in contract-2 panic! makes reply !!!)
#[no_mangle]
unsafe extern "C" fn handle_reply(){
    msg::send(USER_ID, b"send in handle_reply in contract-1", 0).expect("Error in send in contract-1 in handle_reply");
    msg::reply(b"some payload", 0).expect("Error in reply in contract-1 in handle_reply"); 
}

// just in case
#[no_mangle]
unsafe extern "C" fn my_handle_signal(){
    msg::send(USER_ID, b"send in handle_signal in contract-1", 0).expect("Error in send in contract-1 in handle_signal");

    // check what does reply in handle_signal
    msg::reply(b"reply in handle_signal in contract-1", 0).expect("Error in reply in contract-1 in handle_signal");
}