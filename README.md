# Mpv Subs Popout 
A little application that makes it possible to display mpv's subs anywhere you want.

Why? You can now watch shows in foreign languages on your second monitor and do other stuff on your main monitor while still being able to read the subs. (Thats why I built this anway)

https://github.com/sdaqo/mpv-subs-popout/assets/63876564/42beb106-ae04-4115-8262-03bc50414d8a


## Installation
This app uses mpv's ipc to get the current subs. To activate this you can either add `--input-ipc-server=/tmp/mpvsock` every time you use mpv or better add the line `input-ipc-server=/tmp/mpvsock` to your `mpv.conf` ([location of config](https://mpv.io/manual/stable/#files)).

## Features
- Always works: Open mpv after the Popout or before, it detects mpv and attaches itself
- Custom Font and Font size
- Always on Top & Docked mode 
