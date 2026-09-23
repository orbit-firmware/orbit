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

if [just](https://github.com/casey/just) is installed
```shell
cd orbit
just compile MY_KEYBOARD
```

or plain script
```shell
cd orbit
cargo install cargo-play # only required once

cargo play ./orbit/build.rs -- MY_KEYBOARD # [!code focus]
cd build # [!code focus]
cargo build --release # [!code focus]
```

This produces the firmware ELF at `build/target/thumbv7em-none-eabi/release/MY_KEYBOARD`.
Flash it with `just flash MY_KEYBOARD` (needs [probe-rs](https://probe.rs)).

<div class="c-spacer-small"></div>

### Flashing

::: info
`MY_KEYBOARD` should be replaced with the keyboard of your choice.  
A full list is available [here](https://github.com/orbit-firmware/orbit/tree/master/keyboards)
:::


if [just](https://github.com/casey/just) is installed
```shell
cd orbit # [!code focus]
just flash MY_KEYBOARD # [!code focus]
# optionally pass the debug feature if you want to debug via st-link or j-link
just flash MY_KEYBOARD debug
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

if [just](https://github.com/casey/just) is installed
```shell
cd orbit

# creates container and connects to docker tty
just docker  # [!code focus]

# once conencted to the docker container
just compile MY_KEYBOARD # [!code focus]
```

or plain script
```shell
cd orbit/docker

# creates container and connects to docker tty
docker-compose up -d # [!code focus]
docker exec -it orbit bash # [!code focus]

# once conencted to the docker container
just compile MY_KEYBOARD # [!code focus]
```

This produces the firmware ELF at `build/target/thumbv7em-none-eabi/release/MY_KEYBOARD`.
Flash it with `just flash MY_KEYBOARD` (needs [probe-rs](https://probe.rs)).


<div class="c-spacer-large"></div>

## Github Actions

Github actions allow you to remotely compile the firmware,  
without even needing anyhing on ur computer.  
Though you have to fork the [user](https://github.com/orbit-firmware/user) repository.
  
More Information can be found inside the repository.
