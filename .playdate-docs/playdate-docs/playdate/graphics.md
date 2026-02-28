# playdate.graphics

## Functions

### playdate.graphics.clear

```lua
playdate.graphics.clear(color: integer): nil
```

Clears the entire display, setting the color to either the given *color* argument, or the current background color set in setBackgroundColor(color) if no argument is given.
Equivalent to `playdate->graphics->clear()` in the C API.

### playdate.graphics.clearClipRect

```lua
playdate.graphics.clearClipRect(): nil
```

Clears the current clipping rectangle, set with `setClipRect()`.
Equivalent to `playdate->graphics->clearClipRect()` in the C API.

### playdate.graphics.clearStencil

```lua
playdate.graphics.clearStencil(): nil
```

Clears the stencil buffer.

### playdate.graphics.clearStencilImage

```lua
playdate.graphics.clearStencilImage(): nil
```

*Deprecated.*
Clears the stencil buffer.

### playdate.graphics.drawArc

```lua
playdate.graphics.drawArc(arc: _Arc): nil
playdate.graphics.drawArc(x: integer, y: integer, radius: number, startAngle: number, endAngle: number): nil
```

Draws an arc using the current color.
Angles are specified in degrees, not radians.

### playdate.graphics.drawCircleAtPoint

```lua
playdate.graphics.drawCircleAtPoint(p: _Point, radius: number): nil
playdate.graphics.drawCircleAtPoint(x: integer, y: integer, radius: number): nil
```

Draws a circle at the point *(x, y)* (or *p*) with radius *radius*.

### playdate.graphics.drawCircleInRect

```lua
playdate.graphics.drawCircleInRect(r: _Rect): nil
playdate.graphics.drawCircleInRect(x: integer, y: integer, width: integer, height: integer): nil
```

Draws a circle in the rect *r* or the rect with origin *(x, y)* and size *(width, height)*.
If the rect is not a square, the circle will be drawn centered in the rect.

### playdate.graphics.drawEllipseInRect

```lua
playdate.graphics.drawEllipseInRect(rect: _Rect, startAngle: number, endAngle: number): nil
playdate.graphics.drawEllipseInRect(x: integer, y: integer, width: integer, height: integer, startAngle: number, endAngle: number): nil
```

Draws an ellipse in the rect *r* or the rect with origin *(x, y)* and size *(width, height)*.
*startAngle* and *endAngle*, if provided, should be in degrees (not radians), and will cause only the segment of the ellipse between *startAngle* and *endAngle* to be drawn.

### playdate.graphics.drawLine

```lua
playdate.graphics.drawLine(ls: _LineSegment): nil
playdate.graphics.drawLine(x1: integer, y1: integer, x2: integer, y2: integer): nil
```

Draws a line from (*x1*, *y1*) to (*x2*, *y2*), or draws the playdate.geometry.lineSegment *ls*.
Line width is specified by setLineWidth(). End cap style is specified by setLineCapStyle().
Equivalent to `playdate->graphics->drawLine()` in the C API.

### playdate.graphics.drawPixel

```lua
playdate.graphics.drawPixel(x: integer, y: integer): nil
```

Draw a single pixel in the current color at (*x*, *y*).
`**playdate.graphics.drawPixel(p)**`
Draw a single pixel in the current color at playdate.geometry.point *p*.

### playdate.graphics.drawPolygon

```lua
playdate.graphics.drawPolygon(p: _Polygon): nil
playdate.graphics.drawPolygon(x1: integer, y1: integer, x2: integer, y2: integer, ...: integer): nil
```

Draw the playdate.geometry.polygon *p*. Only draws a line between the first and last vertex if the polygon is closed.
Line width is specified by setLineWidth().

### playdate.graphics.drawRect

```lua
playdate.graphics.drawRect(r: _Rect): nil
playdate.graphics.drawRect(x: integer, y: integer, w: integer, h: integer): nil
```

