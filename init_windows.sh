#!/bin/bash

# GBDK
#
# NOTE: opening the GBDK folder with Visual Studio should set up the project with CMake

curl -L https://github.com/gbdk-2020/gbdk-2020/releases/latest/download/gbdk-win.zip --output gbdk.zip
7z x gbdk.zip -otools/
rm gbdk.zip
mv tools/gbdk tools/_gbdk
