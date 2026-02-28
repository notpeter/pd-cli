# playdate.geometry

## Functions

### playdate.geometry.distanceToPoint

```lua
playdate.geometry.distanceToPoint(x1: integer, y1: integer, x2: integer, y2: integer): number
```

Returns the the distance from point *(x1, y1)* to point *(x2, y2)*.
Compared to geometry.point:distanceToPoint(), this version will be slightly faster.

### playdate.geometry.squaredDistanceToPoint

```lua
playdate.geometry.squaredDistanceToPoint(x1: integer, y1: integer, x2: integer, y2: integer): number
```

Returns the square of the distance from point *(x1, y1)* to point *(x2, y2)*.
Compared to geometry.point:squaredDistanceToPoint(), this version will be slightly faster.

## Classes

### playdate.geometry

```lua
---@class playdate.geometry
---@field kUnflipped integer 0
---@field kFlippedX integer 1
---@field kFlippedY integer 2
---@field kFlippedXY integer 3
```

## See Also:

- [affineTransform](geometry/affineTransform.md)
- [arc](geometry/arc.md)
- [lineSegment](geometry/lineSegment.md)
- [point](geometry/point.md)
- [polygon](geometry/polygon.md)
- [rect](geometry/rect.md)
- [size](geometry/size.md)
- [vector2D](geometry/vector2D.md)
