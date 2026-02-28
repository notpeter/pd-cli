# playdate.geometry.lineSegment

## Functions

### playdate.geometry.lineSegment.fast_intersection

```lua
playdate.geometry.lineSegment.fast_intersection(x1: integer, y1: integer, x2: integer, y2: integer, x3: integer, y3: integer, x4: integer, y4: integer): (boolean, number?, number?)
```

For use in inner loops where speed is the priority.
Returns true if there is an intersection between the line segments defined by *(x1, y1)*, *(x2, y2)* and *(x3, y3)*, *(x4, y4)*. If there is an intersection, *x, y* values representing the intersection point are also returned.

### playdate.geometry.lineSegment.new

```lua
playdate.geometry.lineSegment.new(x1: integer, y1: integer, x2: integer, y2: integer): _LineSegment
```

Returns a new playdate.geometry.lineSegment.

### playdate.geometry.lineSegment:closestPointOnLineToPoint

```lua
playdate.geometry.lineSegment:closestPointOnLineToPoint(p: _Point): _Point
```

Returns a playdate.geometry.point that is the closest point to point *p* that is on the line segment.

### playdate.geometry.lineSegment:copy

```lua
playdate.geometry.lineSegment:copy(): _LineSegment
```

Returns a new copy of the line segment.

### playdate.geometry.lineSegment:intersectsLineSegment

```lua
playdate.geometry.lineSegment:intersectsLineSegment(ls: _LineSegment): (boolean, _Point?)
```

Returns true if there is an intersection between the caller and the line segment *ls*.
If there is an intersection, a playdate.geometry.point representing that point is also returned.

### playdate.geometry.lineSegment:intersectsPolygon

```lua
playdate.geometry.lineSegment:intersectsPolygon(poly: _Polygon): (boolean, _Point[]?)
```

Returns the values (*intersects*, *intersectionPoints*).
*intersects* is true if there is at least one intersection between the caller and poly.
*intersectionPoints* is an array of playdate.geometry.points containing all intersection points between the caller and poly.

### playdate.geometry.lineSegment:intersectsRect

```lua
playdate.geometry.lineSegment:intersectsRect(rect: _Rect): (boolean, _Point[]?)
```

Returns the values (*intersects*, *intersectionPoints*).
*intersects* is true if there is at least one intersection between the caller and rect.
*intersectionPoints* is an array of playdate.geometry.points containing all intersection points between the caller and rect.

### playdate.geometry.lineSegment:length

```lua
playdate.geometry.lineSegment:length(): number
```

Returns the length of the line segment.

### playdate.geometry.lineSegment:midPoint

```lua
playdate.geometry.lineSegment:midPoint(): _Point
```

Returns a playdate.geometry.point representing the mid point of the line segment.

### playdate.geometry.lineSegment:offset

```lua
playdate.geometry.lineSegment:offset(dx: integer, dy: integer): nil
```

Modifies the line segment, offsetting its values by *dx*, *dy*.

### playdate.geometry.lineSegment:offsetBy

```lua
playdate.geometry.lineSegment:offsetBy(dx: integer, dy: integer): _LineSegment
```

Returns a new line segment, the given segment offset by *dx*, *dy*.

### playdate.geometry.lineSegment:pointOnLine

```lua
playdate.geometry.lineSegment:pointOnLine(distance: integer, extend: boolean): _Point
```

Returns a playdate.geometry.point on the line segment, `distance` pixels from the start of the line. If `extend` is true, the returned point is allowed to project past the segment’s endpoints; otherwise, it is constrained to the line segment’s initial point if `distance` is negative, or the end point if `distance` is greater than the segment’s length.

### playdate.geometry.lineSegment:segmentVector

```lua
playdate.geometry.lineSegment:segmentVector(): _Vector2D
```

Returns a playdate.geometry.vector2D representation of the line segment.

### playdate.geometry.lineSegment:unpack

```lua
playdate.geometry.lineSegment:unpack(): (number, number, number, number)
```

Returns the values *x1, y1, x2, y2*.

## Classes

### _LineSegment

```lua
---@class _LineSegment : playdate.geometry.lineSegment
---@field x1 integer
---@field y1 integer
---@field x2 integer
---@field y2 integer
```
