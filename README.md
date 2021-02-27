# esp32-sandbox

This is a simple sandbox to test out some functionality on the ESP32 with rust.

## Environment Setup
If you're using clion, then you should do the following to enable IntelliSense.

1. Link the custom rust build into rustup.
```shell
rustup toolchain link xtensa /{PATH_TO_RUST_XTENSA}/build/{TARGET_TRIPLE}/stage2

# Example
rustup toolchain link xtensa ~/Developer/github/MabezDev/rust-xtensa/build/x86_64-apple-darwin/stage2
```
2. Override the rust toolchain in this directory.
```shell
rustup override set xtensa 
```

3. Source the environment variables
```shell
source setenv.sh
```

4. Open Clion in this directory
```shell
clion .
```

## Flash and Run
Use `cargo-espflash` to flash the board.
```
cargo espflash --release <port>
```

## License
This is licensed under the [MIT license](./LICENSE).
