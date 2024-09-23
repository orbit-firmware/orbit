# Layers

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


| Method | Action | Description |
| ------ | ------------- | ----------- |
| Momentary Layer | `ml(layer)` | Enables the layer while the key is held down |
| Toggle Layer | `tl(layer)` | Switches to the layer when pressed. <br> Switches to the previous layer when pressed again |
| Set Layer | `sl(layer)` | Switches to the layer when pressed |
| Set Default Layer | `sbl(layer)` | Sets the [base layer](#base-layer). <br> This might be used to switch from QWERTY to DVORAK layout etc.  |



### Base layer
This layer is the always-active layer that other layers stack on top of.  
In other words, it defines how your basic keys work, without any other layering.  
This layer always has the index `0`.  