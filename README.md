# Mpv Subs Popout 
A little application that makes it possible to display mpv's subs anywhere you want, it also includes translation (potentially for language learning).

Why? You can now watch shows in foreign languages on your second monitor and do other stuff on your main monitor while still being able to read the subs. (Thats why I built this anway)



https://github.com/sdaqo/mpv-subs-popout/assets/63876564/063e52e4-dddd-46e6-92f1-af6a486d3fe0


## Installation

### Mpv Configuration
This app uses mpv's ipc to get the current subs so you have to activate it:

For Linux Add `input-ipc-server=/tmp/mpvsock` to your [mpv config](https://mpv.io/manual/stable/#files-~/-config/mpv).

For Windows Add `input-ipc-server=\\.\pipe\mpvsock` to your [mpv config](https://mpv.io/manual/stable/#files-on-windows). 



### Arch
With Aur helper:
```sh
yay -S mpv-subs-popout 
```
If you do not have any aur helpers:

```sh
mkdir mpv-subs-popout && cd mpv-subs-popout

# Get the PKGBUILD
wget 'https://raw.githubusercontent.com/sdaqo/mpv-subs-popout/main/PKGBUILD'

makepkg -si
cd .. && rm -rf mpv-subs-popout
```

### Debian (or a derivative)
Download the latest .deb release from the [releases page](https://github.com/sdaqo/mpv-subs-popout/releases/latest). And then:

```sh
# The ./ is important else it does not install from the file.
apt install ./name_of_deb.deb
```

### Other Distros
Just download the binary release from the [releases page](https://github.com/sdaqo/mpv-subs-popout/releases/latest) or maybe your distro has any way to install from .deb files (search it up).

### Windows
Download the latest release from the [releases page](https://github.com/sdaqo/mpv-subs-popout/releases/latest).

Unzip the .zip and start `mpv-subs-popout.exe` to start the application. 


## Building

### Linux
To Build a plain and simple binary you will need the libs for gtk3, glib, pango and cairo (probably preinstalled if you are on a system with graphical interface). You will also need Cargo (I recommend you use [rustup](https://rustup.rs/) to install it).
```sh
git clone "https://github.com/sdaqo/mpv-subs-popout" && cd mpv-subs-popout

cargo build --release

# To run it
./target/release/mpv-subs-popout
```
To build the .deb do this:
```sh
cargo deb
cd target/debian
```

### Windows
Install rust and Cargo with [rustup](https://rustup.rs/).

**All the following commands are executed in the Windows Powershell**

1. Install [Chocolately](https://chocolatey.org/). This makes it easier to install the building requirements.
```powershell
Set-ExecutionPolicy Bypass -Scope Process -Force; iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))
```

2. Install build requirements
```powershell
choco install git
choco install mysys2
choco install visualstudio2022buildtools
choco install visualstudio2022-workload-vctools
choco install python

# Restart Shell here
``` 

From here you might also want to follow the [ci build steps](https://github.com/sdaqo/mpv-subs-popout/blob/c98c1702a798a7a92464e04784d60b1e24f77008/.github/workflows/rust.yml#L10), they lead you to the same *.zip as in the release builds.

3. Install gvsbuild
```powershell
python -m pip install --user pipx
python -m pipx ensurepath

# Restart Shell here

pipx install gvsbuild
```

4. Build Gtk3 and add it to your Environment Variables
```powershell
# This will take a while
gvsbuild build gtk3


$env:Path = "C:\gtk-build\gtk\x64\release\bin;" + $env:Path
$env:LIB = "C:\gtk-build\gtk\x64\release\lib;" + $env:LIB
$env:INCLUDE = "C:\gtk-build\gtk\x64\release\include;C:\gtk-build\gtk\x64\release\include\cairo;C:\gtk-build\gtk\x64\release\include\glib-2.0;C:\gtk-build\gtk\x64\release\include\gobject-introspection-1.0;C:\gtk-build\gtk\x64\release\lib\glib-2.0\include;" + $env:INCLUDE
```

5. Build Rust application
```powershell
# Clone Repo
git clone "https://github.com/sdaqo/mpv-subs-popout"
cd 'mpv-subs-popout'

# Set toolchain
rustup default stable-msvc

# Build
cargo build --release

# To run it
./target/release/mpv-subs-popout.exe
```

## Features
- Full custom translator widget (Google, DeeplX)
- Auto translation in seperate line
- Always works: Open mpv after the Popout or before, it detects mpv and attaches itself
- Custom Font and Font size
- Always on Top & Docked mode
- Custom bg color
- Custom text color
- Linux & Windows Support

**Note 1: Some features like docked or always on top do not work under wayland!**

**Note 2: When setting the api key for the /v2/translate endpoint for DeeplX do it like this (with space) `[yourAccessToken] [yourAuthKey]`**

### Planned
- Dictionary for looking up words
- Any other suggested features (Make an Issue)
