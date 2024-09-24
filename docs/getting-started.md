# Getting Started


This guide will tell you how to compile and flash your firmware.  
Make sure you read [Configuration](/configuration.html) first if you want custom functionalities.


## Locally


### Prerequisites

Install `rust`:
[https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Install `probe-rs`:
```shell
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh
```

Clone the main repository:
```shell
cd /folder/of/your/choice
git clone https://github.com/orbit-firmware/orbit.git orbit # [!code focus]
```



<div class="c-spacer-small"></div>

### Compiling

::: info
`MY_KEYBOARD` should be replaced with the keyboard of your choice.  
A full list is available [here](https://github.com/orbit-firmware/orbit/tree/master/keyboards)
:::

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
cd build # [!code focus]
cargo objcopy --release -- -O binary ../firmware.bin # [!code focus]
cargo objcopy --release -- -O ihex ../firmware.hex # [!code focus]
```

This produces `firmware.hex/bin` in the orbit directory.

<div class="c-spacer-small"></div>

### Flashing

::: info
`MY_KEYBOARD` should be replaced with the keyboard of your choice.  
A full list is available [here](https://github.com/orbit-firmware/orbit/tree/master/keyboards)
:::


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
cd build # [!code focus]
cargo embed # [!code focus]

# optionally pass the debug feature if you want to debug via st-link or j-link
cargo embed --features debug
```
  
<div class="c-spacer-large"></div>


## Locally (Docker)

You can also use docker to produce the firmware files.  
This allows you to not intstall any tools (except docker itself) on your harddrive.  

To install docker, visit [https://www.docker.com/](https://www.docker.com/).

if [gnu make](https://www.gnu.org/software/make/) is installed
```shell
make docker // creates container and connects to docker tty
make kb=MY_KEYBOARD
```

or plain script
```shell
cd orbit/docker
docker-compose up -d # [!code focus]
docker exec -it orbit bash # [!code focus]

make kb=MY_KEYBOARD # [!code focus]
```

This produces `firmware.hex/bin` in the orbit directory.


<div class="c-spacer-large"></div>

## Github Actions

Github actions allow you to remotely compile the firmware,  
without even needing anyhing on ur computer.  
Though you have to fork the [user](https://github.com/orbit-firmware/user) repository.
  
More Information can be found inside the repository.
