# Bang

## TODO
- add hooks to clean up child processes on exit
  - I think this is being cleaned up already
- figure out layout.
  - Lazy docker layout would be neat
  - left side is list of all running commands
  - main box of program input
- figure out config format
- would be nice to add way to kill/restart certain commands without killing all of them
- make separate thread for reading input (maybe one for rendering?)
- Handle errors properly
- Make message types to pass around threads
- Log scrolling
  - basic scrolling works but support user scrolling


## resources
- https://www.nikbrendler.com/rust-process-communication/
- https://github.com/DevinR528/rumatui/blob/main/src/main.rs
- https://github.com/lemunozm/termchat
- https://github.com/ClementTsang/bottom