# Missing Semester Window Manager MSWM
Missing Semester Window Manager is an attempt at passing the course "Missing Semester" by implementing a custom window manager in Rust based on x11rb.
The aim is to remain minimalistic in design with a clean implementation of essentials as well as functionalities and extendability according to our personal preferences.

## Requirements
This project is designed for Linux.
While it may work on Mac-Systems, we cannot guarantee for full feature availability.
MS Windows is not supported.

Installation of xserver-xephyr and xinit may be required.
The correct dependencies can be installed via apt-get:
```bash
sudo apt-get install xserver-xephyr
sudo apt-get install xinit
```
(If you are using a system that does not have aptitude, you probably do not need help with this...)

## Usage
Testing a window manager before using it as daily driver is important.
Therefor, we include a script (run.sh) that lets you play around with MSWM in a safe environment without having to change your system defaults.
To exectue it run:
```bash
./run.sh
```
It opens a virtual desktop with a console, a watch widget and some goggly eyes.
Windows can be dragged with `super-key + left mouse` and resized with `super-key + right mouse`.

Further features are currently in development. [...]

## License
For legal reasons, MSWM is currently *not* Open Source.
That said, if parts of the source code reappear in other projects in the future, it may be outside of our domain to pursue legal action. ¯\_(ツ)_/¯
