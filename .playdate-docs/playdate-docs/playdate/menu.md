# playdate.menu

## Functions

### playdate.getSystemMenu

```lua
playdate.getSystemMenu(): playdate.menu
```

Returns a `playdate.menu` object. Use this to add your custom menu items.

### playdate.menu.item:getTitle

```lua
playdate.menu.item:getTitle(): string
```

Returns the title displayed for this menu item.

### playdate.menu.item:getValue

```lua
playdate.menu.item:getValue(): (integer|boolean|string)
```

Returns the value for this menu item.

### playdate.menu.item:setCallback

```lua
playdate.menu.item:setCallback(callback: function): nil
```

Sets the callback function for this menu item.

### playdate.menu.item:setTitle

```lua
playdate.menu.item:setTitle(newTitle: string): nil
```

Sets the title displayed for this menu item.
The `title` for a menu item can also be set using dot syntax.

### playdate.menu.item:setValue

```lua
playdate.menu.item:setValue(newValue: (integer|boolean|string)): nil
```

Sets the value for this menu item. The value is of a different type depending on the type of menu item:
* normal: integer
* checkmark: boolean
* options: string
Values for any menu type can also be set using integers.
The `value` for a menu item can also be set using dot syntax.

### playdate.menu:addCheckmarkMenuItem

```lua
playdate.menu:addCheckmarkMenuItem(title: string, initialValue: boolean, callback: function): nil
```

Creates a new menu item that can be checked or unchecked by the player.
*title* will be the title displayed by the menu item.
*initialValue* can be set to `true` or `false`, indicating the checked state of the menu item. Optional, defaults to `false`.
If this menu item is interacted with while the system menu is open, *callback* will be called when the menu is closed, before playdate.gameWillResume is called. The callback function will be passed one argument, a boolean value, indicating the current value of the menu item.
If the returned playdate.menu.item is nil, a second `errorMessage` return value will indicate the reason the operation failed.
Playdate OS allows a maximum of **three** custom menu items to be added to the System Menu.

### playdate.menu:addMenuItem

```lua
playdate.menu:addMenuItem(title: string, callback: function): nil
```

*title* will be the title displayed by the menu item.

### playdate.menu:addOptionsMenuItem

```lua
playdate.menu:addOptionsMenuItem(title: string, options: string[], initalValue: string, callback: function): nil
```

Creates a menu item that allows the player to cycle through a set of options.
*title* will be the title displayed by the menu item.
*options* should be an array-style table of strings representing the states the menu item can have. Due to limited horizontal space, the option strings and title should be kept short for this type of menu item.
*initialValue* can optionally be set to any of the values in the options array.
If the value of this menu item is changed while the system menu is open, *callback* will be called when the menu is closed, before playdate.gameWillResume is called. The callback function will be passed one string argument indicating the currently selection option.
If the returned playdate.menu.item is nil, a second `errorMessage` return value will indicate the reason the operation failed.
Playdate OS allows a maximum of **three** custom menu items to be added to the System Menu.

### playdate.menu:getMenuItems

```lua
playdate.menu:getMenuItems(): _MenuItem[]
```

### playdate.menu:removeAllMenuItems

```lua
playdate.menu:removeAllMenuItems(): nil
```

Removes from the referenced menu object all playdate.menu.items added by your game.
Items that were added to the System Menu by the operating system cannot be removed by this operation, or any other.

### playdate.menu:removeMenuItem

```lua
playdate.menu:removeMenuItem(menuItem: _MenuItem): nil
```

Removes the specified playdate.menu.item from the menu.

### playdate.setMenuImage

```lua
playdate.setMenuImage(image: _Image, xOffset: integer): nil
```

While the game is paused it can optionally provide an image to be displayed alongside the System Menu. Use this function to set that image.
*image* should be a 400 x 240 pixel playdate.graphics.image. All important content should be in the left half of the image in an area 200 pixels wide, as the menu will obscure the rest. The right side of the image will be visible briefly as the menu animates in and out.
Optionally, *xOffset* can be provided which must be a number between 0 and 200 and will cause the menu image to animate to a position offset left by *xOffset* pixels as the menu is animated in.
To remove a previously-set menu image, pass `nil` for the *image* argument.

## Classes

### _MenuItem

```lua
---@class _MenuItem : playdate.menu.item
---@field title string
---@field value (integer|boolean|string)
```
