# blinky
STM32F411RE with embedded rust no_std bare metal blinky app.

# Setup
Designed for **Linux Mint 22.2** (Ubuntu 24.04).<br>
Open a terminal and run the following:<br>
```bash
rustup install nightly
rustup default nightly
rustup target add thumbv7em-none-eabihf
rustup component add llvm-tools-preview
cargo install cargo-binutils
sudo apt install gcc-arm-none-eabi gdb-multiarch
sudo apt install libudev-dev
sudo apt install openocd
```

# Build
Use the **build.sh** script to get going.<br>
```bash
./build.sh 
b -- build release
c -- clean release
r -- reset target
e -- erase target
f -- flash target
k -- clear screen
m -- show menu
x -- exit
> 
```

# Flash
Use the **build.sh** script entry **f -- flash target**<br>
or just copy the target **bin** file onto the **NODE_F411RE** drive.

# Size
Total filesystem size of the **firmware** binary is about 612 bytes.<br>
Below the detailed **sections** output:<br>
```bash
blinky  :
section             size        addr
.text                596   0x8000000
.ARM.exidx            16   0x8000254
.data                  0  0x20000000
.bss                  28  0x20000000
.heap               8192         0x0
.comment             148         0x0
.ARM.attributes       60         0x0
Total               9040
```

## License

This project is licensed under the BSD 3-Clause License - see the LICENSE
file for details.

Copyright (c) 2026 alexander14k28@gmail.com

See [LICENSE](LICENSE) for the license governing this project.
