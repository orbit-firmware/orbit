# Layers Action

Layers allow you to add more functionality within a smaller footprint.  
You can think of them like the Shift key: `one key can define how other keys behave`.  
However, unlike Shift, there are multiple ways to enable layers.  
By utilizing layers, orbit provides a flexible and powerful way to enhance your keyboard experience.

Benefits of Using Layers

- **Efficient Space Management**  
  Layers allow for more complex configurations without increasing the physical number of keys.
- **Customization**  
  Create personalized key mappings for different applications or tasks by using various layers.


## Usage

If you are not sure how to write actions, look up the syntax [here](/actions#syntax).  

| Method | Code | Description |
| ------ | ------------- | ----------- |
| Momentary Layer | `ml(layer, [modifier_mask])` | Enables the layer while the key is held down |
| Toggle Layer | `tl(layer, [modifier_mask])` | Switches to the layer when pressed. <br> Switches to the previous layer when pressed again |
| Set Layer | `sl(layer, [modifier_mask])` | Switches to the layer when pressed |
| Set Default Layer | `sbl(layer, [modifier_mask])` | Sets the [base layer](#base-layer). <br> This might be used to switch from QWERTY to DVORAK layout etc.  |

#### Modifier Mask
You can optionally pass a `modifier mask` that has to be active for the layer action to activate.  
A list of masks can be found [here](/modifiers#masks).  



### Base layer
This layer is the always-active layer that other layers stack on top of.  
In other words, it defines how your basic keys work, without any other layering.  
This layer always has the index `0`.

## Examples

The following examples only have 4 keys defined for ease of demonstration.

```
# when key one is pressed down, it switches to layer 1
layer 0
press | ml(1) q     w     e # [!code focus]

# on layer 2 we replace q, w and e with 1, 2 and 3
layer 1
press | ---  1     2     3  # [!code focus]

```


```
# normally key one is "esc"
# when key one is held, it switches to layer 1
layer 0
press | esc   q     w     e
hold  | ml(1) q     w     e # [!code focus]

# on layer 2 we replace "q", "w" and "e" with "1", "2" and "3"
layer 1
press | ---  1     2     3  # [!code focus]

```








