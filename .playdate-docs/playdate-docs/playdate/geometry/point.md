# playdate.geometry.point

## Functions

### playdate.geometry.point.new

```lua
playdate.geometry.point.new(x: integer, y: integer): _Point
```

Returns a new playdate.geometry.point.

### playdate.geometry.point:__add

```lua
playdate.geometry.point:__add(v: _Vector2D): _Point
```

Returns a new point by adding the vector *v* to point *p*.

### playdate.geometry.point:__concat

```lua
playdate.geometry.point:__concat(p2: _Point): _LineSegment
```

Returns a new lineSegment connecting points *p1* and *p2*.

### playdate.geometry.point:__mul

```lua
playdate.geometry.point:__mul(t: _AffineTransform): _Point
```

Returns a new point by applying the transform *t* to point *p*.

### playdate.geometry.point:__sub

```lua
playdate.geometry.point:__sub(p2: _Point): _Vector2D
```

Returns the vector constructed by subtracting *p2* from *p1*. By this construction, *p2* + (*p1* - *p2*) == *p1*.

### playdate.geometry.point:copy

```lua
playdate.geometry.point:copy(): _Point
```

Returns a new copy of the point.

### playdate.geometry.point:distanceToPoint

```lua
playdate.geometry.point:distanceToPoint(p: _Point): number
```

Returns the distance to point *p*.

### playdate.geometry.point:offset

```lua
playdate.geometry.point:offset(dx: integer, dy: integer): nil
```

Modifies the point, offsetting its values by *dx*, *dy*.

### playdate.geometry.point:offsetBy

```lua
playdate.geometry.point:offsetBy(dx: integer, dy: integer): _Point
```

Returns a new point object, the given point offset by *dx*, *dy*.

### playdate.geometry.point:squaredDistanceToPoint

```lua
playdate.geometry.point:squaredDistanceToPoint(p: _Point): number
```

Returns the square of the distance to point *p*.

### playdate.geometry.point:unpack

```lua
playdate.geometry.point:unpack(): (number, number)
```

Returns the values *x, y*.

## Classes

### _Point

```lua
---@class _Point : playdate.geometry.point
---@field x number
---@field y number
```
