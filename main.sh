#!/usr/bin/env sh

file='../ivy/text.nc'
#file='../gcode_rust_libary/file.nc'

echo 'cargo run -- --input '"$file"' --output image.png --blockwidth 20 --blockheight 20 --imgwidth 4096 --imgheight 4096 --fnvalue 100 && xdg-open image.png'
cargo run -- --input "$file" --output image.png --blockwidth 10 --blockheight 10 --imgwidth 4096 --imgheight 4096 --fnvalue 100 && xdg-open image.png

