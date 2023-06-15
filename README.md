# gear-contract-tests

1. Build: "cargo build --release"
2. Upload (upload on gear idea): click "Programs", "Upload program", select your program .opt.wasm from ./target/wasm32-unknown-unknown/release/,
then enter name of program, payload can be empty

In case of two smart contracts, you'll need to upload both of them, and when send messages to contract-1 as payload write contract-2_id

## test1:
It tests what makes reply in handle_reply (reply in handle_reply in contract-1 just reply to contract-2 and contract-2 process that reply in its handle_reply)

## test2:
Try to run handle_signal (but got nothing, gear idea just sends comment about panic)

## test3:
Try to run handle_signal in case of two smart contracts (when panic occurs in contract-2 it somehow makes reply to contract-1, and contract-1 process it in its handle_reply)
