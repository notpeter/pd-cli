# playdate.geometry.polygon

## Functions

### playdate.geometry.polygon.new

```lua
playdate.geometry.polygon.new(numberOfVertices: integer): _Polygon
playdate.geometry.polygon.new(p1: _Point, p2: _Point, ...: integer): _Polygon
playdate.geometry.polygon.new(x1: integer, y1: integer, x2: integer, y2: integer, ...: integer): _Polygon
```

`new(x1, y1, x2, y2, ..., xn, yn)` returns a new playdate.geometry.polygon with vertices *(x1, y1)* through *(xn, yn)*.  The Lua function `table.unpack()` can be used to turn an array into function arguments.
`new(p1, p2, ..., pn)` does the same, except the points are expressed via point objects.
`new(numberOfVertices)` returns a new playdate.geometry.polygon with space allocated for *numberOfVertices* vertices. All vertices are initially (0, 0). Vertex coordinates can be set with playdate.geometry.polygon:setPointAt().
If the polygon’s first and last points are coincident, the polygon will be considered closed. Alternatively, you may call :close() to automatically close the polygon.
To draw a polygon, use `playdate.graphics.drawPolygon()`.

### playdate.geometry.polygon:__mul

```lua
playdate.geometry.polygon:__mul(t: _AffineTransform): _Polygon
```

Returns a new polygon formed by applying the transform *t* to polygon *p*.

### playdate.geometry.polygon:close

```lua
playdate.geometry.polygon:close(): nil
```

`:close()` closes a polygon. If the polygon’s first and last point aren’t coincident, a line segment will be generated to connect them.

### playdate.geometry.polygon:containsPoint

```lua
playdate.geometry.polygon:containsPoint(p: _Point, fillRule: integer): boolean
playdate.geometry.polygon:containsPoint(x: integer, y: integer, fillRule: integer): boolean
```

Returns a boolean value, true if the point *p* or the point at *(x, y)* is contained within the caller polygon.
`fillrule` is an optional argument that can be one of the values defined in playdate.graphics.setPolygonFillRule. By default `*playdate.graphics.kPolygonFillEvenOdd*` is used.

### playdate.geometry.polygon:copy

```lua
playdate.geometry.polygon:copy(): _Polygon
```

Returns a copy of a polygon.

### playdate.geometry.polygon:count

```lua
playdate.geometry.polygon:count(): integer
```

Returns the number of points in the polygon.

### playdate.geometry.polygon:getBounds

```lua
playdate.geometry.polygon:getBounds(): (number, number, number, number)
```

Returns multiple values (*x*, *y*, *width*, *height*) giving the axis-aligned bounding box for the polygon.

### playdate.geometry.polygon:getBoundsRect

```lua
playdate.geometry.polygon:getBoundsRect(): _Rect
```

Returns the axis-aligned bounding box for the given polygon as a `playdate.geometry.rect` object.

### playdate.geometry.polygon:getPointAt

```lua
playdate.geometry.polygon:getPointAt(n: integer): _Point
```

Returns the polygon’s *n*-th point.

### playdate.geometry.polygon:intersects

```lua
playdate.geometry.polygon:intersects(p: _Point): boolean
```

Returns true if the given polygon intersects the polygon *p*.

### playdate.geometry.polygon:isClosed

```lua
playdate.geometry.polygon:isClosed(): boolean
```

Returns true if the polygon is closed, false if not.

### playdate.geometry.polygon:length

```lua
playdate.geometry.polygon:length(): number
```

Returns the total length of all line segments in the polygon.

### playdate.geometry.polygon:pointOnPolygon

```lua
playdate.geometry.polygon:pointOnPolygon(distance: integer, extend: boolean): _Point
```

Returns a playdate.geometry.point on one of the polygon’s line segments, `distance` pixels from the start of the polygon. If `extend` is true, the  point is allowed to project past the polygon’s ends; otherwise, it is constrained to the polygon’s initial point if `distance` is negative, or the last point if `distance` is greater than the polygon’s length.

### playdate.geometry.polygon:setPointAt

```lua
playdate.geometry.polygon:setPointAt(n: integer, x: integer, y: integer): nil
```

Sets the polygon’s *n*-th point to (*x*, *y*).

### playdate.geometry.polygon:translate

```lua
playdate.geometry.polygon:translate(dx: integer, dy: integer): nil
```

Translates each point on the polygon by *dx*, *dy* pixels.
