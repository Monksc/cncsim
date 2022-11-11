#!/usr/bin/env sh

echo 'cargo run -- --input ../ivy/text.nc --output image.png --blockwidth 20 --blockheight 20 --imgwidth 4096 --imgheight 4096 --fnvalue 100 && xdg-open image.png'
cargo run -- --input ../ivy/text.nc --output image.png --blockwidth 10 --blockheight 10 --imgwidth 4096 --imgheight 4096 --fnvalue 100 && xdg-open image.png

