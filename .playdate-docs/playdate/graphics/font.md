# playdate.graphics.font

## Functions

### playdate.graphics.drawLocalizedText

```lua
playdate.graphics.drawLocalizedText(key: string, rect: _Rect, language: (integer|string), leadingAdjustment: integer): nil
playdate.graphics.drawLocalizedText(key: string, x: integer, y: integer, width: integer, height: integer, language: (integer|string), leadingAdjustment: integer, wrapMode: integer, alignment: integer): nil
playdate.graphics.drawLocalizedText(key: string, x: integer, y: integer, language: (integer|string), leadingAdjustment: integer): nil
```

Draws the text found by doing a lookup of *key* in the .strings file corresponding to the current system language, or *language*, if specified.
The optional *language* argument can be one of the strings "en", "jp", or one of the constants:
* *playdate.graphics.font.kLanguageEnglish*
* *playdate.graphics.font.kLanguageJapanese*
Other arguments work the same as in `drawText()`.
For more information about localization and strings files, see the Localization section.

### playdate.graphics.drawLocalizedTextAligned

```lua
playdate.graphics.drawLocalizedTextAligned(text: string, x: integer, y: integer, alignment: integer, language: (integer|string), leadingAdjustment: integer): nil
```

You must import *CoreLibs/graphics* to use this function.
Same as drawTextAligned() except localized text is drawn.

### playdate.graphics.drawLocalizedTextInRect

```lua
playdate.graphics.drawLocalizedTextInRect(text: string, rect: _Rect, leadingAdjustment: integer, truncationString: string, alignment: integer, font: _Font, language: (integer|string)): nil
playdate.graphics.drawLocalizedTextInRect(text: string, x: integer, y: integer, width: integer, height: integer, leadingAdjustment: integer, truncationString: string, alignment: integer, font: playdate.graphics.font, language: (integer|string)): nil
```

You must import *CoreLibs/graphics* to use these functions.
Same as drawTextInRect() except localized text is drawn.

### playdate.graphics.drawText

```lua
playdate.graphics.drawText(text: string, rect: _Rect, fontFamily: table<integer, _Font>, leadingAdjustment: integer, wrapMode: integer, alignment: integer): nil
playdate.graphics.drawText(text: string, x: integer, y: integer, width: integer, height: integer, fontFamily: table<integer, _Font>, leadingAdjustment: integer, wrapMode: integer, alignment: integer): nil
playdate.graphics.drawText(text: string, x: integer, y: integer, fontFamily: table<integer, _Font>, leadingAdjustment: integer): nil
```

Draws the text using the current font and font advance at location (*x*, *y*). If *width* and *height* are specified, drawing is constrained to the rectangle `(x,y,width,height)`, using the given *wrapMode* and *alignment*, if provided. Alternatively, a `playdate.geometry.rect` object can be passed instead of `x,y,width,height`. Valid values for *wrapMode* are
* *playdate.graphics.kWrapClip*
* *playdate.graphics.kWrapCharacter*
* *playdate.graphics.kWrapWord*
and values for *alignment* are
* *playdate.graphics.kAlignLeft*
* *playdate.graphics.kAlignCenter*
* *playdate.graphics.kAlignRight*
The default wrap mode is `playdate.graphics.kWrapWord` and the default alignment is `playdate.graphics.kAlignLeft`.
If *fontFamily* is provided, the text is draw using the given fonts instead of the currently set font. *fontFamily* should be a table of fonts using keys as specified in setFontFamily(fontFamily).
The optional *leadingAdjustment* may be used to modify the spacing between lines of text. Pass nil to use the default leading for the font.
Returns two numbers indicating the width and height of the drawn text.
**Styling text**
To draw bold text, surround the bold portion of text with asterisks. To draw italic text, surround the italic portion of text with underscores. For example:
```
playdate.graphics.drawText("normal *bold* _italic_", x, y)
```
which will output: "normal **bold** *italic*". Bold and italic font variations must be set using setFont() with the appropriate variant argument, otherwise the default Playdate fonts will be used.
**Escaping styling characters**
To draw an asterisk or underscore, use a double-asterisk or double-underscore. Styles may not be nested, but double-characters can be used inside of a styled portion of text.
For a complete set of characters allowed in *text*, see playdate.graphics.font. In addition, the newline character `\n` is allowed and works as expected.
**Avoiding styling**
Use playdate.graphics.font:drawText(), which doesn’t support formatted text.
**Inverting text color**
To draw white-on-black text (assuming the font you are using is defined in the standard black-on-transparent manner), first call playdate.graphics.setImageDrawMode(playdate.graphics.kDrawModeFillWhite), followed by the appropriate drawText() call. setImageDrawMode() affects how text is rendered because characters are technically images.
Equivalent to `playdate->graphics->drawText()` in the C API.

