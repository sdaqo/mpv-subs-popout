# mpvipc

A small library which provides bindings to control existing mpv instances through sockets.

To make use of this library, please make sure mpv is started with the following option:
`
$ mpv --input-ipc-server=/tmp/mpv.sock --idle ...
`

## Dependencies

- `mpv`
- `cargo` (makedep)

## Install

- [Cargo](https://crates.io/crates/mpvipc)

You can use this package with cargo.

## Example

Make sure mpv is started with the following option:
`
$ mpv --input-ipc-server=/tmp/mpv.sock --idle
`

Here is a small code example which connects to the socket /tmp/mpv.sock and toggles playback.

```rust
extern crate mpvipc;

use mpvipc::*;
use std::sync::mpsc::channel;

fn main() {
    let mpv = Mpv::connect("/tmp/mpv.sock").unwrap();
    let paused: bool = mpv.get_property("pause").unwrap();
    mpv.set_property("pause", !paused).expect("Error pausing");
}
```

For a more extensive example and proof of concept, see project [mpvc](https://gitlab.com/mpv-ipc/mpvc-rs).

## Bugs / Ideas

Check out the [Issue Tracker](https://gitlab.com/mpv-ipc/mpvipc/issues)
