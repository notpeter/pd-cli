# playdate.keyboard

## Functions

### playdate.keyboard.hide

```lua
playdate.keyboard.hide(): nil
```

Hides the keyboard.

### playdate.keyboard.isVisible

```lua
playdate.keyboard.isVisible(): nil
```

Returns true if the keyboard is currently being shown.

### playdate.keyboard.keyboardAnimatingCallback

```lua
playdate.keyboard.keyboardAnimatingCallback(): nil
```

If set, this function is called as the keyboard animates open or closed. Provided as a way to sync animations with the keyboard movement.

### playdate.keyboard.keyboardDidHideCallback

```lua
playdate.keyboard.keyboardDidHideCallback(): nil
```

If set, this function will be called when the keyboard has finished the hide animation.

### playdate.keyboard.keyboardDidShowCallback

```lua
playdate.keyboard.keyboardDidShowCallback(): nil
```

If set, this function will be called when the keyboard is finished the opening animation.

### playdate.keyboard.keyboardWillHideCallback

```lua
playdate.keyboard.keyboardWillHideCallback(): nil
```

If set, this function will be called when the keyboard starts to close. A Boolean argument will be passed to the callback, `true` if the user selected "OK" close the keyboard, `false` otherwise.

### playdate.keyboard.left

```lua
playdate.keyboard.left(): nil
```

Returns the current x location of the left edge of the keyboard.

### playdate.keyboard.setCapitalizationBehavior

```lua
playdate.keyboard.setCapitalizationBehavior(behavior: integer): nil
```

*behavior* should be one of the constants *playdate.keyboard.kCapitalizationNormal*, *playdate.keyboard.kCapitalizationWords*, or *playdate.keyboard.kCapitalizationSentences*.
In the case of *playdate.keyboard.kCapitalizationWords*, the keyboard selection will automatically move to the upper case column after a space is entered. For *playdate.keyboard.kCapitalizationSentences* the selection will automatically move to the upper case column after a period and a space have been entered.

### playdate.keyboard.show

```lua
playdate.keyboard.show(text: string): nil
```

Opens the keyboard, taking over input focus.
*text*, if provided, will be used to set the initial text value of the keyboard.

### playdate.keyboard.textChangedCallback

```lua
playdate.keyboard.textChangedCallback(): nil
```

If set, this function will be called every time a character is entered or deleted.

### playdate.keyboard.width

```lua
playdate.keyboard.width(): nil
```

Returns the pixel width of the keyboard.

## Classes

### playdate.keyboard

```lua
---@class playdate.keyboard
---@field kCapitalizationNormal integer 1
---@field kCapitalizationSentences integer 3
---@field kCapitalizationWords integer 2
---@field text string
```