### playdate.graphics.drawTextAligned

```lua
playdate.graphics.drawTextAligned(text: string, x: integer, y: integer, alignment: integer, leadingAdjustment: integer): nil
```

You must import *CoreLibs/graphics* to use this function.
Draws the string *text* aligned to the left, right, or centered on the *x* coordinate. Pass one of *kTextAlignment.left*, *kTextAlignment.center*, *kTextAlignment.right* for the *alignment* parameter.
For text formatting options, see drawText()
To draw unstyled text using a single font, see playdate.graphics.font:drawTextAligned()

### playdate.graphics.drawTextInRect

```lua
playdate.graphics.drawTextInRect(text: string, rect: _Rect, leadingAdjustment: integer, truncationString: string, alignment: integer, font: _Font): nil
playdate.graphics.drawTextInRect(text: string, x: integer, y: integer, width: integer, height: integer, leadingAdjustment: integer, truncationString: string, alignment: integer, font: playdate.graphics.font): nil
```

You must import *CoreLibs/graphics* to use these functions.
Draws the text using the current font and font advance into the rect defined by (`*x*`, `*y*`, `*width*`, `*height*`) (or `*rect*`).
If `*truncationString*` is provided and the text cannot fit in the rect, `*truncationString*` will be appended to the last line.
`*alignment*`, if provided, should be one of one of `*kTextAlignment.left*`, `*kTextAlignment.center*`, `*kTextAlignment.right*`. Pass `nil` for `*leadingAdjustment*` and `*truncationString*` if those parameters are not required.
`*font*`, if provided, will cause the text to be drawn unstyled using font:drawText() rather than playdate.graphics.drawText() using the currently-set system fonts.
For text formatting options, see drawText()
Returns `*width*`, `*height*`, `*textWasTruncated*`
`*width*` and `*height*` indicate the size in pixels of the drawn text. These values may be smaller than the width and height specified when calling the function.
`*textWasTruncated*` indicates if the text was truncated to fit within the specified rect.

### playdate.graphics.font.new

```lua
playdate.graphics.font.new(path: string): _Font
```

Returns a playdate.graphics.font object from the data at *path*. If there is no file at *path*, the function returns nil.

### playdate.graphics.font.newFamily

```lua
playdate.graphics.font.newFamily(fontPaths: table<integer, string>): _Font[]
```

Returns a font family table from the font files specified in *fontPaths*. *fontPaths* should be a table with the following format:
```
local fontPaths = {
 [playdate.graphics.font.kVariantNormal] = "path/to/normalFont",
    [playdate.graphics.font.kVariantBold] = "path/to/boldFont",
    [playdate.graphics.font.kVariantItalic] = "path/to/italicFont"
}
```
The table returned is of the same format with font objects in place of the paths, and is appropriate to pass to the functions setFontFamily() and getTextSize().

### playdate.graphics.font:drawText

```lua
playdate.graphics.font:drawText(text: string, rect: _Rect, leadingAdjustment: integer, wrapMode: integer, alignment: integer): nil
playdate.graphics.font:drawText(text: string, x: integer, y: integer, width: integer, height: integer, leadingAdjustment: integer, wrapMode: integer, alignment: integer): nil
playdate.graphics.font:drawText(text: string, x: integer, y: integer, leadingAdjustment: integer): nil
```

