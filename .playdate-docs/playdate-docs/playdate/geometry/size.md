# playdate.geometry.size

## Functions

### playdate.geometry.size.new

```lua
playdate.geometry.size.new(width: integer, height: integer): _Size
```

Returns a new playdate.geometry.size.

### playdate.geometry.size:copy

```lua
playdate.geometry.size:copy(): _Size
```

Returns a new copy of the size.

### playdate.geometry.size:unpack

```lua
playdate.geometry.size:unpack(): (number, number)
```

Returns the values *width, height*.

## Classes

### _Size

```lua
---@class _Size : playdate.geometry.size
---@field width number
---@field height number
```
