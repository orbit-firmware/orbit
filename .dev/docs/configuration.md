# Configuration

## The `keyboard.toml` file
The keyboard file controls the capabileties of your keyboard.  
There are two places where this file can be placed.
 - `keyboards/my_keyboard.toml`:  
  for offical keyboards published in the repository
 - `user/keyboard.toml`:  
  Your own independent configuration

If you compile with an official configuration [`keyboards/..`](https://github.com/rmk-firmware/rmk/tree/master/keyboards) but also provide a `user` configuration,  
the contents will be merged.  
You can also use this to adjust official configurations to your taste.  


Here is an example configuration:
```toml
[keyboard]
product_id = 0x0000 # your product id
vendor_id = 0x0000 # your vendor id
name = "My Keyboard" # The keyboards name
manufacturer = "RMK Inc." # the keyboards manufacturer
chip = "stm32f303cbtx" # What main mcu/chip the keyboard is using https://github.com/rmk-firmware/rmk/tree/master/chips
key_count = 42 # How many total keys the keyboard has
debounce_ms = 50 # how many ms the key debounce should last 
keycodes = "french" # changes keycodes to represent a french keyboard

# which behaviors are active
[behaviors]
press = true
hold = true

# which actions are active
[actions]
layer_toggle = true
mouse = true

# if the keyboard uses a matrix
# NOTE: cant be defined together with [multiplexers]
[matrix]
analogue_read = true # default false
row_count = 3
col_count = 12
row_pins = ["PA0", "PA1", "PA2"]
col_pins = ["PA3", "PA4", "PA5", "PA6", "PA8", "PA9", "PA10", "PA15", "PB0", "PB1", "PB2", "PB10"]

# if the keyboard uses multiplexers
# NOTE: cant be defined together with [matrix]
[multiplexers]
analogue_read = true # default false
count = 3
channels = 16
sel_pins = ["PB3", "PB4", "PB6", "PB5"]
com_pins = ["PA13", "PA10", "PA9"]

# The row/col layout for the keys in order
# If multiplexers are defined, it corresponds to [multiplexer, channel]
[layout]
keys = [
  [0,9], [0,1], [0,3], [0,4], [1,0], [1,1], [1,2], [1,5], [2,0], [2,1], [2,2], [2,4],
  [0,8], [0,2], [0,5], [1,9], [1,8], [1,3], [1,4], [2,9], [2,8], [2,3], [0,6],
  [0,7], [0,0], [1,5], [1,4], [1,3], [1,0], [2,6], [2,5], [2,3], [2,0], [0,3],
  [0,2], [0,1], [0,1], [1,2], [1,1], [2,4], [2,2], [2,1],
]

[rgb] # per key rgb data pin(s)
data_pins = "PA7"

[underglow] # underglow rgb data pin(s)
data_pins = ["PA8"]

[swd] # serial wire debug pin configuration
swo = "PB2"
swclk = "PA14"
swdio = "PA15"

```

## Behaviors
Behaviors are controlling:
  - When a keypress is triggered.
  - What the final output of a keypress is.

## Actions
Actions are executable functions that change how your keyboard behaaves when they are triggerd.

