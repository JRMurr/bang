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

- re-work actions
- Handle errors properly (maybe just use color_erye)
- Log scrolling
  - On scroll it stops following the output
  - add callback to (set scroll)[https://docs.rs/cursive/0.19.0/cursive/views/struct.ScrollView.html#method.set_scroll_strategy] if at bottom?
- tests :(
  - can get by with mostly config/path reading stuff


## resources
- https://github.com/DevHyperCoder/rbmenu-tui
- https://github.com/tokio-rs/console/tree/main/tokio-console 
- https://www.nikbrendler.com/rust-process-communication/
- https://github.com/DevinR528/rumatui/blob/main/src/main.rs
- https://github.com/lemunozm/termchat
- https://github.com/ClementTsang/bottom
- https://github.com/kdheepak/taskwarrior-tui
