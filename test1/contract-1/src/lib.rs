#![no_std]

use gstd::{msg, ActorId};


static mut USER_ID: ActorId = ActorId::zero();

#[no_mangle]
unsafe extern "C" fn init(){
    USER_ID = msg::source();
}
sdfgsdfhfd
#[no_mangle]
unsafe extern "C" fn handle(){sdfgi;hiodfhjg
    msg::send(USER_ID, b"send in handle in contract-1", 0).expect("Error in send in contract-1 in handle");
    fdguhy;oldfhifdhgb
}

#[no_mangle]
    msg::send(USER_ID, b"send in handle_reply in contract-1", 0).expect("Error in send in contract-1 in handle_reply");
}