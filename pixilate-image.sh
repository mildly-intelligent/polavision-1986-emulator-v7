#!/bin/bash

image="$(magick "$1" -modulate 100,250,100 /dev/stdout | chafa -f symbols --symbols=solid --dither=ordered -c 256)"

if [[ "$2" = "" ]]; then
    printf "$image"
else
    printf "$image" | tee "$2"
fi