Draws a string at the specified *x, y* coordinate using this particular font instance. (Compare to playdate.graphics.drawText(text, x, y), which draws the string with whatever the "current font" is, as defined by playdate.graphics.setFont(font)).
If *width* and *height* are specified, drawing is constrained to the rectangle `(x,y,width,height)`, using the given `wrapMode` and `alignment` if provided. Alternatively, a `playdate.geometry.rect` object can be passed instead of `x,y,width,height`. Valid values for *wrapMode* are
* *playdate.graphics.kWrapClip*
* *playdate.graphics.kWrapCharacter*
* *playdate.graphics.kWrapWord*
and values for *alignment* are
* *playdate.graphics.kAlignLeft*
* *playdate.graphics.kAlignCenter*
* *playdate.graphics.kAlignRight*
The default wrap mode is `playdate.graphics.kWrapWord` and the default alignment is `playdate.graphics.kAlignLeft`.
The optional *leadingAdjustment* may be used to modify the spacing between lines of text.
The function returns two numbers indicating the width and height of the drawn text.
`font:drawText()` does not support inline styles like bold and italics. Instead use playdate.graphics.drawText().

### playdate.graphics.font:drawTextAligned

```lua
playdate.graphics.font:drawTextAligned(text: string, x: integer, y: integer, alignment: integer, leadingAdjustment: integer): nil
```

You must import *CoreLibs/graphics* to use this function.
Draws the string *text* aligned to the left, right, or centered on the *x* coordinate. Pass one of *kTextAlignment.left*, *kTextAlignment.center*, *kTextAlignment.right* for the *alignment* parameter. (Compare to playdate.graphics.drawTextAligned(text, x, y, alignment), which draws the string with the "current font", as defined by playdate.graphics.setFont(font)).

### playdate.graphics.font:getGlyph

```lua
playdate.graphics.font:getGlyph(character: string): _Image
```

Returns the `playdate.graphics.image` containing the requested glyph. *character* can either be a string or a unicode codepoint number.

### playdate.graphics.font:getHeight

```lua
playdate.graphics.font:getHeight(): integer
```

Returns the pixel height of this font.

### playdate.graphics.font:getLeading

```lua
playdate.graphics.font:getLeading(): integer
```

Returns the leading (spacing between lines) of this font, in pixels.

### playdate.graphics.font:getTextWidth

```lua
playdate.graphics.font:getTextWidth(text: string): integer
```

Returns the pixel width of the text when rendered with this font.

### playdate.graphics.font:getTracking

```lua
playdate.graphics.font:getTracking(): integer
```

Returns the tracking of this font (spacing between letters), in pixels.
Equivalent to `playdate->graphics->getTextTracking()` in the C API.

### playdate.graphics.font:setLeading

```lua
playdate.graphics.font:setLeading(pixels: integer): nil
```

Sets the leading (spacing between lines) of this font, in pixels.
Equivalent to `playdate->graphics->setTextLeading()` in the C API.

### playdate.graphics.font:setTracking

```lua
playdate.graphics.font:setTracking(pixels: integer): nil
```

Sets the tracking of this font (spacing between letters), in pixels.
Equivalent to `playdate->graphics->setTextTracking()` in the C API.

### playdate.graphics.getFont

```lua
playdate.graphics.getFont(variant: (integer|string)): _Font
```

Returns the current font, a playdate.graphics.font.

### playdate.graphics.getFontTracking

```lua
playdate.graphics.getFontTracking(): integer
```

Gets the global font tracking (spacing between letters) in pixels.

### playdate.graphics.getLocalizedText

```lua
playdate.graphics.getLocalizedText(key: string, language: (integer|string)): string
```

Returns a string found by doing a lookup of *key* in the .strings file corresponding to the current system language, or *language*, if specified.
The optional *language* argument can be one of the strings "en", "jp", or one of the constants:
* *playdate.graphics.font.kLanguageEnglish*
* *playdate.graphics.font.kLanguageJapanese*
For more information about localization and strings files, see the Localization section.

### playdate.graphics.getSystemFont

```lua
playdate.graphics.getSystemFont(variant: (integer|string)): _Font
```

