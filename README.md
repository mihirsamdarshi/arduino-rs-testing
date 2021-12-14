# Learning How to Write Rust for Arduino

Getting started with Rust for Arduino. I am using an Arduino Uno

## Compilation

cargo build -Z build-std=core --target avr-atmega328p.json --release

## Flashing

Wrote a script called `flash.sh` to compile the binary and then flash the compiled binary to an ATmega328P board (
Arduino Uno).