Draws the rect *r* or the rect with origin (*x*, *y*) with a size of (*w*, *h*).
Line width is specified by setLineWidth(). Stroke location is specified by setStrokeLocation().
Equivalent to `playdate->graphics->drawRect()` in the C API.

### playdate.graphics.drawRoundRect

```lua
playdate.graphics.drawRoundRect(r: _Rect, radius: number): nil
playdate.graphics.drawRoundRect(x: integer, y: integer, w: integer, h: integer, radius: number): nil
```

Draws a rectangle with rounded corners in the rect *r* or the rect with origin (*x*, *y*) and size (*w*, *h*).
*radius* defines the radius of the corners.

### playdate.graphics.drawSineWave

```lua
playdate.graphics.drawSineWave(startX: integer, startY: integer, endX: integer, endY: integer, startAmplitude: integer, endAmplitude: integer, period: integer, phaseShift: integer): nil
```

You must import *CoreLibs/graphics* to use this function.
Draws an approximation of a sine wave between the points *startX, startY* and *endX, endY*.
* *startAmplitude*: The number of pixels above and below the line from *startX, startY* and *endX, endY* the peaks and valleys of the wave will be drawn at the start of the wave.
* *endAmplitude*: The number of pixels above and below the line from *startX, startY* and *endX, endY* the peaks and valleys of the wave will be drawn at the end of the wave.
* *period*: The distance between peaks, in pixels.
* *phaseShift*: If provided, specifies the wave’s offset, in pixels.

### playdate.graphics.drawTriangle

```lua
playdate.graphics.drawTriangle(x1: integer, y1: integer, x2: integer, y2: integer, x3: integer, y3: integer): nil
```

Draws a triangle with vertices (*x1*, *y1*), (*x2*, *y2*), and (*x3*, *y3*).

### playdate.graphics.fillCircleAtPoint

```lua
playdate.graphics.fillCircleAtPoint(p: _Point, radius: number): nil
playdate.graphics.fillCircleAtPoint(x: integer, y: integer, radius: number): nil
```

Draws a filled circle at the point *(x, y)* (or *p*) with radius *radius*.

### playdate.graphics.fillCircleInRect

```lua
playdate.graphics.fillCircleInRect(r: _Rect): nil
playdate.graphics.fillCircleInRect(x: integer, y: integer, width: integer, height: integer): nil
```

Draws a filled circle in the rect *r* or the rect with origin *(x, y)* and size *(width, height)*.
If the rect is not a square, the circle will be drawn centered in the rect.

### playdate.graphics.fillEllipseInRect

```lua
playdate.graphics.fillEllipseInRect(rect: _Rect, startAngle: number, endAngle: number): nil
playdate.graphics.fillEllipseInRect(x: integer, y: integer, width: integer, height: integer, startAngle: number, endAngle: number): nil
```

Draws a filled ellipse in the rect *r* or the rect with origin *(x, y)* and size *(width, height)*.
*startAngle* and *endAngle*, if provided, should be in degrees (not radians), and will cause only the segment of the ellipse between *startAngle* and *endAngle* to be drawn.

### playdate.graphics.fillPolygon

```lua
playdate.graphics.fillPolygon(p: _Polygon): nil
playdate.graphics.fillPolygon(x1: integer, y1: integer, x2: integer, y2: integer, ...: integer): nil
```

Fills the polygon specified by the playdate.geometry.polygon *p* with the currently selected color or pattern. The function throws an error if the polygon is not closed.

### playdate.graphics.fillRect

```lua
playdate.graphics.fillRect(r: _Rect): nil
playdate.graphics.fillRect(x: integer, y: integer, width: integer, height: integer): nil
```

Draws the filled rectangle *r* or the rect at (*x*, *y*) of the given width and height.
Equivalent to `playdate->graphics->fillRect()` in the C API.

### playdate.graphics.fillRoundRect

```lua
playdate.graphics.fillRoundRect(r: _Rect, radius: number): nil
playdate.graphics.fillRoundRect(x: integer, y: integer, w: integer, h: integer, radius: number): nil
```

