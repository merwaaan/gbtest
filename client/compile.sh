#!/bin/bash

mkdir -p _build

../tools/_gbdk/bin/lcc -Wa-l -Wl-m -Wl-j -o _build/client.gb src/main.c
