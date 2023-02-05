# Missing Semester Window Manager MSWM
Missing Semester Window Manager is an attempt at passing the course "Missing Semester" by implementing a custom window manager in Rust based on `x11rb`.
The aim is to remain minimalistic in design with a clean implementation of essentials as well as functionalities according to our (mostly Keanus') personal preferences.

~ Keanu Pöschko, Peter Pfeiffer

## Installation

### Requirements
This project is designed for Linux.
While it may work on Mac-Systems, we cannot guarantee for full feature availability.
MS Windows is not supported.

Installation of `xserver-xephyr`, `xinit` and `libxkbcommon-x11-dev` may be required.
If you are using aptitude, the correct dependencies can be installed via apt-get:
```bash
sudo apt-get install xserver-xephyr xinit libxkbcommon-x11-dev
```
(If you are using a system that does not have aptitude, you probably do not need help with this...)

### Rust
We do not ship binaries, so you will have to compile it yourself. For this, cargo is required.
It can be installed with:
```bash
curl https://sh.rustup.rs -sSf | sh
```
To compile the project, execute
```bash
cargo build
```
in the directory `mswm`. Cargo automatically takes care of any crate-based dependencies.

## Usage

### Testing
Testing a window manager beforehand is important.
Therefor, we include a script (`run.sh`) that lets you play around with MSWM in a safe environment without having to change your system defaults.
It also takes care of compiling the project.
To exectue it run:
```bash
./run.sh
```
It opens a virtual desktop (xephyr) with a console, a some windows for you to play around with.
The mouse and keyboard can be captured/released with `ctrl + shift`.

While we are proud of what we have created thus far, we would not yet recommend you to switch to MSWM on your main system ...

### Controls
Windows can be dragged by moving the cursor while pressing `super-key + left mouse`.
Resizing works similarly with `super-key + right mouse`.
`super-key + C` applies a Fibonacci layout to the windows.

## License
For legal reasons, MSWM is currently *not* under any Open Source license.
That said, if parts of the source code reappear in other projects in the future, it may be outside of our domain to pursue legal action. ¯\_(ツ)_/¯
