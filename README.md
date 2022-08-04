# Bang

Run multiple commands in parallel and hop between the output logs of each

## TODO
- add hooks to clean up child processes on exit
  - I think this is being cleaned up already
- would be nice to add way to kill/restart certain commands without killing all of them
- make separate thread for reading input (maybe one for rendering?)
- Handle errors properly
- Make message types to pass around threads
- Log scrolling
  - basic scrolling works but support user scrolling
- handle errors of commands gracefully
  - might be nice to use https://docs.rs/subprocess/latest/subprocess/index.html since it has redirection


## resources
- https://www.nikbrendler.com/rust-process-communication/
- https://github.com/DevinR528/rumatui/blob/main/src/main.rs
- https://github.com/lemunozm/termchat
- https://github.com/ClementTsang/bottom