# playdate.graphics.animation

## Functions

### playdate.graphics.animation.blinker.new

```lua
playdate.graphics.animation.blinker.new(onDuration: integer, offDuration: integer, loop: boolean, cycles: integer, default: boolean): _Blinker
```

Creates a new blinker object. Check the object’s `on` property to determine whether the blinker is on (`true`) or off (`false`). The default properties are:
* *onDuration*: 200 (the number of milliseconds the blinker is "on")
* *offDuration*: 200 (the number of milliseconds the blinker is "off")
* *loop*: false (should the blinker restart after completing)
* *cycles*: 6 (the number of changes the blinker goes through before it’s complete)
* *default*: true (the state the blinker will start in. **Note:** if default is `true`, `blinker.on` will return `true` when the blinker is in its *onDuration* phase. If default is `false`, `blinker.on` will return `false` when the blinker is in its *onDuration* phase.)
Other informative properties:
* *counter*: Read this property to see which cycle the blinker is on (counts from *n* down to zero)
* *on*: Read this property to determine the current state of the blinker. The blinker always starts in the state specified by the `default` property.
* *running*: Read this property to see if the blinker is actively running

### playdate.graphics.animation.blinker.stopAll

```lua
playdate.graphics.animation.blinker.stopAll(): nil
```

Stops all blinkers.

### playdate.graphics.animation.blinker.updateAll

```lua
playdate.graphics.animation.blinker.updateAll(): nil
```

Updates the state of all valid blinkers by calling :update() on each.
If you intend to use blinkers, be sure to call `:updateAll()` once a cycle, ideally in your game’s `playdate.update()` function.

### playdate.graphics.animation.blinker:remove

```lua
playdate.graphics.animation.blinker:remove(): nil
```

Flags the caller for removal from the global list of blinkers

### playdate.graphics.animation.blinker:start

```lua
playdate.graphics.animation.blinker:start(onDuration: integer, offDuration: integer, loop: boolean, cycles: integer, default: boolean): nil
```

Starts a blinker if it’s not running. Pass values for any property values you wish to modify.

### playdate.graphics.animation.blinker:startLoop

```lua
playdate.graphics.animation.blinker:startLoop(): nil
```

Starts a blinker if it’s not running and sets its `loop` property to true. Equivalent to calling `playdate.graphics.animation.blinker:start(nil, nil, true)`

### playdate.graphics.animation.blinker:stop

```lua
playdate.graphics.animation.blinker:stop(): nil
```

Stops a blinker if it’s running, returning the blinker’s `on` properly to the default value.

### playdate.graphics.animation.blinker:update

```lua
playdate.graphics.animation.blinker:update(): nil
```

Updates the caller’s state.

### playdate.graphics.animation.loop.new

```lua
playdate.graphics.animation.loop.new(interval: number, imageTable: _ImageTable, shouldLoop: boolean): _AnimationLoop
```

Creates a new animation object.
* ***imageTable*** must be a `playdate.graphics.imagetable` or an array-style table of `playdate.graphics.images`.
The following properties can be read or set directly, and have these defaults:
* ***interval*** : the value of *interval*, if passed, or 100ms (the elapsed time before advancing to the next imageTable frame)
* ***startFrame*** : 1 (the value the object resets to when the loop completes)
* ***endFrame*** : the number of images in *imageTable* if passed, or 1 (the last frame value in the loop)
* ***frame*** : 1 (the current frame counter)
* ***step*** : 1 (the value by which frame increments)
* ***shouldLoop*** : the value of *shouldLoop*, if passed, or true. (whether the object loops when it completes)
* ***paused*** : false (paused loops don’t change their frame value)

### playdate.graphics.animation.loop:draw

```lua
playdate.graphics.animation.loop:draw(x: integer, y: integer, flip: (integer|string)): nil
```

Draw’s the loop’s current image at *x*, *y*.
The *flip* argument is optional; see `playdate.graphics.image:draw()` for valid values.

### playdate.graphics.animation.loop:image

```lua
playdate.graphics.animation.loop:image(): _Image
```

Returns a `playdate.graphics.image` from the caller’s *imageTable* if it exists. The image returned will be at the imageTable’s index that matches the caller’s *frame*.

### playdate.graphics.animation.loop:isValid

```lua
playdate.graphics.animation.loop:isValid(): boolean
```

Returns false if the loop has passed its last frame and does not loop.

### playdate.graphics.animation.loop:setImageTable

```lua
playdate.graphics.animation.loop:setImageTable(imageTable: _ImageTable): nil
```

Sets the `playdate.graphics.imagetable` to be used for this animation loop, and sets the loop’s endFrame property to #imageTable.

## Classes

### _AnimationLoop

```lua
---@class _AnimationLoop : playdate.graphics.animation.loop
---@field delay number
---@field startFrame integer
---@field endFrame integer
---@field frame integer
---@field step integer
---@field shouldLoop boolean
---@field paused boolean
```

### _Blinker

```lua
---@class _Blinker : playdate.graphics.animation.blinker
---@field onDuration integer
---@field offDuration integer
---@field loop boolean
---@field cycles integer
---@field default boolean
---@field counter integer
---@field on boolean
---@field running boolean
```
