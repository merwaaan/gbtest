#!/bin/bash

emulator=$1
game=$2
instances=$3

for ((i = 0; i < instances; i++)); do
  $emulator $game &
  sleep 1s
done