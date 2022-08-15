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
- Handle errors properly
- Log scrolling
  - Need to fork the built-in list widget to get access to `ListState.output`/(getItemBounds)[https://github.com/fdehau/tui-rs/blob/fafad6c96109610825aad89c4bba5253e01101ed/src/widgets/list.rs#L131]
  - The issue is right now scolling works by selected the last/next line. If the selected line is the last line (in auto-scroll mode) we should select the line right before the top line on the screen so we see older outputs
- handle errors of commands gracefully
- tests :(
  - can get by with mostly config/path reading stuff


## resources
- https://www.nikbrendler.com/rust-process-communication/
- https://github.com/DevinR528/rumatui/blob/main/src/main.rs
- https://github.com/lemunozm/termchat
- https://github.com/ClementTsang/bottom
- https://github.com/kdheepak/taskwarrior-tui
