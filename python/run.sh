#!/bin/bash

dir=$(readlink -f $(dirname "$0"))
cd "${dir}"

if [ ! -d lib ]; then
    pip3 install --target lib -r requirements.txt --prefix ""
fi

PYTHONPATH=./lib python3 mpd-webthing.py
