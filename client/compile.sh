#!/bin/bash

mkdir -p _build

# GameBoy
 ../tools/_gbdk/bin/lcc -Wa-l -Wl-m -Wl-j -o _build/client.gb src/main.c

# GameBoy Color
#
# Define GAMEBOYCOLOR to distinguish platforms
../tools/_gbdk/bin/lcc -DGAMEBOYCOLOR -Wa-l -Wl-m -Wl-j -Wm-yC -o _build/client.gbc src/main.c
