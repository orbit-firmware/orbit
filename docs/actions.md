# Actions

Actions can be defined inside a keymap instead of using keycodes.  
This allows the keyboard to execute specific behaviors rather than just sending keypresses to the computer.  
Example use cases for actions would be to implement mouse control functionalities or rgb lighting controls.


## Implementations:
 - [Layers](actions/layers.md)
 - [Modifiers](actions/modifiers.md)
 - [Mouse](actions/mouse.md)
 - [RGB](actions/rgb.md)



## Syntax

Inside a keymap actions are defined like this.

```orbit
layer 1
_______________________________
press | ... action(1)    ...       # [!code focus]
```

### Optional Parameters
If you encounter something like this `action(pameter1, [parameter2])`,  
It means that the parameters within `[` `]` are optional.  
Thus you can define the action either/or.  

```orbit
layer 1
_______________________________
press | ... action(1)    ...       # [!code focus]
_______________________________
press | ... action(1, 2) ...       # [!code focus]
```
