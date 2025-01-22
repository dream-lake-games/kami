#!/bin/bash

trunk build --release

# Trunk builds with paths like import lsdafkjsfdklj from '/kami-fdslkjsdfklj.js'
# Itch needs './kami-flkjdslsdjkf'
sed -i '' 's/\/kami-/\.\/kami-/g' dist/index.html

butler push ./dist dream-lake-games/kami:html
