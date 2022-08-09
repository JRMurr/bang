# Bang

Run multiple commands in parallel and swap between the output logs of each.


## Quick start

### Config

Add the following to `~/.config/bang/bang.toml`
```toml
commands = [
    {command = "ping -i 0.1 localhost"},
    {command = "ping 1.1.1.1"},
    {command = "ls", name = "list config dir", running_dir="~/.config/"},
]
```

Then run `bang`. Type `?` to see help.

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