Draws a filled rectangle with rounded corners in the rect *r* or the rect with origin (*x*, *y*) and size (*w*, *h*).
*radius* defines the radius of the corners.

### playdate.graphics.fillTriangle

```lua
playdate.graphics.fillTriangle(x1: integer, y1: integer, x2: integer, y2: integer, x3: integer, y3: integer): nil
```

Draws a filled triangle with vertices (*x1*, *y1*), (*x2*, *y2*), and (*x3*, *y3*).
Equivalent to `playdate->graphics->fillTriangle()` in the C API.

### playdate.graphics.getBackgroundColor

```lua
playdate.graphics.getBackgroundColor(): integer
```

Gets the color used for drawing the background, if necessary, before playdate.graphics.sprites are drawn on top.

### playdate.graphics.getClipRect

```lua
playdate.graphics.getClipRect(): (integer, integer, integer, integer)
```

`getClipRect()` returns multiple values (*x*, *y*, *width*, *height*) giving the current clipping rectangle.

### playdate.graphics.getColor

```lua
playdate.graphics.getColor(): integer
```

Gets the current drawing color for primitives.

### playdate.graphics.getDisplayImage

```lua
playdate.graphics.getDisplayImage(): _Image
```

Returns a copy the contents of the *last completed frame*, i.e., a "screenshot", as a playdate.graphics.image.
Display functions like setMosaic(), setInverted(), setScale(), and setOffset() do not affect the returned image.

### playdate.graphics.getDrawOffset

```lua
playdate.graphics.getDrawOffset(): (integer, integer)
```

`getDrawOffset()` returns multiple values (*x*, *y*) giving the current draw offset.

### playdate.graphics.getImageDrawMode

```lua
playdate.graphics.getImageDrawMode(): integer
```

Gets the current drawing mode for images.

### playdate.graphics.getLineWidth

```lua
playdate.graphics.getLineWidth(): integer
```

Gets the current line width.

### playdate.graphics.getScreenClipRect

```lua
playdate.graphics.getScreenClipRect(): (integer, integer, integer, integer)
```

Returns the clip rect as in `getClipRect()`, but using screen coordinates instead of world coordinates.

### playdate.graphics.getStrokeLocation

```lua
playdate.graphics.getStrokeLocation(): integer
```

Gets the current stroke position.

### playdate.graphics.getWorkingImage

```lua
playdate.graphics.getWorkingImage(): _Image
```

Returns a copy the contents of the working frame buffer — *the current frame, in-progress* — as a playdate.graphics.image.
Display functions like setMosaic(), setInverted(), setScale(), and setOffset() do not affect the returned image.

### playdate.graphics.lockFocus

```lua
playdate.graphics.lockFocus(image: _Image): nil
```

`lockFocus()` routes all drawing to the given playdate.graphics.image. playdate.graphics.unlockFocus() returns drawing to the frame buffer.
If you draw into an image with color set to *playdate.graphics.kColorClear*, those drawn pixels will be set to transparent. When you later draw the image into the framebuffer, those pixels will not be rendered, i.e., will act as transparent pixels in the image.
playdate.graphics.pushContext(*image*) will also allow offscreen drawing into an image, with the additional benefit of being able to save and restore the graphics state.

### playdate.graphics.popContext

```lua
playdate.graphics.popContext(): nil
```

Pops a graphics context off the context stack and restores its state.
Equivalent to `playdate->graphics->popContext()` in the C API.

### playdate.graphics.pushContext

```lua
playdate.graphics.pushContext(image: _Image): nil
```

Pushes the current graphics state to the context stack and creates a new context. If a playdate.graphics.image is given, drawing functions are applied to the image instead of the screen buffer.
If you draw into an image context with color set to *playdate.graphics.kColorClear*, those drawn pixels will be set to transparent. When you later draw the image into the framebuffer, those pixels will not be rendered, i.e., will act as transparent pixels in the image.
playdate.graphics.lockFocus(*image*) will reroute drawing into an image, without saving the overall graphics context.
Equivalent to `playdate->graphics->pushContext()` in the C API.

