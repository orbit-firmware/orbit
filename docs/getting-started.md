# Getting Started

## Locally

1. Install rust:  
[https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

2. Clone the main repository:
```shell
git clone https://github.com/rmk-firmware/rmk.git rmk
cd rmk
```

3. Then run:
```shell
cd rmk && RMK_KEYBOARD=MY_KEYBOARD cargo build --release
```
or *(if [gnu make](https://www.gnu.org/software/make/) is installed)*
```shell
make kb=MY_KEYBOARD
```

`MY_KEYBOARD` should be replaced with the keyboard of your choice.  
A full list is available [here](https://github.com/rmk-firmware/rmk/tree/master/keyboards)

4. this should produce `firmware.hex/bin` in the rmk directory.

## Github Actions