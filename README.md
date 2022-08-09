# Bang

Run multiple commands in parallel and hop between the output logs of each

## TODO
- make separate thread for reading input (maybe one for rendering?)
- Handle errors properly
- Log scrolling
  - basic scrolling works but support user scrolling
- handle errors of commands gracefully
- tests :(
  - can get by with mostly config/path reading stuff


## resources
- https://www.nikbrendler.com/rust-process-communication/
- https://github.com/DevinR528/rumatui/blob/main/src/main.rs
- https://github.com/lemunozm/termchat
- https://github.com/ClementTsang/bottom
- https://github.com/kdheepak/taskwarrior-tui