### playdate.graphics.setBackgroundColor

```lua
playdate.graphics.setBackgroundColor(color: integer): nil
```

Sets the color used for drawing the background, if necessary, before playdate.graphics.sprites are drawn on top.
*color* should be one of the constants:
* *playdate.graphics.kColorBlack*
* *playdate.graphics.kColorWhite*
* *playdate.graphics.kColorClear*
Use *kColorClear* if you intend to draw behind sprites.
Equivalent to `playdate->graphics->setBackgroundColor()` in the C API.

### playdate.graphics.setClipRect

```lua
playdate.graphics.setClipRect(rect: _Rect): nil
playdate.graphics.setClipRect(x: integer, y: integer, width: integer, height: integer): nil
```

`setClipRect()` sets the clipping rectangle for all subsequent graphics drawing, including bitmaps. The argument can either be separate dimensions or a playdate.geometry.rect object. The clip rect is automatically cleared at the beginning of the `playdate.update()` callback. The function uses world coordinates; that is, the given rectangle will be translated by the current drawing offset. To use screen coordinates instead, use `setScreenClipRect()`

### playdate.graphics.setColor

```lua
playdate.graphics.setColor(color: integer): nil
```

Sets and gets the current drawing color for primitives.
*color* should be one of the constants:
* *playdate.graphics.kColorBlack*
* *playdate.graphics.kColorWhite*
* *playdate.graphics.kColorClear*
* *playdate.graphics.kColorXOR*
This color applies to drawing primitive shapes such as lines and rectangles, not bitmap images.
`setColor()` and `setPattern()` / `setDitherPattern()` are mutually exclusive. Setting a color will overwrite a pattern, and vice versa.

### playdate.graphics.setDitherPattern

```lua
playdate.graphics.setDitherPattern(alpha: number, ditherType: integer): nil
```

Sets the pattern used for drawing to a dithered pattern. If the current drawing color is white, the pattern is white pixels on a transparent background and (due to a bug) the *alpha* value is inverted: 1.0 is transparent and 0 is opaque. Otherwise, the pattern is black pixels on a transparent background and *alpha* 0 is transparent while 1.0 is opaque.
The optional *ditherType* argument is a dither type as used in `playdate.graphics.image:blurredImage()`, and should be an ordered dither type; i.e., line, screen, or Bayer.
The error-diffusing dither types Floyd-Steinberg (`kDitherTypeFloydSteinberg`), Burkes (`kDitherTypeBurkes`), and Atkinson (`kDitherTypeAtkinson`) are allowed but produce very unpredictable results here.

### playdate.graphics.setDrawOffset

```lua
playdate.graphics.setDrawOffset(x: integer, y: integer): nil
```

`setDrawOffset(x, y)` offsets the origin point for all drawing calls to *x*, *y* (can be negative). So, for example, if the offset is set to -20, -20, an image drawn at 20, 20 will appear at the origin (in the upper left corner.)
This is useful, for example, for centering a "camera" on a sprite that is moving around a world larger than the screen.
The *x* and *y* arguments to `.setDrawOffset()` are always specified in the original, unaltered coordinate system. So, for instance, repeated calls to `playdate.graphics.setDrawOffset(-10, -10)` will leave the draw offset unchanged. Likewise, `.setDrawOffset(0, 0)` will always "disable" the offset.
It can be useful to have operations sometimes ignore the draw offsets. For example, you may want to have the score or some other heads-up display appear onscreen apart from scrolling content. A sprite can be set to ignore offsets by calling playdate.graphics.sprite:setIgnoresDrawOffset(true). playdate.graphics.image:drawIgnoringOffsets() lets you render an image using screen coordinates.
Equivalent to `playdate->graphics->setDrawOffset()` in the C API.

### playdate.graphics.setImageDrawMode

```lua
playdate.graphics.setImageDrawMode(mode: integer): nil
```

