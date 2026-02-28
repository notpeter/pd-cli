# playdate.display

## Functions

### playdate.display.flush

```lua
playdate.display.flush(): nil
```

Sends the contents of the frame buffer to the display immediately. Useful if you have called playdate.stop() to disable update callbacks in, say, the case where your app updates the display only in reaction to button presses.

### playdate.display.getHeight

```lua
playdate.display.getHeight(): integer
```

Returns the height the Playdate display, taking the current display scale into account; e.g., if the scale is 2, the values returned will be based off of a 200 x 120-pixel screen rather than the native 400 x 240. (See playdate.display.setScale().)
Equivalent to `playdate->display->getHeight()` in the C API.

### playdate.display.getInverted

```lua
playdate.display.getInverted(): boolean
```

Returns the current value of the display invert flag.

### playdate.display.getMosaic

```lua
playdate.display.getMosaic(): (integer, integer)
```

Returns the current mosaic effect settings as multiple values (*x*, *y*).

### playdate.display.getOffset

```lua
playdate.display.getOffset(): (integer, integer)
```

`getOffset()` returns the current display offset as multiple values (*x*, *y*).

### playdate.display.getRect

```lua
playdate.display.getRect(): _Rect
```

Returns the values *(x, y, width, height)* describing the Playdate display size. Takes the current display scale into account; e.g., if the scale is 2, the values returned will be based off of a 200 x 120-pixel screen rather than the native 400 x 240. (See playdate.display.setScale().)

### playdate.display.getRefreshRate

```lua
playdate.display.getRefreshRate(): integer
```

Returns the specified refresh rate in frames per second. See also playdate.getFPS() for *measured, actual* frame rate.

### playdate.display.getScale

```lua
playdate.display.getScale(): integer
```

Gets the display scale factor. Valid values for *scale* are 1, 2, 4, and 8.

### playdate.display.getSize

```lua
playdate.display.getSize(): (integer, integer)
```

Returns the values *(width, height)* describing the Playdate display size. Takes the current display scale into account; e.g., if the scale is 2, the values returned will be based off of a 200 x 120-pixel screen rather than the native 400 x 240. (See playdate.display.setScale().)

### playdate.display.getWidth

```lua
playdate.display.getWidth(): integer
```

Returns the width the Playdate display, taking the current display scale into account; e.g., if the scale is 2, the values returned will be based off of a 200 x 120-pixel screen rather than the native 400 x 240. (See playdate.display.setScale().)
Equivalent to `playdate->display->getWidth()` in the C API.

### playdate.display.loadImage

```lua
playdate.display.loadImage(path: string): nil
```

The simplest method for putting an image on the display. Copies the contents of the image at *path* directly to the frame buffer. The image must be 400x240 pixels with no transparency.
Loading an image via playdate.graphics.image.new() and drawing it at a desired coordinate with playdate.graphics.image:draw() offers more flexibility.

### playdate.display.setFlipped

```lua
playdate.display.setFlipped(x: boolean, y: boolean): nil
```

Flips the display on the x or y axis, or both.
Function arguments are booleans, and in Lua `0` evaluates to `true`.
Equivalent to `playdate->display->setFlipped()` in the C API.

### playdate.display.setInverted

```lua
playdate.display.setInverted(flag: boolean): nil
```

If the argument passed to `setInverted()` is true, the frame buffer will be drawn inverted (everything onscreen that was black will now be white, etc.)
Equivalent to `playdate->display->setInverted()` in the C API.

### playdate.display.setMosaic

```lua
playdate.display.setMosaic(x: integer, y: integer): nil
```

Adds a mosaic effect to the display. Valid *x* and *y* values are between 0 and 3, inclusive.
Equivalent to `playdate->display->setMosaic()` in the C API.

### playdate.display.setOffset

```lua
playdate.display.setOffset(x: integer, y: integer): nil
```

Offsets the entire display by *x*, *y*. Offset values can be negative. The "exposed" part of the display is black or white, according to the value set in playdate.graphics.setBackgroundColor(). This is an efficient way to make a "shake" effect without redrawing anything.
This function is different from playdate.graphics.setDrawOffset().
Equivalent to `playdate->display->setOffset()` in the C API.
```
-- You can copy and paste this example directly as your main.lua file to see it in action
import "CoreLibs/graphics"
import "CoreLibs/timer"
-- This function relies on the use of timers, so the timer core library
-- must be imported, and updateTimers() must be called in the update loop
local function screenShake(shakeTime, shakeMagnitude)
    -- Creating a value timer that goes from shakeMagnitude to 0, over
    -- the course of 'shakeTime' milliseconds
    local shakeTimer = playdate.timer.new(shakeTime, shakeMagnitude, 0)
    -- Every frame when the timer is active, we shake the screen
    shakeTimer.updateCallback = function(timer)
        -- Using the timer value, so the shaking magnitude
        -- gradually decreases over time
        local magnitude = math.floor(timer.value)
        local shakeX = math.random(-magnitude, magnitude)
        local shakeY = math.random(-magnitude, magnitude)
        playdate.display.setOffset(shakeX, shakeY)
    end
    -- Resetting the display offset at the end of the screen shake
    shakeTimer.timerEndedCallback = function()
        playdate.display.setOffset(0, 0)
    end
end
function playdate.update()
    playdate.timer.updateTimers()
    if playdate.buttonJustPressed(playdate.kButtonA) then
        -- Shake the screen for 500ms, with the screen
        -- shaking around by about 5 pixels on each side
        screenShake(500, 5)
    end
    -- A circle to be able to view what the shaking looks like
    playdate.graphics.fillCircleAtPoint(200, 120, 10)
end
```

### playdate.display.setRefreshRate

```lua
playdate.display.setRefreshRate(rate: number): nil
```

Sets the desired refresh rate in frames per second. The default is 30 fps, which is a recommended figure that balances animation smoothness with performance and power considerations. Maximum is 50 fps.
If *rate* is 0, playdate.update() is called as soon as possible. Since the display refreshes line-by-line, and unchanged lines aren’t sent to the display, the update cycle will be faster than 30 times a second but at an indeterminate rate. playdate.getCurrentTimeMilliseconds() should then be used as a steady time base.
Equivalent to `playdate->display->setRefreshRate()` in the C API.

### playdate.display.setScale

```lua
playdate.display.setScale(scale: integer): nil
```

Sets the display scale factor. Valid values for *scale* are 1, 2, 4, and 8.
The top-left corner of the frame buffer is scaled up to fill the display; e.g., if the scale is set to 4, the pixels in rectangle [0,100] x [0,60] are drawn on the screen as 4 x 4 squares.
Equivalent to `playdate->display->setScale()` in the C API.
