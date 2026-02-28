# playdate.lifecycle

## Functions

### playdate.gameWillResume

```lua
playdate.gameWillResume(): nil
```

Called before the system resumes the game.

### playdate.gameWillTerminate

```lua
playdate.gameWillTerminate(): nil
```

Called when the player chooses to exit the game via the System Menu or Menu button.

### playdate.restart

```lua
playdate.restart(arg: string): nil
```

Reinitializes the Playdate runtime and restarts the currently running game. The optional string `arg` passed in is available after restart in playdate.argv as if it had been passed in on the command line when launching the simulator. The `arg` string will be split on spaces, but respecting quotes, when added to the argv list.

### playdate.serialMessageReceived

```lua
playdate.serialMessageReceived(message: string): nil
```

Called when a `msg {text}` command is received on the serial port. The text following the command is passed to the function as the string *message*.
Running `!msg {message}` in the simulator Lua console sends the command to the device if one is connected, otherwise it sends it to the game running in the simulator.

### playdate.start

```lua
playdate.start(): nil
```

Resumes per-frame callbacks to playdate.update().

### playdate.stop

```lua
playdate.stop(): nil
```

Stops per-frame callbacks to playdate.update(). Useful in conjunction with playdate.display.flush() if your program only does things in response to button presses.

### playdate.update

```lua
playdate.update(): nil
```

Implement this callback and Playdate OS will call it once per frame. This is the place to put the main update-and-draw code for your game. Playdate will attempt to call this function by default 30 times per second; that value can be changed by calling playdate.display.setRefreshRate().
If your `update()` function takes too long to execute, Playdate OS may not be able to call it as often as specified by the current refresh rate. In this case, Playdate OS will simply try and call it as often as it can, with a not-to-exceed rate of playdate.display.getRefreshRate() frames per second.

### playdate.wait

```lua
playdate.wait(milliseconds: integer): nil
```

Suspends callbacks to `playdate.update()` for the specified number of milliseconds.
`playdate.wait()` is ideal for pausing game execution to, for example, show a message to the player. Because `.update()` will not be called, the screen will freeze during `.wait()`. Audio will continue to play. Animation during this wait period is possible, but you will need to explicitly call `playdate.display.flush()` once per frame.
While timers should pause during `playdate.wait()` (assuming `playdate.timer.updateTimers()` and `playdate.frameTimer.updateTimers()` are invoked during `playdate.update()`), animators will *not* pause during `playdate.wait()`. Be sure to account for this in your code.
