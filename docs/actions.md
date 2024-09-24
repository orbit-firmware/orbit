# Actions

Actions can be written inside a keymap instead of a keycode.  
They will then execute instead of the keypress.


## Syntax

Inside a keymap actions are defined like this.

```
action_name(parameter1, parameter2)
```

### Optional Parameters
If you encounter something like this `action_name(pameter1, [parameter2])`,  
It means that the second parameter is optional.  
Thus you can define the action either/or.  
```
action_name(parameter1)
action_name(parameter1, parameter2)
```