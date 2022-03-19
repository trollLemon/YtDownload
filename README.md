# YtDownload
A GUIApp to download youtube videos

## Installation & Usage

### Windows
You have to run it on the nightly channel:
```bash
$ cargo +nightly run
```
If you don't have the nightly channel yet run:
```bash
$ rustup default nightly
```

### Raspberry Pi

**Give up !** You'll end up with a [hard to solve error](https://raspberrypi.stackexchange.com/questions/61078/qt-applications-dont-work-due-to-libegl): "libEGL warning: DRI2: failed to authenticate"

But here are the solutions to the errors to get to that point:(install OpenSSL, GTK, Glib)
```bash
$ sudo apt install libssl-dev
$ sudo apt-get install libgtk-3-dev
$ sudo apt-get install libglib2.0-dev
```
