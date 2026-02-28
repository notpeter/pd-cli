# playdate.geometry.arc

## Functions

### playdate.geometry.arc.new

```lua
playdate.geometry.arc.new(x: integer, y: integer, radius: number, startAngle: number, endAngle: number, direction: boolean): _Arc
```

Returns a new playdate.geometry.arc. Angles should be specified in degrees. Zero degrees represents the top of the circle.
If specified, *direction* should be true for clockwise, false for counterclockwise. If not specified, the direction is inferred from the start and end angles.

### playdate.geometry.arc:copy

```lua
playdate.geometry.arc:copy(): _Arc
```

Returns a new copy of the arc.

### playdate.geometry.arc:isClockwise

```lua
playdate.geometry.arc:isClockwise(): boolean
```

Returns true if the direction of the arc is clockwise.

### playdate.geometry.arc:length

```lua
playdate.geometry.arc:length(): number
```

Returns the length of the arc.

### playdate.geometry.arc:pointOnArc

```lua
playdate.geometry.arc:pointOnArc(distance: integer, extend: boolean): _Point
```

Returns a new point on the arc, `distance` pixels from the arc’s start angle. If `extend` is true, the returned point is allowed to project past the arc’s endpoints; otherwise, it is constrained to the arc’s initial point if `distance` is negative, or the end point if `distance` is greater than the arc’s length.

### playdate.geometry.arc:setIsClockwise

```lua
playdate.geometry.arc:setIsClockwise(flag: boolean): nil
```

Sets the direction of the arc.

## Classes

### _Arc

```lua
---@class _Arc : playdate.geometry.arc
---@field x integer
---@field y integer
---@field radius integer
---@field startAngle number
---@field endAngle number
---@field direction boolean
```
