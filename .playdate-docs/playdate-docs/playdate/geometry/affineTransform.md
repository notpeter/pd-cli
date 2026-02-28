# playdate.geometry.affineTransform

## Functions

### playdate.geometry.affineTransform.new

```lua
playdate.geometry.affineTransform.new(): _AffineTransform
playdate.geometry.affineTransform.new(m11: number, m12: number, m21: number, m22: number, tx: number, ty: number): _AffineTransform
```

Returns a new playdate.geometry.affineTransform that is the identity transform.

### playdate.geometry.affineTransform:__mul

```lua
playdate.geometry.affineTransform:__mul(p: _Point): _Point
playdate.geometry.affineTransform:__mul(t: _AffineTransform): _AffineTransform
playdate.geometry.affineTransform:__mul(v: _Vector2D): _Vector2D
```

Returns the point created by applying the transform *t* to the `point` *p*

### playdate.geometry.affineTransform:concat

```lua
playdate.geometry.affineTransform:concat(af: _AffineTransform): nil
```

Mutates the the caller. The affine transform *af* is concatenated to the caller.
Concatenation combines two affine transformation matrices by multiplying them together. You might perform several concatenations in order to create a single affine transform that contains the cumulative effects of several transformations.
Note that matrix operations are not commutative — the order in which you concatenate matrices is important. That is, the result of multiplying matrix t1 by matrix t2 does not necessarily equal the result of multiplying matrix t2 by matrix t1.

### playdate.geometry.affineTransform:copy

```lua
playdate.geometry.affineTransform:copy(): _AffineTransform
```

Returns a new copy of the affine transform.

### playdate.geometry.affineTransform:invert

```lua
playdate.geometry.affineTransform:invert(): nil
```

Mutates the caller so that it is an affine transformation matrix constructed by inverting itself.
Inversion is generally used to provide reverse transformation of points within transformed objects. Given the coordinates (x, y), which have been transformed by a given matrix to new coordinates (x’, y’), transforming the coordinates (x’, y’) by the inverse matrix produces the original coordinates (x, y).

### playdate.geometry.affineTransform:reset

```lua
playdate.geometry.affineTransform:reset(): nil
```

Mutates the the caller, changing it to an identity transform matrix.

### playdate.geometry.affineTransform:rotate

```lua
playdate.geometry.affineTransform:rotate(angle: number, point: _Point): nil
playdate.geometry.affineTransform:rotate(angle: number, x: integer, y: integer): nil
```

Mutates the caller by applying a rotation transformation.
*angle* is the value, in degrees, by which to rotate the affine transform. A positive value specifies clockwise rotation and a negative value specifies counterclockwise rotation. If the optional playdate.geometry.point *point* argument is given, the transform rotates around the *point* instead of (0,0).

### playdate.geometry.affineTransform:rotatedBy

```lua
playdate.geometry.affineTransform:rotatedBy(angle: number, point: _Point): _AffineTransform
playdate.geometry.affineTransform:rotatedBy(angle: number, x: integer, y: integer): _AffineTransform
```

Returns a copy of the calling affine transform with a rotate transformation appended.
*angle* is the value, in degrees, by which to rotate the affine transform. A positive value specifies clockwise rotation and a negative value specifies counterclockwise rotation.  If the optional point *point* argument is given, the transform rotates around the *point* instead of (0,0).

### playdate.geometry.affineTransform:scale

```lua
playdate.geometry.affineTransform:scale(sx: number, sy: number): nil
```

Mutates the caller by applying a scaling transformation.
If both parameters are passed, *sx* is used to scale the x values of the transform, *sy* is used to scale the y values.
If only one parameter is passed, it is used to scale both x and y values.

### playdate.geometry.affineTransform:scaledBy

```lua
playdate.geometry.affineTransform:scaledBy(sx: number, sy: number): _AffineTransform
```

Returns a copy of the calling affine transform with a scaling transformation appended.
If both parameters are passed, *sx* is used to scale the x values of the transform, *sy* is used to scale the y values.
If only one parameter is passed, it is used to scale both x and y values.

### playdate.geometry.affineTransform:skew

```lua
playdate.geometry.affineTransform:skew(sx: number, sy: number): nil
```

Mutates the caller, appending a skew transformation.  *sx* is the value by which to skew the x axis, and *sy* the value for the y axis. Values are in degrees.

### playdate.geometry.affineTransform:skewedBy

```lua
playdate.geometry.affineTransform:skewedBy(sx: number, sy: number): _AffineTransform
```

Returns the given transform with a skew transformation appended.  *sx* is the value by which to skew the x axis, and *sy* the value for the y axis. Values are in degrees.

### playdate.geometry.affineTransform:transformAABB

```lua
playdate.geometry.affineTransform:transformAABB(r: _Rect): nil
```

Modifies the axis aligned bounding box *r* (a rect) by applying the affine transform.

### playdate.geometry.affineTransform:transformLineSegment

```lua
playdate.geometry.affineTransform:transformLineSegment(ls: _LineSegment): nil
```

Modifies the line segment *ls* by applying the affine transform.

### playdate.geometry.affineTransform:transformPoint

```lua
playdate.geometry.affineTransform:transformPoint(p: _Point): nil
```

Modifies the point *p* by applying the affine transform.

### playdate.geometry.affineTransform:transformPolygon

```lua
playdate.geometry.affineTransform:transformPolygon(p: _Polygon): nil
```

Modifies the polygon *p* by applying the affine transform.

### playdate.geometry.affineTransform:transformXY

```lua
playdate.geometry.affineTransform:transformXY(x: integer, y: integer): (number, number)
```

Returns two values calculated by applying the affine transform to the point (*x*, *y*)

### playdate.geometry.affineTransform:transformedAABB

```lua
playdate.geometry.affineTransform:transformedAABB(r: _Rect): _Rect
```

As above, but returns a new rect rather than modifying *r*.

### playdate.geometry.affineTransform:transformedLineSegment

```lua
playdate.geometry.affineTransform:transformedLineSegment(ls: _LineSegment): _LineSegment
```

As above, but returns a new line segment rather than modifying *ls*.

### playdate.geometry.affineTransform:transformedPoint

```lua
playdate.geometry.affineTransform:transformedPoint(p: _Point): _Point
```

As above, but returns a new point rather than modifying *p*.

### playdate.geometry.affineTransform:transformedPolygon

```lua
playdate.geometry.affineTransform:transformedPolygon(p: _Polygon): _Polygon
```

As above, but returns a new polygon rather than modifying *p*.

### playdate.geometry.affineTransform:translate

```lua
playdate.geometry.affineTransform:translate(dx: integer, dy: integer): nil
```

Mutates the caller by applying a translate transformation.  x values are moved by *dx*, y values by *dy*.

### playdate.geometry.affineTransform:translatedBy

```lua
playdate.geometry.affineTransform:translatedBy(dx: integer, dy: integer): _AffineTransform
```

Returns a copy of the calling affine transform with a translate transformation appended.
