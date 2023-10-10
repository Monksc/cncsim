#!/usr/bin/env sh

file='../ivy/text.nc'

runcustomblockwidth() {
    cargo run -- --input "$file" --output image.png --blockwidth "$1" --blockheight "$2" --imgwidth $(($1 * $3)) --imgheight $(($2 * $3)) --fnvalue 100 && (xdg-open image.png &); (sleep 2 && clear)
}

echo 'cargo run --release -- --input '"$file"' --output image.png --blockwidth 20 --blockheight 20 --imgwidth 4096 --imgheight 4096 --fnvalue 100 && xdg-open image.png'
#cargo run -- --input "$file" --output image.png --blockwidth 20 --blockheight 10 --imgwidth 4096 --imgheight 2048 --fnvalue 100 && xdg-open image.png
#runcustomblockwidth 13 20 "$((2**7))"
runcustomblockwidth 5 5 "$((2**7))"

