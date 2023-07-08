# Mpv Subs Popout 
A little application that makes it possible to display mpv's subs anywhere you want.

Why? You can now watch shows in foreign languages on your second monitor and do other stuff on your main monitor while still being able to read the subs. (Thats why I built this anway)

https://github.com/sdaqo/mpv-subs-popout/assets/63876564/42beb106-ae04-4115-8262-03bc50414d8a


## Installation
This app uses mpv's ipc to get the current subs. To activate this you can either add `--input-ipc-server=/tmp/mpvsock` every time you use mpv or better add the line `input-ipc-server=/tmp/mpvsock` to your `mpv.conf` ([location of config](https://mpv.io/manual/stable/#files)).

> Currently only Linux is Supported. Support for Windows is planned.

### Arch
```sh
mkdir mpv-subs-popout && cd mpv-subs-popout

# Get the PKGBUILD
wget 'https://raw.githubusercontent.com/sdaqo/mpv-subs-popout/main/PKGBUILD'

makepkg -si
cd .. && rm -rf mpv-subs-popout
```

### Debian (or a derivative)
Download the latest .deb release from the releases. And then:

```sh
# The ./ is important else it does not install from the file.
apt install ./name_of_deb.deb
```

### Other Distros
Just download the binary release or maybe your distro has any way to install from .deb files (search it up).


## Building
To Build a plain and simple binary you will need the libs for gtk3, glib, pango and cairo (probably preinstalled if you are on a system with graphical interface). You will also need Cargo (I recommend you use [rustup](https://rustup.rs/) to install it).
```sh
cargo build --release
./target/release/mpv-subs-popout
```
To build the .deb do this:
```sh
cargo deb
cd target/debian
```


## Features
- Always works: Open mpv after the Popout or before, it detects mpv and attaches itself
- Custom Font and Font size
- Always on Top & Docked mode 
