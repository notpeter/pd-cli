# playdate.graphics.nineSlice

## Functions

### playdate.graphics.nineSlice.new

```lua
playdate.graphics.nineSlice.new(imagePath: string, innerX: integer, innerY: integer, innerWidth: integer, innerHeight: integer): _NineSlice
```

Returns a new 9 slice image from the image at imagePath with the stretchable region defined by other parameters. The arguments represent the origin and dimensions of the innermost ("center") slice.

### playdate.graphics.nineSlice:drawInRect

```lua
playdate.graphics.nineSlice:drawInRect(rect: _Rect): nil
playdate.graphics.nineSlice:drawInRect(x: integer, y: integer, width: integer, height: integer): nil
```

Draws the 9 slice image at the desired coordinates by stretching the defined region to achieve the width and height inputs.

### playdate.graphics.nineSlice:getMinSize

```lua
playdate.graphics.nineSlice:getMinSize(): (integer, integer)
```

Returns the minimum size of the 9 slice image as a pair *(width, height)*.

### playdate.graphics.nineSlice:getSize

```lua
playdate.graphics.nineSlice:getSize(): (integer, integer)
```

Returns the size of the 9 slice image as a pair *(width, height)*.

## Classes

### _NineSlice

```lua
---@class _NineSlice : playdate.graphics.nineSlice
---@field innerRect _Rect
---@field minWidth integer
---@field minHeight integer
```
