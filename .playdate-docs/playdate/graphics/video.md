# playdate.graphics.video

## Functions

### playdate.graphics.video.new

```lua
playdate.graphics.video.new(path: string): _Video
```

Returns a playdate.graphics.video object from the pdv file at *path*. If the file at *path* can’t be opened, the function returns nil.

### playdate.graphics.video:getContext

```lua
playdate.graphics.video:getContext(): _Image
```

Returns the image into which the video will be rendered, creating it if needed.

### playdate.graphics.video:getCurrentFrame

```lua
playdate.graphics.video:getCurrentFrame(): integer
```

Returns the frame number of the currently displayed frame.

### playdate.graphics.video:getFrameCount

```lua
playdate.graphics.video:getFrameCount(): integer
```

Returns the number of frames in the video.

### playdate.graphics.video:getFrameRate

```lua
playdate.graphics.video:getFrameRate(): number
```

Returns the number of frames per second of the video source. This number is simply for record-keeping, it is not used internally—​the game code is responsible for figuring out which frame to show when.

### playdate.graphics.video:getSize

```lua
playdate.graphics.video:getSize(): (integer, integer)
```

Returns the width and height of the video as multiple vlaues (*width*, *height*).

### playdate.graphics.video:renderFrame

```lua
playdate.graphics.video:renderFrame(number: integer): nil
```

Draws the given frame into the video’s render context.

### playdate.graphics.video:setContext

```lua
playdate.graphics.video:setContext(image: _Image): nil
```

Sets the given image to the video render context. Future `video:renderFrame()` calls will draw into this image.

### playdate.graphics.video:useScreenContext

```lua
playdate.graphics.video:useScreenContext(): nil
```

Sets the display framebuffer as the video’s render context.
