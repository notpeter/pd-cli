# playdate.sound.controlsignal

## Functions

### playdate.sound.controlsignal.new

```lua
playdate.sound.controlsignal.new(): _ControlSignal
```

Creates a new control signal object, for automating effect parameters, channel pan and level, etc.

### playdate.sound.controlsignal:addEvent

```lua
playdate.sound.controlsignal:addEvent(event: table): nil
playdate.sound.controlsignal:addEvent(step: integer, value: number, interpolate: boolean): nil
```

`addEvent` is a simpler way of adding events one at a time than setting the entire *events* table. Arguments are either the values themselves in the given order, or a table containing values for `step`, `value`, and optionally `interpolate`. If `interpolate` is set, the signal’s output value is linearly interpolated from `value` at step `step` to the next event’s value at its given step.

### playdate.sound.controlsignal:clearEvents

```lua
playdate.sound.controlsignal:clearEvents(): nil
```

Clears all events from the control signal.

### playdate.sound.controlsignal:getControllerType

```lua
playdate.sound.controlsignal:getControllerType(): integer
```

Control signals in midi files are assigned a controller number, which describes the intent of the control. This function returns the controller number.

### playdate.sound.controlsignal:getValue

```lua
playdate.sound.controlsignal:getValue(): number
```

Returns the current output value of the control signal.

### playdate.sound.controlsignal:setControllerType

```lua
playdate.sound.controlsignal:setControllerType(number: integer): nil
```

Sets the midi controller number for the control signal, if that’s something you want to do. The value has no effect on playback.

### playdate.sound.controlsignal:setOffset

```lua
playdate.sound.controlsignal:setOffset(offset: number): nil
```

Sets the offset value for the control signal.

### playdate.sound.controlsignal:setScale

```lua
playdate.sound.controlsignal:setScale(scale: number): nil
```

Sets the scale value for the control signal.

## Classes

### _ControlSignal

```lua
---@class _ControlSignal : playdate.sound.controlsignal
---@field events _SoundControlEvent
```

### _SoundControlEvent

```lua
---@class _SoundControlEvent
---@field step integer
---@field value number
---@field interpolate? boolean
```
