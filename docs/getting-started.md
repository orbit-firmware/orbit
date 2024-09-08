# Getting Started

## Locally

1. Install rust:  
[https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

2. Clone the main repository:
```shell
git clone https://github.com/rmk-firmware/rmk.git rmk
```

3. Then run:
*(if [gnu make](https://www.gnu.org/software/make/) is installed)*
```shell
cd rmk
make kb=MY_KEYBOARD
```
else
```shell
chmod +x rmk/dev/compile.sh
rmk/dev/compile.sh MY_KEYBOARD 
```

`MY_KEYBOARD` should be replaced with the keyboard of your choice.  
A full list is available [here](https://github.com/rmk-firmware/rmk/tree/master/keyboards)

4. this should produce `firmware.hex/bin` in the rmk directory.


## Locally (Docker)
```
  make docker // creates container and connects to docker tty
  make kb=MY_KEYBOARD
```


## Github Actions

An example can be found [here](https://github.com/rmk-firmware/rmk-user-example).