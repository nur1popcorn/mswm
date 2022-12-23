#!/bin/sh

cargo build

unset XDG_SEAT
XEPHYR=$(whereis -b Xephyr | cut -f2 -d' ')
xinit ./xinitrc -- "$XEPHYR" :100 -screen 800x600
