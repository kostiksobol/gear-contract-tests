#![no_std]

use gstd::{msg, ActorId};


static mut USER_ID: ActorId = ActorId::zero();

#[no_mangle]
unsafe extern "C" fn init(){
    USER_ID = msg::source();
}

#[no_mangle]
unsafe extern "C" fn handle(){
    let contract2_id: ActorId = msg::load().expect("Error in decode ActorId");
    msg::send(USER_ID, b"send in handle in contract-1", 0).expect("Error in send in contract-1 in handle");
    msg::send(contract2_id, USER_ID, 0).expect("Error in send in contract-1 in handle"); // user_id as payload, since we want feedback from contract-2 too
}

#[no_mangle]
unsafe extern "C" fn handle_reply(){
    msg::send(USER_ID, b"send in handle_reply in contract-1", 0).expect("Error in send in contract-1 in handle_reply");
    msg::reply(USER_ID, 0).expect("Error in reply in contract-1 in handle_reply"); // user_id as payload, since we want feedback from contract-2 too
}