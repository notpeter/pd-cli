# playdate.geometry.rect

## Functions

### playdate.geometry.rect.fast_intersection

```lua
playdate.geometry.rect.fast_intersection(x1: integer, y1: integer, w1: integer, h1: integer, x2: integer, y2: integer, w2: integer, h2: integer): (number, number, number, number)
```

For use in inner loops where speed is the priority. About 3x faster than intersection.
Returns multiple values (*x, y, width, height*) representing the overlapping portion of the two rects defined by *x1, y1, w1, h1* and *x2, y2, w2, h2*. If there is no intersection, (0, 0, 0, 0) is returned.

### playdate.geometry.rect.fast_union

```lua
playdate.geometry.rect.fast_union(x1: integer, y1: integer, w1: integer, h1: integer, x2: integer, y2: integer, w2: integer, h2: integer): (number, number, number, number)
```

For use in inner loops where speed is the priority. About 3x faster than union.
Returns multiple values (*x, y, width, height*) representing the smallest possible rect that contains the two rects defined by *x1, y1, w1, h1* and *x2, y2, w2, h2*.

### playdate.geometry.rect.new

```lua
playdate.geometry.rect.new(x: integer, y: integer, width: integer, height: integer): _Rect
```

Returns a new playdate.geometry.rect.

### playdate.geometry.rect:centerPoint

```lua
playdate.geometry.rect:centerPoint(): _Point
```

Returns a point at the center of the caller.

### playdate.geometry.rect:containsPoint

```lua
playdate.geometry.rect:containsPoint(p: _Point): boolean
playdate.geometry.rect:containsPoint(x: integer, y: integer): boolean
```

Returns true if the point *p* is contained within the caller rect.

### playdate.geometry.rect:containsRect

```lua
playdate.geometry.rect:containsRect(r2: _Rect): boolean
playdate.geometry.rect:containsRect(x: integer, y: integer, width: integer, height: integer): boolean
```

Returns true if the rect *r2* is contained within the caller rect.

### playdate.geometry.rect:copy

```lua
playdate.geometry.rect:copy(): _Rect
```

Returns a new copy of the rect.

### playdate.geometry.rect:flipRelativeToRect

```lua
playdate.geometry.rect:flipRelativeToRect(r2: _Rect, flip: (integer|string)): nil
```

Flips the caller about the center of rect *r2*.
*flip* should be one of the following constants:
* *playdate.geometry.kUnflipped*
* *playdate.geometry.kFlippedX*
* *playdate.geometry.kFlippedY*
* *playdate.geometry.kFlippedXY*

### playdate.geometry.rect:inset

```lua
playdate.geometry.rect:inset(dx: integer, dy: integer): nil
```

Insets the rect by the given *dx* and *dy*.

### playdate.geometry.rect:insetBy

```lua
playdate.geometry.rect:insetBy(dx: integer, dy: integer): _Rect
```

Returns a rect that is inset by the given *dx* and *dy*, with the same center point.

### playdate.geometry.rect:intersection

```lua
playdate.geometry.rect:intersection(r2: _Rect): _Rect
```

Returns a rect representing the overlapping portion of the caller and *r2*.

### playdate.geometry.rect:intersects

```lua
playdate.geometry.rect:intersects(r2: _Rect): boolean
```

Returns true if *r2* intersects the caller.

### playdate.geometry.rect:isEmpty

```lua
playdate.geometry.rect:isEmpty(): boolean
```

Returns true if a rectangle has zero width or height.

### playdate.geometry.rect:isEqual

```lua
playdate.geometry.rect:isEqual(r2: _Rect): boolean
```

Returns true if the *x*, *y*, *width*, and *height* values of the caller and *r2* are all equal.

### playdate.geometry.rect:offset

```lua
playdate.geometry.rect:offset(dx: integer, dy: integer): nil
```

Offsets the rect by the given *dx* and *dy*.

### playdate.geometry.rect:offsetBy

```lua
playdate.geometry.rect:offsetBy(dx: integer, dy: integer): _Rect
```

Returns a rect with its origin point offset by *dx*, *dy*.

### playdate.geometry.rect:toPolygon

```lua
playdate.geometry.rect:toPolygon(): _Polygon
```

Returns a new playdate.geometry.polygon version of the rect.

### playdate.geometry.rect:union

```lua
playdate.geometry.rect:union(r2: _Rect): _Rect
```

Returns the smallest possible rect that contains both the source rect and *r2*.

### playdate.geometry.rect:unpack

```lua
playdate.geometry.rect:unpack(): (number, number, number, number)
```

Returns *x*, *y*, *width* and *height* as individual values.

## Classes

### _Rect

```lua
---@class _Rect : playdate.geometry.rect
---@field x number
---@field y number
---@field width number
---@field height number
---@field top number
---@field bottom number
---@field left number
---@field right number
---@field size _Size
```
