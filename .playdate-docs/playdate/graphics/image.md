# playdate.graphics.image

## Functions

### playdate.graphics.checkAlphaCollision

```lua
playdate.graphics.checkAlphaCollision(image1: _Image, x1: integer, y1: integer, flip1: integer, image2: _Image, x2: integer, y2: integer, flip2: integer): boolean
```

Returns true if the non-alpha-masked portions of *image1* and *image2* overlap if they were drawn at positions (*x1*, *y1*) and (*x2*, *y2*) and flipped according to *flip1* and *flip2*, which should each be one of the values listed in `playdate.graphics.image:draw()`.

### playdate.graphics.image.new

```lua
playdate.graphics.image.new(path: string): (_Image?, string?)
playdate.graphics.image.new(width: integer, height: integer, bgcolor: integer): _Image
```

Returns a playdate.graphics.image object from the data at *path*. If there is no file at *path*, the function returns nil and a second value describing the error.

### playdate.graphics.image:addMask

```lua
playdate.graphics.image:addMask(opaque: boolean): nil
```

Adds a mask to the image if it doesn’t already have one. If *opaque* is `true` or not specified, the image mask applied will be completely white, so the image will be entirely opaque. If *opaque* is `false`, the mask will be completely black, so the image will be entirely transparent.

### playdate.graphics.image:blendWithImage

```lua
playdate.graphics.image:blendWithImage(image: _Image, alpha: number, ditherType: integer): _Image
```

Returns an image that is a blend between the caller and *image*.
* *image*: the playdate.graphics.image to be blended with the caller.
* *alpha*: The alpha value assigned to the caller. *image* will have an alpha of (1 - *alpha*).
* *ditherType*: The caller and *image* are blended into a greyscale image and dithered with one of the dithering algorithms listed in `playdate.graphics.image:blurredImage()`

### playdate.graphics.image:blurredImage

```lua
playdate.graphics.image:blurredImage(radius: number, numPasses: integer, ditherType: integer, padEdges: boolean, xPhase: integer, yPhase: integer): _Image
```

Returns a blurred copy of the caller.
* *radius*: A bigger radius means a more blurred result. Processing time is independent of the radius.
* *numPasses*: A box blur is used to blur the image. The more passes, the more closely the blur approximates a gaussian blur. However, higher values will take more time to process.
* *ditherType*: The original image is blurred into a greyscale image then dithered back to 1-bit using one of the following dithering algorithms:
  * *playdate.graphics.image.kDitherTypeNone*
  * *playdate.graphics.image.kDitherTypeDiagonalLine*
  * *playdate.graphics.image.kDitherTypeVerticalLine*
  * *playdate.graphics.image.kDitherTypeHorizontalLine*
  * *playdate.graphics.image.kDitherTypeScreen*
  * *playdate.graphics.image.kDitherTypeBayer2x2*
  * *playdate.graphics.image.kDitherTypeBayer4x4*
  * *playdate.graphics.image.kDitherTypeBayer8x8*
  * *playdate.graphics.image.kDitherTypeFloydSteinberg*
  * *playdate.graphics.image.kDitherTypeBurkes*
  * *playdate.graphics.image.kDitherTypeAtkinson*
* *padEdges*: Boolean indicating whether the edges of the images should be padded to accommodate the blur radius. Defaults to false.
* *xPhase*, *yPhase*: optional; integer values that affect the appearance of *playdate.graphics.image.kDitherTypeDiagonalLine*,  *playdate.graphics.image.kDitherTypeVerticalLine*, *playdate.graphics.image.kDitherTypeHorizontalLine*,  *playdate.graphics.image.kDitherTypeScreen*, *playdate.graphics.image.kDitherTypeBayer2x2*, *playdate.graphics.image.kDitherTypeBayer4x4*, and *playdate.graphics.image.kDitherTypeBayer8x8*.

### playdate.graphics.image:clear

```lua
playdate.graphics.image:clear(color: integer): nil
```

