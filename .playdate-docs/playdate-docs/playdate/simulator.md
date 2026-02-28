# playdate.simulator

## Functions

### playdate.clearConsole

```lua
playdate.clearConsole(): nil
```

Clears the simulator console.

### playdate.debugDraw

```lua
playdate.debugDraw(): nil
```

Called immediately after playdate.update(), any drawing performed during this callback is overlaid on the display in 50% transparent red (or another color selected with playdate.setDebugDrawColor()).
White pixels are drawn in the debugDrawColor. Black pixels are transparent.

### playdate.keyPressed

```lua
playdate.keyPressed(key: string): nil
```

Lets you act on keyboard keypresses when running in the Simulator ONLY. These can be useful for adding debugging functions that can be enabled via your keyboard.
It is possible test a game on Playdate hardware and trap computer keyboard keypresses if you are using the Simulator’s `Control Device with Simulator` option.
`key` is a string containing the character pressed or released on the keyboard. Note that:
* The key in question needs to have a textual representation or these functions will not be called. For instance, alphanumeric keys will call these functions; keyboard directional arrows will not.
* If the keypress in question is already in use by the Simulator for another purpose (say, to control the d-pad or A/B buttons), these functions will not be called.
* If *key* is an alphabetic character, the value will always be lowercase, even if the user deliberately typed an uppercase character.

### playdate.keyReleased

```lua
playdate.keyReleased(key: string): nil
```

Lets you act on keyboard key releases when running in the Simulator ONLY. These can be useful for adding debugging functions that can be enabled via your keyboard.

### playdate.setDebugDrawColor

```lua
playdate.setDebugDrawColor(r: number, g: number, b: number, a: number): nil
```

Sets the color of the playdate.debugDraw() overlay image. Values are in the range 0-1.

### playdate.simulator.exit

```lua
playdate.simulator.exit(): nil
```

Quits the Playdate Simulator app.

### playdate.simulator.getURL

```lua
playdate.simulator.getURL(url: string): string
```

Returns the contents of the URL *url* as a string.

### playdate.simulator.writeToFile

```lua
playdate.simulator.writeToFile(image: _Image, path: string): nil
```

Writes an image to a PNG file at the path specified. Only available on the Simulator.
*path* represents a path on your development computer, not the Playdate filesystem. It’s recommended you prefix your path with `~/` to ensure you are writing to a writeable directory, for example, `~/myImageFile.png`. Please include the `.png` file extension in your path name. Any directories in your path must already exist on your development computer in order for the file to be written.

### playdate.simulator.openURL

```lua
playdate.simulator.openURL(url: string): nil
```
