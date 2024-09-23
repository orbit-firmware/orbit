# Getting Started


This guide will tell you how to compile and flash your firmware.  
Make sure you read [Configuration](/configuration.html) first if you want custom functionalities.


## Locally


### Prerequisites
Rust:  
[https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

Clone the main repository:
```shell
cd /folder/of/your/choice
git clone https://github.com/orbit-firmware/orbit.git orbit
```

`MY_KEYBOARD` should be replaced with the keyboard of your choice.  
A full list is available [here](https://github.com/orbit-firmware/orbit/tree/master/keyboards)

<div class="c-spacer-small"></div>

### Compiling
if [gnu make](https://www.gnu.org/software/make/) is installed
```shell
cd orbit
make kb=MY_KEYBOARD
```

or plain script
```shell
cd orbit
cargo install cargo-play # only required once

cargo play ./orbit/build.rs -- MY_KEYBOARD # [!code focus]
cd build
cargo objcopy --release -- -O binary ../firmware.bin # [!code focus]
cargo objcopy --release -- -O ihex ../firmware.hex # [!code focus]
```

This produces `firmware.hex/bin` in the orbit directory.

<div class="c-spacer-small"></div>

### Flashing
if [gnu make](https://www.gnu.org/software/make/) is installed
```shell
cd orbit
make flash kb=MY_KEYBOARD
```

or plain script
```shell
cd orbit
cargo install cargo-play # only required once
cargo install cargo-embed # only required once

cargo play ./orbit/build.rs -- MY_KEYBOARD # [!code focus]
cd build
cargo embed # [!code focus]

# optionally pass the debug feature if you want to debug via st-link or j-link
cargo embed --features debug

```
  
<div class="c-spacer-large"></div>


## Locally (Docker)
```shell
  make docker // creates container and connects to docker tty
  make kb=MY_KEYBOARD
```
This produces `firmware.hex/bin` in the orbit directory.


<div class="c-spacer-large"></div>

## Github Actions

An example can be found [here](https://github.com/orbit-firmware/user).