Erases the contents of the image, setting all pixels to white if *color* is *playdate.graphics.kColorWhite*, black if it’s *playdate.graphics.kColorBlack*, or clear if it’s *playdate.graphics.kColorClear*. If the image is cleared to black or white, the mask (if it exists) is set to fully opaque. If the image is cleared to kColorClear and the image doesn’t have a mask, a mask is added to it.

### playdate.graphics.image:clearMask

```lua
playdate.graphics.image:clearMask(opaque: boolean): nil
```

Erases the contents of the image’s mask, so that the image is entirely opaque if *opaque* is 1, transparent otherwise. This function has no effect if the image doesn’t have a mask.

### playdate.graphics.image:copy

```lua
playdate.graphics.image:copy(): _Image
```

Returns a new `playdate.graphics.image` that is an exact copy of the original.

### playdate.graphics.image:draw

```lua
playdate.graphics.image:draw(p: _Point, flip: (integer|string), sourceRect: _Rect): nil
playdate.graphics.image:draw(x: integer, y: integer, flip: (integer|string), sourceRect: _Rect): nil
```

Draws the image with its upper-left corner at location (*x*, *y*) or playdate.geometry.point *p*.
The optional *flip* argument can be one of the following:
* *playdate.graphics.kImageUnflipped*: the image is drawn normally
* *playdate.graphics.kImageFlippedX*: the image is flipped left to right
* *playdate.graphics.kImageFlippedY*: the image is flipped top to bottom
* *playdate.graphics.kImageFlippedXY*: the image if flipped both ways; i.e., rotated 180 degrees
Alternately, one of the strings "flipX", "flipY", or "flipXY" can be used for the *flip* argument.
*sourceRect*, if specified, will cause only the part of the image within sourceRect to be drawn. *sourceRect* should be relative to the image’s bounds and can be a playdate.geometry.rect or four integers, (*x*, *y*, *w*, *h*), representing the rect.

### playdate.graphics.image:drawAnchored

```lua
playdate.graphics.image:drawAnchored(x: integer, y: integer, ax: number, ay: number, flip: (integer|string)): nil
```

Draws the image at location *(x, y)* centered at the point within the image represented by *(ax, ay)* in unit coordinate space. For example, values of *ax = 0.0*, *ay = 0.0* represent the image’s top-left corner, *ax = 1.0*, *ay = 1.0* represent the bottom-right, and *ax = 0.5*, *ay = 0.5* represent the center of the image.
The *flip* argument is optional; see `playdate.graphics.image:draw()` for valid values.
You must import *CoreLibs/graphics* to use this method.

### playdate.graphics.image:drawBlurred

```lua
playdate.graphics.image:drawBlurred(x: integer, y: integer, radius: number, numPasses: integer, ditherType: integer, flip: (integer|string), xPhase: integer, yPhase: integer): nil
```

Draws a blurred version of the image at (*x*, *y*).
* *radius*: A bigger radius means a more blurred result. Processing time is independent of the radius.
* *numPasses*: A box blur is used to blur the image. The more passes, the more closely the blur approximates a gaussian blur. However, higher values will take more time to process.
* *ditherType*: The algorithm to use when blurring the image, must be one of the values listed in `playdate.graphics.image:blurredImage()`
* *flip*: optional; see `playdate.graphics.image:draw()` for valid values.
* *xPhase*, *yPhase*: optional; integer values that affect the appearance of *playdate.graphics.image.kDitherTypeDiagonalLine*,  *playdate.graphics.image.kDitherTypeVerticalLine*, *playdate.graphics.image.kDitherTypeHorizontalLine*,  *playdate.graphics.image.kDitherTypeScreen*, *playdate.graphics.image.kDitherTypeBayer2x2*, *playdate.graphics.image.kDitherTypeBayer4x4*, and *playdate.graphics.image.kDitherTypeBayer8x8*.

### playdate.graphics.image:drawCentered

```lua
playdate.graphics.image:drawCentered(x: integer, y: integer, flip: (integer|string)): nil
```

Draws the image centered at location *(x, y)*.
The *flip* argument is optional; see `playdate.graphics.image:draw()` for valid values.
You must import *CoreLibs/graphics* to use this method.

