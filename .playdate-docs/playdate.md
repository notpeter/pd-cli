# playdate

## Functions

### playdate.apiVersion

```lua
playdate.apiVersion(): (integer, integer)
```

Returns two values, the current API version of the Playdate runtime and the minimum API version supported by the runtime.

### playdate.drawFPS

```lua
playdate.drawFPS(x: integer, y: integer): nil
```

Calculates the current frames per second and draws that value at *x, y*.

### playdate.getFPS

```lua
playdate.getFPS(): number
```

Returns the *measured, actual* refresh rate in frames per second. This value may be different from the *specified* refresh rate (see playdate.display.getRefreshRate()) by a little or a lot depending upon how much calculation is being done per frame.

## Classes

### _Metadata

```lua
---@class _Metadata
---@field name string
---@field author string
---@field description string
---@field bundleID string
---@field version string
---@field buildNumber integer
---@field pdxversion integer
---@field imagePath? string
---@field launchSoundPath? string
---@field contentWarning? string
---@field contentWarning2? string
```

### _SystemInfo

```lua
---@class _SystemInfo
---@field buildtime string
---@field commit string
---@field pdxcompatversion integer
---@field pdxversion integer
---@field sdk string
```

### playdate

```lua
---@class playdate
---@field argv string[]
---@field isSimulator boolean
---@field kButtonLeft integer 1
---@field kButtonRight integer 2
---@field kButtonUp integer 4
---@field kButtonDown integer 8
---@field kButtonB integer 16
---@field kButtonA integer 32
---@field metadata _Metadata
---@field systeminfo _SystemInfo
```

## See Also:

- [accelerometer](playdate/accelerometer.md)
- [datastore](playdate/datastore.md)
- [device](playdate/device.md)
- [display](playdate/display.md)
- [easingFunctions](playdate/easingFunctions.md)
- [file](playdate/file.md)
- [frameTimer](playdate/frameTimer.md)
- [geometry](playdate/geometry.md)
- [graphics](playdate/graphics.md)
- [input](playdate/input.md)
- [inputhandlers](playdate/inputhandlers.md)
- [keyboard](playdate/keyboard.md)
- [lifecycle](playdate/lifecycle.md)
- [math](playdate/math.md)
- [menu](playdate/menu.md)
- [mirror](playdate/mirror.md)
- [network](playdate/network.md)
- [pathfinder](playdate/pathfinder.md)
- [profiling](playdate/profiling.md)
- [scoreboards](playdate/scoreboards.md)
- [settings](playdate/settings.md)
- [simulator](playdate/simulator.md)
- [sound](playdate/sound.md)
- [string](playdate/string.md)
- [time](playdate/time.md)
- [timer](playdate/timer.md)
- [ui](playdate/ui.md)
