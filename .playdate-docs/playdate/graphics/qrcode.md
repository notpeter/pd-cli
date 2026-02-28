# playdate.graphics.qrcode

## Functions

### playdate.graphics.generateQRCode

```lua
playdate.graphics.generateQRCode(stringToEncode: string, desiredEdgeDimension: integer, callback: fun(image?: _Image, err?: string)): _Timer
```

You must import *CoreLibs/qrcode* to use this function.
This function uses `playdate.timer` internally, so be sure to call `playdate.timer.updateTimers()` in your main `playdate.update()` function, otherwise the callback will never be invoked.
Asynchronously returns an image representing a QR code for the passed-in string to the function `callback`. The arguments passed to the callback are *image*, *errorMessage*. (If an *errorMessage* string is returned, *image* will be nil.)
`desiredEdgeDimension` lets you specify an approximate edge dimension in pixels for the desired QR code,&nbsp;though the function has limited flexibility in sizing QR codes, based on the amount of information to be encoded, and the restrictions of a 1-bit screen. The function will attempt to generate a QR code *smaller* than `desiredEdgeDimension` if possible. (Note that QR codes always have the same width and height.)
If you specify nil for `desiredEdgeDimension`, the returned image will balance small size with easy readability. If you specify 0, the returned image will be the smallest possible QR code for the specified string.
`generateQRCode()` will return a reference to the timer it uses to run asynchronously. If you wish to stop execution of the background process generating the QR code, call `:remove()` on that returned timer.
If you know ahead of time what data you plan to encode, it is much faster to pre-generate the QR code, store it as a .png file in your game, and draw the .png at runtime. You can use `playdate.simulator.writeToFile()` to create this .png file.

### playdate.graphics.generateQRCodeSync

```lua
playdate.graphics.generateQRCodeSync(stringToEncode: string, desiredEdgeDimension: integer): (_Image, string)
```