### playdate.graphics.image:drawFaded

```lua
playdate.graphics.image:drawFaded(x: integer, y: integer, alpha: number, ditherType: integer): nil
```

Draws a partially transparent image with its upper-left corner at location (*x*, *y*)
* *alpha*: The alpha value used to draw the image, with 1 being fully opaque, and 0 being completely transparent.
* *ditherType*: The caller is faded using one of the dithering algorithms listed in `playdate.graphics.image:blurredImage()`

### playdate.graphics.image:drawIgnoringOffset

```lua
playdate.graphics.image:drawIgnoringOffset(p: _Point, flip: (integer|string)): nil
playdate.graphics.image:drawIgnoringOffset(x: integer, y: integer, flip: (integer|string)): nil
```

Draws the image ignoring the currently-set `drawOffset`.

### playdate.graphics.image:drawRotated

```lua
playdate.graphics.image:drawRotated(x: integer, y: integer, angle: number, scale: number, yscale: number): nil
```

Draws this image centered at point *(x,y)* at (clockwise) *angle* degrees, scaled by optional argument *scale*, with an optional separate scaling for the y axis.

### playdate.graphics.image:drawSampled

```lua
playdate.graphics.image:drawSampled(x: integer, y: integer, width: integer, height: integer, centerx: number, centery: number, dxx: number, dyx: number, dxy: number, dyy: number, dx: integer, dy: integer, z: integer, tiltAngle: number, tile: boolean): nil
```

Draws the image as if it’s mapped onto a tilted plane, transforming the target coordinates to image coordinates using an affine transform:
```
x' = dxx * x + dyx * y + dx
y' = dxy * x + dyy * y + dy
```
* *x, y, width, height*: The rectangle to fill
* *centerx, centery*: The point in the above rectangle [in (0,1)x(0,1) coordinates] for the center of the transform
* *dxx, dyx, dxy, dyy, dx, dy*: Defines an affine transform from geometry coordinates to image coordinates
* *z*: The distance from the viewer to the target plane — lower z means more exaggerated perspective
* *tiltAngle*: The tilt of the target plane about the x axis, in degrees
* *tile*: A boolean, indicating whether the image is tiled on the target plane
The *Mode7Driver* demo in the */Examples* folder of the SDK demonstrates the usage of this function.

### playdate.graphics.image:drawScaled

```lua
playdate.graphics.image:drawScaled(x: integer, y: integer, scale: number, yscale: number): nil
```

Draws this image with its upper-left corner at  point *(x,y)*, scaled by amount *scale*, with an optional separate scaling for the y axis.

### playdate.graphics.image:drawTiled

```lua
playdate.graphics.image:drawTiled(rect: _Rect, flip: (integer|string)): nil
playdate.graphics.image:drawTiled(x: integer, y: integer, width: integer, height: integer, flip: (integer|string)): nil
```

Tiles the image into the given rectangle, using either listed dimensions or a `playdate.geometry.rect` object, and the optional flip style.

### playdate.graphics.image:drawWithTransform

```lua
playdate.graphics.image:drawWithTransform(xform: _AffineTransform, x: integer, y: integer): nil
```

Draws this image centered at point *(x,y)* with the transform *xform* applied.

### playdate.graphics.image:fadedImage

```lua
playdate.graphics.image:fadedImage(alpha: number, ditherType: integer): _Image
```

Returns a faded version of the caller.
* *alpha*: The alpha value assigned to the caller, in the range 0.0 - 1.0. If an image mask already exists it is multiplied by *alpha*.
* *ditherType*: The caller is faded into a greyscale image and dithered with one of the dithering algorithms listed in playdate.graphics.image:blurredImage()

### playdate.graphics.image:getMaskImage

```lua
playdate.graphics.image:getMaskImage(): _Image
```

If the image has a mask, returns the mask as a separate image. Otherwise, returns `nil`.
The returned image references the original’s data, so drawing into this image alters the original image’s mask.

### playdate.graphics.image:getSize

