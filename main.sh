#!/usr/bin/env sh

#file='../ivy/text.nc'
#file='../ivy/test/The Square 1.nc'
#file='../flowersCAM/test/tempoutput/file-1.nc'
#file='../flowersCAM/test/tempoutput/file-2.nc'
#file='../flowersCAM/test/tempoutput/tool-infinite-contour-inward-1.0.nc'
#file='../flowersCAM/test/tempoutput/tool-baseup-1.0.nc'
file='../flowersCAM/test/tempoutput/new_file-5.nc'

runcustomblockwidth() {
    cargo run -- --input "$file" --output image.png --blockwidth "$1" --blockheight "$2" --imgwidth $(($1 * $3)) --imgheight $(($2 * $3)) --fnvalue 100 && (xdg-open image.png &);# (sleep 2 && clear)
}

echo 'cargo run --release -- --input '"$file"' --output image.png --blockwidth 20 --blockheight 20 --imgwidth 4096 --imgheight 4096 --fnvalue 100 && xdg-open image.png'
#cargo run -- --input "$file" --output image.png --blockwidth 20 --blockheight 10 --imgwidth 4096 --imgheight 2048 --fnvalue 100 && xdg-open image.png
#runcustomblockwidth 13 20 "$((2**7))"
runcustomblockwidth 13 20 "$((2**6))"

