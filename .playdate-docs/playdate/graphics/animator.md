# playdate.graphics.animator

## Functions

### playdate.graphics.animator.new

```lua
playdate.graphics.animator.new(duration: integer, arc: _Arc, easingFunction: function, startTimeOffset: integer): _Animator
playdate.graphics.animator.new(duration: integer, lineSegment: _LineSegment, easingFunction: function, startTimeOffset: integer): _Animator
playdate.graphics.animator.new(duration: integer, polygon: _Polygon, easingFunction: function, startTimeOffset: integer): _Animator
playdate.graphics.animator.new(duration: integer, startValue: (number|_Point), endValue: (number|_Point), easingFunction: function, startTimeOffset: integer): _Animator
playdate.graphics.animator.new(durations: integer, parts: number[], easingFunctions: function[], startTimeOffset: integer): _Animator
```

Creates a new Animator that will animate along the provided playdate.geometry.arc

### playdate.graphics.animator:currentValue

```lua
playdate.graphics.animator:currentValue(): (number|_Point)
```

Returns the current value of the animation, which will be either a number or a playdate.geometry.point, depending on the type of animator.

### playdate.graphics.animator:ended

```lua
playdate.graphics.animator:ended(): boolean
```

Returns true if the animation is completed. Only returns true if this function or `currentValue()` has been called since the animation ended in order to allow animations to fully finish before true is returned.

### playdate.graphics.animator:progress

```lua
playdate.graphics.animator:progress(): number
```

Returns the current progress of the animation as a value from 0 to 1.

### playdate.graphics.animator:reset

```lua
playdate.graphics.animator:reset(duration: integer): nil
```

Resets the animation, setting its start time to the current time, and changes the animation’s duration if a new duration is given.

## Classes

### _Animator

```lua
---@class _Animator : playdate.graphics.animator
---@field repeatCount integer
---@field reverses boolean
---@field easingAmplitude? number
---@field easingPeriod? number
---@field s? number
---@field a? number
---@field p? number
```