```lua
playdate.graphics.image:getSize(): (integer, integer)
```

Returns the pair (*width*, *height*)

### playdate.graphics.image:hasMask

```lua
playdate.graphics.image:hasMask(): boolean
```

Returns *true* if the image has a mask.

### playdate.graphics.image:invertedImage

```lua
playdate.graphics.image:invertedImage(): _Image
```

Returns a color-inverted copy of the caller.

### playdate.graphics.image:load

```lua
playdate.graphics.image:load(path: string): (boolean, string?)
```

Loads a new image from the data at *path* into an already-existing image, without allocating additional memory. The image at *path* must be of the same dimensions as the original.
Returns *(success, [error])*. If the boolean *success* is false, *error* is also returned.

### playdate.graphics.image:removeMask

```lua
playdate.graphics.image:removeMask(): nil
```

Removes the mask from the image if it has one.

### playdate.graphics.image:rotatedImage

```lua
playdate.graphics.image:rotatedImage(angle: number, scale: number, yscale: number): _Image
```

Returns a new image containing this image rotated by (clockwise) *angle* degrees, scaled by optional argument *scale*, with an optional separate scaling for the y axis.
Unless rotating by a multiple of 180 degrees, the new image will have different dimensions than the original.

### playdate.graphics.image:sample

```lua
playdate.graphics.image:sample(x: integer, y: integer): integer
```

Returns *playdate.graphics.kColorWhite* if the image is white at (*x*, *y*), *playdate.graphics.kColorBlack* if it’s black, or *playdate.graphics.kColorClear* if it’s transparent.
The upper-left pixel of the image is at coordinate *(0, 0)*.

### playdate.graphics.image:scaledImage

```lua
playdate.graphics.image:scaledImage(scale: number, yscale: number): _Image
```

Returns a new image containing this image scaled by amount *scale*, with an optional separate scaling for the y axis.

### playdate.graphics.image:setInverted

```lua
playdate.graphics.image:setInverted(flag: boolean): nil
```

If *flag* is true, the image will be drawn with its colors inverted. If the image is being used as a stencil, its behavior is reversed: pixels are drawn where the stencil is black, nothing is drawn where the stencil is white.

### playdate.graphics.image:setMaskImage

```lua
playdate.graphics.image:setMaskImage(maskImage: _Image): nil
```

Sets the image’s mask to a copy of *maskImage*.

### playdate.graphics.image:transformedImage

```lua
playdate.graphics.image:transformedImage(xform: _AffineTransform): _Image
```

Returns a new image containing the image with the transform *xform* applied.

### playdate.graphics.image:vcrPauseFilterImage

```lua
playdate.graphics.image:vcrPauseFilterImage(): _Image
```

Returns an image created by applying a VCR pause effect to the calling image.
To add a VCR effect to a single image, call this function once on the source image; the function will return a distorted version of the source image. To add a VCR effect to a series of frames / video, call this function on every frame and display each returned image. (This function uses an internal random number to determine the appearance of the effect on each frame, so the effect will vary from frame to frame in a way that makes it appear like "live" paused video.)

### playdate.graphics.imageSizeAtPath

```lua
playdate.graphics.imageSizeAtPath(path: string): (integer, integer)
```

Returns the pair (*width*, *height*) for the image at *path* without actually loading the image.

## Classes

### _Image

```lua
---@class _Image : playdate.graphics.image
---@field width integer
---@field height integer
```

### playdate.graphics.image

```lua
---@class playdate.graphics.image
---@field kDitherTypeNone integer 0
---@field kDitherTypeDiagonalLine integer 1
---@field kDitherTypeHorizontalLine integer 3
---@field kDitherTypeVerticalLine integer 2
---@field kDitherTypeScreen integer 4
---@field kDitherTypeBayer2x2 integer 5
---@field kDitherTypeBayer4x4 integer 6
---@field kDitherTypeBayer8x8 integer 7
---@field kDitherTypeFloydSteinberg integer 8
---@field kDitherTypeBurkes integer 9
---@field kDitherTypeAtkinson integer 10
```
