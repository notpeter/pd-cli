# playdate.geometry.vector2D

## Functions

### playdate.geometry.vector2D.new

```lua
playdate.geometry.vector2D.new(x: integer, y: integer): _Vector2D
```

Returns a new playdate.geometry.vector2D.

### playdate.geometry.vector2D.newPolar

```lua
playdate.geometry.vector2D.newPolar(length: number, angle: number): _Vector2D
```

Returns a new playdate.geometry.vector2D. Angles should be specified in degrees. Zero degrees represents the top of the circle.

### playdate.geometry.vector2D:__add

```lua
playdate.geometry.vector2D:__add(v2: _Vector2D): _Vector2D
```

Returns the vector formed by adding vector *v2* to vector *v1*.

### playdate.geometry.vector2D:__div

```lua
playdate.geometry.vector2D:__div(s: number): _Vector2D
```

Returns the vector divided by scalar *s*.

### playdate.geometry.vector2D:__mul

```lua
playdate.geometry.vector2D:__mul(s: number): _Vector2D
playdate.geometry.vector2D:__mul(t: _AffineTransform): _Vector2D
playdate.geometry.vector2D:__mul(v2: _Vector2D): _Vector2D
```

Returns the vector *v1* scaled by *s*.

### playdate.geometry.vector2D:__sub

```lua
playdate.geometry.vector2D:__sub(v2: _Vector2D): _Vector2D
```

Returns the vector formed by subtracting vector *v2* from vector *v1*.

### playdate.geometry.vector2D:__unm

```lua
playdate.geometry.vector2D:__unm(): _Vector2D
```

Returns the vector formed by negating the components of vector *v*.

### playdate.geometry.vector2D:addVector

```lua
playdate.geometry.vector2D:addVector(v: _Vector2D): nil
```

Modifies the caller by adding vector *v*.

### playdate.geometry.vector2D:angleBetween

```lua
playdate.geometry.vector2D:angleBetween(v: _Vector2D): number
```

Returns the angle between the caller and the vector *v*.

### playdate.geometry.vector2D:copy

```lua
playdate.geometry.vector2D:copy(): _Vector2D
```

Returns a new copy of the vector2D.

### playdate.geometry.vector2D:dotProduct

```lua
playdate.geometry.vector2D:dotProduct(v: _Vector2D): number
```

Returns the dot product of the caller and the vector *v*.

### playdate.geometry.vector2D:leftNormal

```lua
playdate.geometry.vector2D:leftNormal(): _Vector2D
```

Returns a vector that is the left normal of the caller.

### playdate.geometry.vector2D:magnitude

```lua
playdate.geometry.vector2D:magnitude(): number
```

Returns the magnitude of the caller.

### playdate.geometry.vector2D:magnitudeSquared

```lua
playdate.geometry.vector2D:magnitudeSquared(): number
```

Returns the square of the magnitude of the caller.

### playdate.geometry.vector2D:normalize

```lua
playdate.geometry.vector2D:normalize(): nil
```

Modifies the caller by normalizing it so that its length is 1. If the vector is (0,0), the vector is unchanged.

### playdate.geometry.vector2D:normalized

```lua
playdate.geometry.vector2D:normalized(): _Vector2D
```

Returns a new vector by normalizing the given vector.

### playdate.geometry.vector2D:projectAlong

```lua
playdate.geometry.vector2D:projectAlong(v: _Vector2D): nil
```

Modifies the caller by projecting it along the vector *v*.

### playdate.geometry.vector2D:projectedAlong

```lua
playdate.geometry.vector2D:projectedAlong(v: _Vector2D): _Vector2D
```

Returns a new vector created by projecting the given vector along the vector *v*.

### playdate.geometry.vector2D:rightNormal

```lua
playdate.geometry.vector2D:rightNormal(): _Vector2D
```

Returns a vector that is the right normal of the caller.

### playdate.geometry.vector2D:scale

```lua
playdate.geometry.vector2D:scale(s: number): nil
```

Modifies the caller, scaling it by amount *s*.

### playdate.geometry.vector2D:scaledBy

```lua
playdate.geometry.vector2D:scaledBy(s: number): _Vector2D
```

Returns the given vector scaled by *s*.

### playdate.geometry.vector2D:unpack

```lua
playdate.geometry.vector2D:unpack(): (number, number)
```

Returns the values *dx, dy*.

## Classes

### _Vector2D

```lua
---@class _Vector2D : playdate.geometry.vector2D
---@field dx number
---@field dy number
```