Like getFont() but returns the system font rather than the currently set font.
*variant* should be one of the strings "normal", "bold", or "italic", or one of the constants:
* *playdate.graphics.font.kVariantNormal*
* *playdate.graphics.font.kVariantBold*
* *playdate.graphics.font.kVariantItalic*

### playdate.graphics.getTextSize

```lua
playdate.graphics.getTextSize(str: string, fontFamily: table<integer, _Font>, leadingAdjustment: integer): (integer, integer)
```

Returns multiple values *(width, height)* giving the dimensions required to draw the text *str* using drawText(). Newline characters (`\n`) are respected.
*fontFamily* should be a table of fonts using keys as specified in setFontFamily(fontFamily). If provided, fonts from *fontFamily* will be used for calculating the size of *str* instead of the currently set font.

### playdate.graphics.getTextSizeForMaxWidth

```lua
playdate.graphics.getTextSizeForMaxWidth(text: string, maxWidth: integer, leadingAdjustment: integer, font: _Font): (integer, integer)
```

You must import *CoreLibs/graphics* to use this function.
Returns `*width*`, `*height*` which indicate the minimum size required for `*text*` to be drawn using drawTextInRect(). The `*width*` returned will be less than or equal to `*maxWidth*`.
`*font*`, if provided, will cause the text size to be calculated without bold or italic styling using the specified font.

### playdate.graphics.imageWithText

```lua
playdate.graphics.imageWithText(text: string, maxWidth: integer, maxHeight: integer, backgroundColor: integer, leadingAdjustment: integer, truncationString: string, alignment: integer, font: playdate.graphics.font): (_Image, boolean)
```

You must import *CoreLibs/graphics* to use this function.
Generates an image containing `*text*`. This is useful if you need to redraw the same text frequently.
`*maxWidth*` and `*maxHeight*` specify the maximum size of the returned image.
`*backgroundColor*`, if specified, will cause the image’s background to be one of *playdate.graphics.kColorWhite*, *playdate.graphics.kColorBlack*, or *playdate.graphics.kColorClear*.
`*font*`, if provided, will cause the text to be drawn without bold or italic styling using the specified font.
The remaining arguments are the same as those in drawTextInRect().
Returns `*image*`, `*textWasTruncated*`
`*image*` is a newly-created image containing the specified text, or nil if an image could not be created. The image’s dimensions may be smaller than `*maxWidth*`, `*maxHeight*`.
`*textWasTruncated*` indicates if the text was truncated to fit within the specified width and height.

### playdate.graphics.setFont

```lua
playdate.graphics.setFont(font: _Font, variant: (integer|string)): nil
```

Sets the current font, a playdate.graphics.font.
*variant* should be one of the strings "normal", "bold", or "italic", or one of the constants:
* *playdate.graphics.font.kVariantNormal*
* *playdate.graphics.font.kVariantBold*
* *playdate.graphics.font.kVariantItalic*
If no variant is specified, *kFontVariantNormal* is used.
Equivalent to `playdate->graphics->setFont()` in the C API.

### playdate.graphics.setFontFamily

```lua
playdate.graphics.setFontFamily(fontFamily: table<integer, _Font>): nil
```

Sets multiple font variants at once. `fontFamily` should be a table using the following format:
```
local fontFamily = {
 [playdate.graphics.font.kVariantNormal] = normal_font,
    [playdate.graphics.font.kVariantBold] = bold_font,
    [playdate.graphics.font.kVariantItalic] = italic_font
}
```
All fonts and font variants need not be present in the table.

### playdate.graphics.setFontTracking

```lua
playdate.graphics.setFontTracking(pixels: integer): nil
```

Sets the global font tracking (spacing between letters) in pixels. This value is added to the font’s own tracking value as specified in its .fnt file.
See playdate.graphics.font:setTracking to adjust tracking on a specific font.

## Classes

### kTextAlignment

```lua
---@class kTextAlignment
---@field left integer 0
---@field right integer 1
---@field center integer 2
```

### playdate.graphics.font

```lua
---@class playdate.graphics.font
---@field kLanguageEnglish integer 0
---@field kLanguageJapanese integer 1
---@field kVariantNormal integer 0
---@field kVariantBold integer 1
---@field kVariantItalic integer 2
```
