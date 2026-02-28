# playdate.graphics.tilemap

## Functions

### playdate.graphics.tilemap.new

```lua
playdate.graphics.tilemap.new(): _TileMap
```

Creates a new tilemap object.

### playdate.graphics.tilemap:draw

```lua
playdate.graphics.tilemap:draw(x: integer, y: integer, sourceRect: _Rect): nil
```

Draws the tilemap at screen coordinate (*x*, *y*).
*sourceRect*, if specified, will cause only the part of the tilemap within sourceRect to be drawn. *sourceRect* should be relative to the tilemap’s bounds and can be a playdate.geometry.rect or four integers, (*x*, *y*, *w*, *h*), representing the rect.

### playdate.graphics.tilemap:drawIgnoringOffset

```lua
playdate.graphics.tilemap:drawIgnoringOffset(x: integer, y: integer, sourceRect: _Rect): nil
```

Draws the tilemap ignoring the currently set `drawOffset`.

### playdate.graphics.tilemap:getCollisionRects

```lua
playdate.graphics.tilemap:getCollisionRects(emptyIDs: integer[]): _Rect[]
```

This function returns an array of playdate.geometry.rect objects that describe the areas of the tilemap that should trigger collisions.  You can also think of them as the "impassable" rects of your tilemap.  These rects will be in tilemap coordinates, not pixel coordinates.
*emptyIDs* is an array that contains the tile IDs of "empty" (or "passable") tiles in the tilemap — in other words, tile IDs that should not trigger a collision. Tiles with default IDs of 0 are treated as empty by default, so you do not need to include 0 in the array.
For example, if you have a tilemap describing terrain, where tile ID 1 represents grass the player can walk over, and tile ID 2 represents mountains that the player can’t cross, you’d pass an array containing just the value 1.  You’ll get a back an array of a minimal number of rects describing the areas where there are mountain tiles.
You can then pass each of those rects into playdate.graphics.sprite.addEmptyCollisionSprite() to add an empty (invisible) sprite into the scene for the built-in collision detection methods.  In this example, collide rects would be added around mountain tiles but not grass tiles.
Alternatively, instead of calling getCollisionRects() at all, you can use the convenience function playdate.graphics.sprite.addWallSprites(), which is effectively a shortcut for calling getCollisionRects() and passing all the resulting rects to addEmptyCollisionSprite().

### playdate.graphics.tilemap:getPixelSize

```lua
playdate.graphics.tilemap:getPixelSize(): (integer, integer)
```

Returns the size of the tilemap in pixels; that is, the size of the image multiplied by the number of rows and columns in the map. Returns multiple values (*width*, *height*).
The tilemap size in pixels is determined by the tile size of the imagetable it is referencing, and the width of the tilemap set via :setTiles() or :setSize(). It is not otherwise configurable.

### playdate.graphics.tilemap:getSize

```lua
playdate.graphics.tilemap:getSize(): (integer, integer)
```

Returns the size of the tilemap, in tiles, as a pair, (*width*, *height*).

### playdate.graphics.tilemap:getTileAtPosition

```lua
playdate.graphics.tilemap:getTileAtPosition(x: integer, y: integer): number?
```

Returns the image index of the tile at the given *x* and *y* coordinate. If *x* or *y* is out of bounds, returns nil.
Tilemaps and imagetables, like Lua arrays, are 1-based, not 0-based. `tilemap:getTileAtPosition(1, 1)` will return the index of the top-leftmost tile.

### playdate.graphics.tilemap:getTileSize

```lua
playdate.graphics.tilemap:getTileSize(): (integer, integer)
```

Returns two values (*width*, *height*), the pixel width and height of an individual tile.
These values are determined by the tile size of the associated imagetable and are not otherwise configurable.

### playdate.graphics.tilemap:getTiles

```lua
playdate.graphics.tilemap:getTiles(): (integer[], integer)
```

Returns *data*, *width*
---
--- *data* is a flat, one-dimensional array-like table containing index values to the tilemap’s imagetable.
---
---  *width* is the width of the tilemap, in number of tiles.

### playdate.graphics.tilemap:setImageTable

```lua
playdate.graphics.tilemap:setImageTable(table: table): nil
```

Sets the tilemap’s playdate.graphics.imagetable to *table*, a playdate.graphics.imagetable.

### playdate.graphics.tilemap:setSize

```lua
playdate.graphics.tilemap:setSize(width: integer, height: integer): nil
```

Sets the tilemap’s width and height, in number of tiles.
The tilemap’s width can also be configured in a call to :setTiles().

### playdate.graphics.tilemap:setTileAtPosition

```lua
playdate.graphics.tilemap:setTileAtPosition(x: integer, y: integer, index: integer): nil
```

Sets the index of the tile at tilemap position (*x*, *y*). *index* is the (1-based) index of the image in the tilemap’s playdate.graphics.imagetable.
This function is especially useful for making small adjustments to existing tilemaps&nbsp;—&nbsp;say, if the state of a tile changes during gameplay.
Tilemaps and imagetables, like Lua arrays, are 1-based, not 0-based. `tilemap:setTileAtPosition(1, 1, 2)` will set the index of the tile in the top-leftmost position to 2.

### playdate.graphics.tilemap:setTiles

```lua
playdate.graphics.tilemap:setTiles(data: integer[], width: integer): nil
```

Sets the tilemap’s width to *width*, then populates the tilemap with *data*, which should be a flat, one-dimensional array-like table containing index values to the tilemap’s imagetable.
This function is especially useful for configuring a large number of tiles at once — say when first loading a game level.