Sets the current drawing mode for images.
The draw mode applies to images and fonts (which are technically images). The draw mode does not apply to primitive shapes such as lines or rectangles.
The available options for *mode* (demonstrated by drawing a two-color background image first, setting the specified draw mode, then drawing the Crankin' character on top) are:
* *playdate.graphics.kDrawModeCopy*: Images are drawn exactly as they are (black pixels are drawn black and white pixels are drawn white)
* *playdate.graphics.kDrawModeWhiteTransparent*: Any white portions of an image are drawn transparent (black pixels are drawn black and white pixels are drawn transparent)
* *playdate.graphics.kDrawModeBlackTransparent*: Any black portions of an image are drawn transparent (black pixels are drawn transparent and white pixels are drawn white)
* *playdate.graphics.kDrawModeFillWhite*: All non-transparent pixels are drawn white (black pixels are drawn white and white pixels are drawn white)
* *playdate.graphics.kDrawModeFillBlack*: All non-transparent pixels are drawn black (black pixels are drawn black and white pixels are drawn black)
* *playdate.graphics.kDrawModeXOR*: Pixels are drawn inverted on white backgrounds, creating an effect where any white pixels in the original image will always be visible, regardless of the background color, and any black pixels will appear transparent (on a white background, black pixels are drawn white and white pixels are drawn black)
* *playdate.graphics.kDrawModeNXOR*: Pixels are drawn inverted on black backgrounds, creating an effect where any black pixels in the original image will always be visible, regardless of the background color, and any white pixels will appear transparent (on a black background, black pixels are drawn white and white pixels are drawn black)
* *playdate.graphics.kDrawModeInverted*: Pixels are drawn inverted (black pixels are drawn white and white pixels are drawn black)
Instead of the above-specified constants, you can also use one of the following strings: "copy", "inverted", "XOR", "NXOR", "whiteTransparent", "blackTransparent", "fillWhite", or "fillBlack".
Equivalent to `playdate->graphics->setDrawMode()` in the C API.

### playdate.graphics.setLineCapStyle

```lua
playdate.graphics.setLineCapStyle(style: integer): nil
```

Specifies the shape of the endpoints drawn by drawLine.
*style* should be one of these constants:
* *playdate.graphics.kLineCapStyleButt*
* *playdate.graphics.kLineCapStyleRound*
* *playdate.graphics.kLineCapStyleSquare*
Equivalent to `playdate->graphics->setLineCapStyle()` in the C API.

### playdate.graphics.setLineWidth

```lua
playdate.graphics.setLineWidth(width: integer): nil
```

Sets the width of the line for drawLine, drawRect, drawPolygon, and drawArc when a playdate.geometry.arc is passed as the argument. This value is saved and restored when pushing and popping the graphics context.

### playdate.graphics.setPattern

```lua
playdate.graphics.setPattern(pattern: integer[]): nil
playdate.graphics.setPattern(image: _Image, x: integer, y: integer): nil
```

Sets the 8x8 pattern used for drawing. The *pattern* argument is an array of 8 numbers describing the bitmap for each row; for example, *{ 0xaa, 0x55, 0xaa, 0x55, 0xaa, 0x55, 0xaa, 0x55 }* specifies a checkerboard pattern. An additional 8 numbers can be specified for an alpha mask bitmap.
To "un-set" a pattern, call `setColor()`. `setColor()` and `setPattern()` / `setDitherPattern()` are mutually exclusive. Setting a pattern will overwrite a color, and vice versa.
`**playdate.graphics.setPattern(image, [x, y])**`
Uses the given playdate.graphics.image to set the 8 x 8 pattern used for drawing. The optional *x*, *y* offset (default 0, 0) indicates the top left corner of the 8 x 8 pattern.

### playdate.graphics.setPolygonFillRule

```lua
playdate.graphics.setPolygonFillRule(rule: integer): nil
```

Sets the winding rule for filling polygons, one of:
* *playdate.graphics.kPolygonFillNonZero*
* *playdate.graphics.kPolygonFillEvenOdd*
See https://en.wikipedia.org/wiki/Nonzero-rule for an explanation of the winding rule.

### playdate.graphics.setScreenClipRect

```lua
playdate.graphics.setScreenClipRect(rect: _Rect): nil
playdate.graphics.setScreenClipRect(x: integer, y: integer, width: integer, height: integer): nil
```

Sets the clip rectangle as above, but uses screen coordinates instead of world coordinates—​that is, it ignores the current drawing offset.
Equivalent to `playdate->graphics->setScreenClipRect()` in the C API.

### playdate.graphics.setStencilImage

```lua
playdate.graphics.setStencilImage(image: _Image, tile: boolean): nil
```

Sets the current stencil to the given image. While the stencil is active, drawing functions will only draw pixels where the stencil is white and nothing is drawn where the stencil is black. If *tile* is set, the the stencil will be tiled; in this case, the image width must be a multiple of 32 pixels.
Equivalent to `playdate->graphics->setStencilImage()` in the C API.

### playdate.graphics.setStencilPattern

```lua
playdate.graphics.setStencilPattern(level: any, ditherType: integer): nil
playdate.graphics.setStencilPattern(pattern: integer[]): nil
playdate.graphics.setStencilPattern(row1: integer, row2: integer, row3: integer, row4: integer, row5: integer, row6: integer, row7: integer, row8: integer): nil
```

Sets the stencil to a dither pattern specified by *level* and optional *ditherType* (defaults to `playdate.graphics.image.kDitherTypeBayer8x8`).

### playdate.graphics.setStrokeLocation

```lua
playdate.graphics.setStrokeLocation(location: integer): nil
```

Specifies where the stroke is placed relative to the rectangle passed into drawRect.
*location* is one of these constants:
* *playdate.graphics.kStrokeCentered*
* *playdate.graphics.kStrokeOutside*
* *playdate.graphics.kStrokeInside*
This value is saved and restored when pushing and popping the graphics context.

### playdate.graphics.unlockFocus

```lua
playdate.graphics.unlockFocus(): nil
```

After calling `unlockFocus()`, drawing is routed to the frame buffer.

## Classes

### playdate.graphics

```lua
---@class playdate.graphics
---@field kAlignLeft integer 33554432
---@field kAlignRight integer 33554434
---@field kAlignCenter integer 33554433
---@field kColorBlack integer 0
---@field kColorWhite integer 1
---@field kColorClear integer 2
---@field kColorXOR integer 3
---@field kDrawModeCopy integer 0
---@field kDrawModeWhiteTransparent integer 1
---@field kDrawModeBlackTransparent integer 2
---@field kDrawModeFillWhite integer 3
---@field kDrawModeFillBlack integer 4
---@field kDrawModeXOR integer 5
---@field kDrawModeNXOR integer 6
---@field kDrawModeInverted integer 7
---@field kImageUnflipped integer 0
---@field kImageFlippedX integer 1
---@field kImageFlippedY integer 2
---@field kImageFlippedXY integer 3
---@field kLineCapStyleButt integer 0
---@field kLineCapStyleSquare integer 1
---@field kLineCapStyleRound integer 2
---@field kPolygonFillNonZero integer 0
---@field kPolygonFillEvenOdd integer 1
---@field kStrokeCentered integer 0
---@field kStrokeInside integer 1
---@field kStrokeOutside integer 2
---@field kWrapClip integer 16777216
---@field kWrapCharacter integer 16777217
---@field kWrapWord integer 16777218
```

## See Also:

- [animation](graphics/animation.md)
- [animator](graphics/animator.md)
- [font](graphics/font.md)
- [image](graphics/image.md)
- [imagetable](graphics/imagetable.md)
- [nineSlice](graphics/nineSlice.md)
- [perlin](graphics/perlin.md)
- [qrcode](graphics/qrcode.md)
- [sprite](graphics/sprite.md)
- [tilemap](graphics/tilemap.md)
- [video](graphics/video.md)
