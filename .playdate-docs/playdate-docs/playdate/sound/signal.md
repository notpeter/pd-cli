# playdate.sound.signal

## Functions

### playdate.sound.signal:getValue

```lua
playdate.sound.signal:getValue(): number
```

Returns the current output value of the signal.

### playdate.sound.signal:setOffset

```lua
playdate.sound.signal:setOffset(offset: number): nil
```

Adds a constant offset to the signal (lfo, envelope, etc.).

### playdate.sound.signal:setScale

```lua
playdate.sound.signal:setScale(scale: integer): nil
```

Multiplies the signal’s output by the given scale factor. The scale is applied before the offset.
