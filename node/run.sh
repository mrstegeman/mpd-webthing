#!/bin/bash

dir=$(readlink -f $(dirname "$0"))
cd "${dir}"

if [ ! -d node_modules ]; then
    npm install
fi

node mpd-webthing.js
