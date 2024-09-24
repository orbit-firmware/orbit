# Configuration

## The `keyboard.toml` file
The keyboard file controls the capabileties of your keyboard.  
There are two places where this file can be placed.
 - `keyboards/my_keyboard.toml`:  
  for keyboards published in the repository
 - `user/keyboard.toml`:  
  Your own independent configuration

If you compile with an predefined configuration [`keyboards/..`](https://github.com/orbit-firmware/orbit/tree/master/keyboards), but also provide a `user` configuration,  
their contents will be merged.  
You can also use this to adjust official configurations to your taste.  


## Keyboard

These are the general device informations used for usb or bluetooth.  

```toml
# keyboard.toml

# keyboard device definition
[keyboard] # [!code focus]
# your product id
# can be left at 0 for non commercial products
product_id = 0x0000 # [!code focus]
# your vendor id
# can be left at 0 for non commercial products
vendor_id = 0x0000 # [!code focus]
# The keyboards name
name = "My Keyboard" # [!code focus]
# the keyboards manufacturer
manufacturer = "orbit Inc." # [!code focus]
# What mcu/chip the keyboard is using 
chip = "stm32f303cb"  # [!code focus]
```
::: info
A list of chips can be found [here](https://github.com/orbit-firmware/orbit/tree/master/orbit/chips).
:::

## Settings
```toml
# keyboard.toml
# keyboard settings
[settings]  # [!code focus]
# language specific keycodes 
keycodes = "german"  # [!code focus]
# key debounce time frame in ms
debounce_time = 10  # [!code focus]
# default time for the hold behavior to activate in ms
hold_time = 180  # [!code focus]
# default time in between taps in ms
tapping_term = 220  # [!code focus]
```

## Behaviors

This controls wich [behaviors](/behaviors) are enabled for your keyboard.
```toml
# keyboard.toml
# which behaviors are active
[behaviors]   # [!code focus]
press = true  # [!code focus]
hold = true  # [!code focus]
```

## Actions
This controls wich [actions](/actions) are enabled for your keyboard.
```toml
# keyboard.toml
# which actions are active
[actions]  # [!code focus]
layers = true  # [!code focus]
mouse = true  # [!code focus]
```

## Flavors
This controls wich [flavors](/flavors) are enabled for your keyboard.
```toml
# keyboard.toml
# which actions are active
[flavors]  # [!code focus]
space_cadet = true  # [!code focus]
```

## Matrix
```toml
# keyboard.toml
# if the keyboard uses a matrix
# NOTE: cant be defined together with [multiplexers]
[matrix]  # [!code focus]
# default false
analogue_read = true  # [!code focus]
row_count = 3  # [!code focus]
col_count = 12  # [!code focus]
row_pins = ["PA0", "PA1", "PA2"]  # [!code focus]
col_pins = ["PA3", "PA4", "PA5", "PA6", "PA8", "PA9", "PA10", "PA15", "PB0", "PB1", "PB2", "PB10"]  # [!code focus]
layout = [  # [!code focus]
  [0,9], [0,1], [0,3], [0,4], [1,0], [1,1], [1,2], [1,5], [2,0], [2,1], [2,2], [2,4],   # [!code focus]
  [0,8], [0,2], [0,5], [1,9], [1,8], [1,3], [1,4], [2,9], [2,8], [2,3], [0,6],   # [!code focus]
  [0,7], [0,0], [1,5], [1,4], [1,3], [1,0], [2,6], [2,5], [2,3], [2,0], [0,3],   # [!code focus]
  [0,2], [0,1], [0,1], [1,2], [1,1], [2,4], [2,2], [2,1],   # [!code focus]
]   # [!code focus]
```


## Multiplexers
```toml
# keyboard.toml
# if the keyboard uses multiplexers
# NOTE: cant be defined together with [matrix]
[multiplexers]  # [!code focus]
 # default false
analogue_read = true  # [!code focus]
count = 3  # [!code focus]
channels = 16  # [!code focus]
sel_pins = ["PB3", "PB4", "PB6", "PB5"]  # [!code focus]
com_pins = ["PA13", "PA10", "PA9"]  # [!code focus]
layout = [  # [!code focus]
  [0,9], [0,1], [0,3], [0,4], [1,0], [1,1], [1,2], [1,5], [2,0], [2,1], [2,2], [2,4],   # [!code focus]
  [0,8], [0,2], [0,5], [1,9], [1,8], [1,3], [1,4], [2,9], [2,8], [2,3], [0,6],   # [!code focus]
  [0,7], [0,0], [1,5], [1,4], [1,3], [1,0], [2,6], [2,5], [2,3], [2,0], [0,3],   # [!code focus]
  [0,2], [0,1], [0,1], [1,2], [1,1], [2,4], [2,2], [2,1],   # [!code focus]
]   # [!code focus]
```



## Lighting

```toml
# keyboard.toml

[lighting]   # [!code focus]
driver = "wsqwe456" # [!code focus]
per_key_rgb = "PA7"  # [!code focus]
underglow  = ["PA8"] # [!code focus]
```

## Serial Wire Debug
```toml
# keyboard.toml
# serial wire debug pin configuration
[swd]  # [!code focus]
swo = "PB2"  # [!code focus]
swclk = "PA14"  # [!code focus]
swdio = "PA15"  # [!code focus]

```