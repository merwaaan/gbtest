#!/bin/bash

mkdir -p _build

if [ "$1" == "gb" ]; then
  ../tools/_gbdk/bin/lcc -Wa-l -Wl-m -Wl-j -o _build/client.gb src/main.c
elif [ "$1" == "gbc" ]; then
  # Define GAMEBOYCOLOR to distinguish platforms
  ../tools/_gbdk/bin/lcc -DGAMEBOYCOLOR -Wa-l -Wl-m -Wl-j -Wm-yC -o _build/client.gbc src/main.c
else
  echo "no platform specified"
  exit 1
fi

