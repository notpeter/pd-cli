# playdate.graphics.sprite

## Functions

### playdate.graphics.sprite.addDirtyRect

```lua
playdate.graphics.sprite.addDirtyRect(x: integer, y: integer, width: integer, height: integer): nil
```

Marks the given rectangle (in screen coordinates) as needing a redraw. playdate.graphics drawing functions now call this automatically, adding their drawn areas to the sprite’s dirty list, so there’s likely no need to call this manually any more. This behavior may change in the future, though.

### playdate.graphics.sprite.addEmptyCollisionSprite

```lua
playdate.graphics.sprite.addEmptyCollisionSprite(r: _Rect): nil
playdate.graphics.sprite.addEmptyCollisionSprite(x: integer, y: integer, w: integer, h: integer): nil
```

You must import *CoreLibs/sprites* to use this function.
This convenience function adds an invisible sprite defined by the rectangle *x*, *y*, *w*, *h* (or the playdate.geometry.rect *r*) for the purpose of triggering collisions.  This is useful for making areas impassable, triggering an event when a sprite enters a certain area, and so on.

### playdate.graphics.sprite.addSprite

```lua
playdate.graphics.sprite.addSprite(sprite: _Sprite): nil
```

Adds the given sprite to the display list, so that it is drawn in the current scene. Note that this is called with a period `.` instead of a colon `:`.

### playdate.graphics.sprite.addWallSprites

```lua
playdate.graphics.sprite.addWallSprites(tilemap: _TileMap, emptyIDs: integer[], xOffset: integer, yOffset: integer): _Sprite[]
```

You must import *CoreLibs/sprites* to use this function.
This convenience function automatically adds empty collision sprites necessary to restrict movement within a tilemap.
*tilemap* is a playdate.graphics.tilemap.
*emptyIDs* is an array of tile IDs that should be considered "passable" — in other words, not walls. Tiles with default IDs of 0 are treated as passable by default, so you do not need to include 0 in the array.
*xOffset, yOffset* optionally indicate the distance the new sprites should be offset from (0,0).
Returns an array-style table of the newly created sprites.
Calling this function is effectively a shortcut for calling playdate.graphics.tilemap:getCollisionRects() and passing the resulting rects to addEmptyCollisionSprite().

### playdate.graphics.sprite.allOverlappingSprites

```lua
playdate.graphics.sprite.allOverlappingSprites(): _Sprite[][]
```

Returns an array of array-style tables, each containing two sprites that have overlapping collide rects. All sprite pairs that are have overlapping collide rects (taking the sprites' group and collides-with masks into consideration) are returned.
```
local collisions = gfx.sprite.allOverlappingSprites()
for i = 1, #collisions do
        local collisionPair = collisions[i]
        local sprite1 = collisionPair[1]
        local sprite2 = collisionPair[2]
        -- do something with the colliding sprites
end
```

### playdate.graphics.sprite.clearClipRectsInRange

```lua
playdate.graphics.sprite.clearClipRectsInRange(startz: integer, endz: integer): nil
```

Clears sprite clip rects in the given z-index range.

### playdate.graphics.sprite.getAllSprites

```lua
playdate.graphics.sprite.getAllSprites(): _Sprite[]
```

Returns an array of all sprites in the display list.

### playdate.graphics.sprite.getAlwaysRedraw

```lua
playdate.graphics.sprite.getAlwaysRedraw(): boolean
```

Return’s the sprites "always redraw" flag.

### playdate.graphics.sprite.new

```lua
playdate.graphics.sprite.new(image_or_tilemap: (_Image|_TileMap)): _Sprite
```

This class method (note the "." syntax rather than ":") returns a new sprite object. A previously-loaded image or tilemap object can be optionally passed-in.
To see your sprite onscreen, you will need to call `:add()` on your sprite to add it to the display list.

### playdate.graphics.sprite.performOnAllSprites

```lua
playdate.graphics.sprite.performOnAllSprites(f: fun(sprite: _Sprite)): nil
```

You must import *CoreLibs/sprites* to use this function.
Performs the function *f* on all sprites in the display list. *f* should take one argument, which will be a sprite.

### playdate.graphics.sprite.querySpriteInfoAlongLine

```lua
playdate.graphics.sprite.querySpriteInfoAlongLine(lineSegment: _LineSegment): _SpriteCollisionInfo[]
playdate.graphics.sprite.querySpriteInfoAlongLine(x1: integer, y1: integer, x2: integer, y2: integer): _SpriteCollisionInfo[]
```

Similar to *querySpritesAlongLine()*, but instead of sprites returns an array of *collisionInfo* tables containing information about sprites intersecting the line segment, and *len*, which is the number of collisions found. If you don’t need this information, use *querySpritesAlongLine()* as it will be faster.
Each *collisionInfo* table contains:
* *sprite*: the sprite being intersected by the segment.
* *entryPoint*: a `point` representing the coordinates of the first intersection between `sprite` and the line segment.
* *exitPoint*: a `point` representing  the coordinates of the second intersection between `sprite` and the line segment.
* *ti1* &amp; *ti2*: numbers between 0 and 1 which indicate how far from the starting point of the line segment the collision happened; t1 for the entry point, t2 for the exit point. This can be useful for things like having a laser cause more damage if the impact is close.

### playdate.graphics.sprite.querySpritesAlongLine

```lua
playdate.graphics.sprite.querySpritesAlongLine(lineSegment: _LineSegment): _Sprite[]
playdate.graphics.sprite.querySpritesAlongLine(x1: integer, y1: integer, x2: integer, y2: integer): _Sprite[]
```

Returns all sprites with collision rects intersecting the line segment.

### playdate.graphics.sprite.querySpritesAtPoint

```lua
playdate.graphics.sprite.querySpritesAtPoint(p: _Point): _Sprite[]
playdate.graphics.sprite.querySpritesAtPoint(x: integer, y: integer): _Sprite[]
```

Returns all sprites with collision rects containing the point.

### playdate.graphics.sprite.querySpritesInRect

```lua
playdate.graphics.sprite.querySpritesInRect(rect: _Rect): _Sprite[]
playdate.graphics.sprite.querySpritesInRect(x: integer, y: integer, width: integer, height: integer): _Sprite[]
```

Returns all sprites with collision rects overlapping the rect.

### playdate.graphics.sprite.redrawBackground

```lua
playdate.graphics.sprite.redrawBackground(): nil
```

You must import *CoreLibs/sprites* to use this function.
Marks the background sprite dirty, forcing the drawing callback to be run when playdate.graphics.sprite.update() is called.

### playdate.graphics.sprite.removeAll

```lua
playdate.graphics.sprite.removeAll(): nil
```

Removes all sprites from the global sprite list.

### playdate.graphics.sprite.removeSprite

```lua
playdate.graphics.sprite.removeSprite(sprite: _Sprite): nil
```

Removes the given sprite from the display list. As with `add()`/`addSprite()`, note that this is called with a period `.` instead of a colon `:`.

### playdate.graphics.sprite.removeSprites

```lua
playdate.graphics.sprite.removeSprites(spriteArray: _Sprite[]): nil
```

Removes all sprites in `spriteArray` from the global sprite list.

### playdate.graphics.sprite.setAlwaysRedraw

```lua
playdate.graphics.sprite.setAlwaysRedraw(flag: boolean): nil
```

If set to true, causes all sprites to draw each frame, whether or not they have been marked dirty. This may speed up the performance of your game if the system’s dirty rect tracking is taking up too much time - for example if there are many sprites moving around on screen at once.

### playdate.graphics.sprite.setBackgroundDrawingCallback

```lua
playdate.graphics.sprite.setBackgroundDrawingCallback(drawCallback: fun(x: integer, y: integer, width: integer, height: integer): nil): _Sprite
```

You must import *CoreLibs/sprites* to use this function.
A convenience function for drawing a background image behind your sprites.
*drawCallback* is a routine you specify that implements your background drawing. The callback should be a function taking the arguments `x, y, width, height`, where *x, y, width, height* specify the region (in screen coordinates, not world coordinates) of the background region that needs to be updated.
Some implementation details: `setBackgroundDrawingCallback()` creates a screen-sized sprite with a z-index set to the lowest possible value so it will draw behind other sprites, and adds the sprite to the display list so that it is drawn in the current scene. The background sprite ignores the drawOffset, and will not be automatically redrawn when the draw offset changes; use playdate.graphics.sprite.redrawBackground() if necessary in this case. *drawCallback* will be called from the newly-created background sprite’s playdate.graphics.sprite:draw() callback function and is where you should do your background drawing. This function returns the newly created playdate.graphics.sprite.

### playdate.graphics.sprite.setClipRectsInRange

```lua
playdate.graphics.sprite.setClipRectsInRange(rect: _Rect, startz: integer, endz: integer): nil
playdate.graphics.sprite.setClipRectsInRange(x: integer, y: integer, width: integer, height: integer, startz: integer, endz: integer): nil
```

Sets the clip rect for sprites in the given z-index range.

### playdate.graphics.sprite.spriteCount

```lua
playdate.graphics.sprite.spriteCount(): integer
```

Returns the number of sprites in the display list.

### playdate.graphics.sprite.spriteWithText

```lua
playdate.graphics.sprite.spriteWithText(text: string, maxWidth: integer, maxHeight: integer, backgroundColor: integer, leadingAdjustment: integer, truncationString: string, alignment: integer, font: _Font): _Sprite
```

You must import *CoreLibs/sprites* to use this function.
A conveneince function that creates a sprite with an image of `*text*`, as generated by imageWithText().
The arguments are the same as those in imageWithText().
Returns `*sprite*`, `*textWasTruncated*`
`*sprite*` is a newly-created sprite with its image set to an image of the text specified. The sprite’s dimensions may be smaller than `*maxWidth*`, `*maxHeight*`.
`*textWasTruncated*` indicates if the text was truncated to fit within the specified width and height.

### playdate.graphics.sprite.update

```lua
playdate.graphics.sprite.update(): nil
```

This class method (note the "." syntax rather than ":") calls the update() function on every sprite in the global sprite list and redraws all of the dirty rects.
You will generally want to call `playdate.graphics.sprite.update()` once in your `playdate.update()` method, to ensure that your sprites are updated and drawn during every frame. Failure to do so may mean your sprites will not appear onscreen.
Be careful not confuse `sprite.update()` with `sprite:update()`: the former updates all sprites; the latter updates just the sprite being invoked.

### playdate.graphics.sprite:add

```lua
playdate.graphics.sprite:add(): nil
```

Adds the given sprite to the display list, so that it is drawn in the current scene.

### playdate.graphics.sprite:alphaCollision

```lua
playdate.graphics.sprite:alphaCollision(anotherSprite: _Sprite): boolean
```

Returns a boolean value set to true if a pixel-by-pixel comparison of the sprite images shows that non-transparent pixels are overlapping, based on the current bounds of the sprites.
This method may be used in conjunction with the standard collision architecture. Say, if `overlappingSprites()` or `moveWithCollisions()` report a collision of two sprite’s bounding rects, alphaCollision() could then be used to discern if a pixel-level collision occurred.

### playdate.graphics.sprite:checkCollisions

```lua
playdate.graphics.sprite:checkCollisions(point: _Point): (integer, integer, _SpriteCollisionData, integer)
playdate.graphics.sprite:checkCollisions(x: integer, y: integer): (integer, integer, _SpriteCollisionData, integer)
```

Returns the same values as `moveWithCollisions()` but does not actually move the sprite.

### playdate.graphics.sprite:clearClipRect

```lua
playdate.graphics.sprite:clearClipRect(): nil
```

Clears the sprite’s current clipping rectangle.

### playdate.graphics.sprite:clearCollideRect

```lua
playdate.graphics.sprite:clearCollideRect(): nil
```

Clears the sprite’s collide rect set with `setCollideRect()`.

### playdate.graphics.sprite:clearStencil

```lua
playdate.graphics.sprite:clearStencil(): nil
```

Clears the sprite’s stencil.

### playdate.graphics.sprite:collisionResponse

```lua
playdate.graphics.sprite:collisionResponse(other: _Sprite): integer
```

A callback that can be defined on a sprite to control the type of collision response that should happen when a collision with *other* occurs. This callback should return one of the following four values:
* *playdate.graphics.sprite.kCollisionTypeSlide*: Use for collisions that should slide over other objects, like Super Mario does over a platform or the ground.
* *playdate.graphics.sprite.kCollisionTypeFreeze*: Use for collisions where the sprite should stop moving as soon as it collides with *other*, such as an arrow hitting a wall.
* *playdate.graphics.sprite.kCollisionTypeOverlap*: Use for collisions in which you want to know about the collision but it should not impact the movement of the sprite, such as when collecting a coin.
* *playdate.graphics.sprite.kCollisionTypeBounce*: Use when the sprite should move away from *other*, like the ball in Pong or Arkanoid.
The strings "slide", "freeze", "overlap", and "bounce" can be used instead of the constants.
Feel free to return different values based on the value of *other*. For example, if *other* is a wall sprite, you may want to return "slide" or "bounce", but if it’s a coin you might return "overlap".
If the callback is not present, or returns nil, *kCollisionTypeFreeze* is used.
Instead of defining a callback, the collisionResponse property of a sprite can be set directly to one of the four collision response types. This will be faster, as the lua function will not need to be called, but does not allow for dynamic behavior.
This method should not attempt to modify the sprites in any way. While it might be tempting to deal with collisions here, doing so will have unexpected and undesirable results. Instead, this function should return one of the collision response values as quickly as possible. If sprites need to be modified as the result of a collision, do so elsewhere, such as by inspecting the list of collisions returned by `moveWithCollisions()`.

### playdate.graphics.sprite:collisionsEnabled

```lua
playdate.graphics.sprite:collisionsEnabled(): boolean
```

Returns the sprite’s *collisionsEnabled* flag.

### playdate.graphics.sprite:copy

```lua
playdate.graphics.sprite:copy(): _Sprite
```

Returns a copy of the caller.

### playdate.graphics.sprite:draw

```lua
playdate.graphics.sprite:draw(x: integer, y: integer, width: integer, height: integer): nil
```

If the sprite doesn’t have an image, the sprite’s draw function is called as needed to update the display. The rect passed in is the current dirty rect being updated by the display list. The rect coordinates passed in are relative to the sprite itself (i.e. x = 0, y = 0 refers to the top left corner of the sprite). Note that the callback is only called when the sprite is on screen and has a size specified via sprite:setSize() or sprite:setBounds().
```
-- You can copy and paste this example directly as your main.lua file to see it in action
import "CoreLibs/graphics"
import "CoreLibs/sprites"
local mySprite = playdate.graphics.sprite.new()
mySprite:moveTo(200, 120)
-- You MUST set a size first for anything to show up (either directly or by setting an image)
mySprite:setSize(30, 30)
mySprite:add()
-- The x, y, width, and height arguments refer to the dirty rect being updated, NOT the sprite dimensions
function mySprite:draw(x, y, width, height)
    -- Custom draw methods gives you more flexibility over what's drawn, but with the added benefits of sprites
    -- Here we're just modulating the circle radius over time
    local spriteWidth, spriteHeight = self:getSize()
    if not self.radius or self.radius > spriteWidth then
        self.radius = 0
    end
    self.radius += 1
    -- Drawing coordinates are relative to the sprite (e.g. (0, 0) is the top left of the sprite)
    playdate.graphics.fillCircleAtPoint(spriteWidth / 2, spriteHeight / 2, self.radius)
end
function playdate.update()
    -- Your custom draw method gets called here, but only if the sprite is dirty
    playdate.graphics.sprite.update()
    -- You might need to manually mark it dirty
    mySprite:markDirty()
end
```

### playdate.graphics.sprite:getBounds

```lua
playdate.graphics.sprite:getBounds(): (integer, integer, integer, integer)
```

`getBounds()` returns multiple values (*x*, *y*, *width*, *height*).

### playdate.graphics.sprite:getBoundsRect

```lua
playdate.graphics.sprite:getBoundsRect(): _Rect
```

`getBoundsRect()` returns the sprite bounds as a `playdate.geometry.rect` object.

### playdate.graphics.sprite:getCenter

```lua
playdate.graphics.sprite:getCenter(): (number, number)
```

Returns multiple values (`x, y`) representing the sprite’s drawing center as a fraction (ranging from 0.0 to 1.0) of the height and width.

### playdate.graphics.sprite:getCenterPoint

```lua
playdate.graphics.sprite:getCenterPoint(): _Point
```

Returns a playdate.geometry.point representing the sprite’s drawing center as a fraction (ranging from 0.0 to 1.0) of the height and width.

### playdate.graphics.sprite:getCollideBounds

```lua
playdate.graphics.sprite:getCollideBounds(): (integer, integer, integer, integer)
```

Returns the sprite’s collide rect as multiple values, (*x*, *y*, *width*, *height*).
This function return coordinates relative to the sprite itself; the sprite’s position has no bearing on these values.

### playdate.graphics.sprite:getCollideRect

```lua
playdate.graphics.sprite:getCollideRect(): _Rect
```

Returns the sprite’s collide rect set with `setCollideRect()`. Return value is a `playdate.geometry.rect`.
This function return coordinates relative to the sprite itself; the sprite’s position has no bearing on these values.

### playdate.graphics.sprite:getCollidesWithGroupsMask

```lua
playdate.graphics.sprite:getCollidesWithGroupsMask(): integer
```

Returns the integer value of the sprite’s collision bitmask.

### playdate.graphics.sprite:getGroupMask

```lua
playdate.graphics.sprite:getGroupMask(): integer
```

`getGroupMask()` returns the integer value of the sprite’s group bitmask.

### playdate.graphics.sprite:getImage

```lua
playdate.graphics.sprite:getImage(): _Image
```

Returns the playdate.graphics.image object that was set with setImage().

### playdate.graphics.sprite:getImageFlip

```lua
playdate.graphics.sprite:getImageFlip(): integer
```

Returns one of the values listed at playdate.graphics.image:draw().

### playdate.graphics.sprite:getPosition

```lua
playdate.graphics.sprite:getPosition(): (integer, integer)
```

Returns the sprite’s current x, y position as multiple values (*x*, *y*).

### playdate.graphics.sprite:getRotation

```lua
playdate.graphics.sprite:getRotation(): number
```

Returns the current rotation of the sprite.

### playdate.graphics.sprite:getScale

```lua
playdate.graphics.sprite:getScale(): (integer, integer)
```

Returns multiple values *(xScale, yScale)*, the current scaling of the sprite.

### playdate.graphics.sprite:getSize

```lua
playdate.graphics.sprite:getSize(): (integer, integer)
```

Returns multiple values *(width, height)*, the current size of the sprite.

### playdate.graphics.sprite:getTag

```lua
playdate.graphics.sprite:getTag(): integer
```

Returns the sprite’s tag, an integer value.

### playdate.graphics.sprite:getZIndex

```lua
playdate.graphics.sprite:getZIndex(): integer
```

Returns the Z-index of the given sprite.

### playdate.graphics.sprite:isOpaque

```lua
playdate.graphics.sprite:isOpaque(): boolean
```

Returns the sprite’s current opaque flag.

### playdate.graphics.sprite:isVisible

```lua
playdate.graphics.sprite:isVisible(): boolean
```

Returns a boolean value, true if the sprite is visible.

### playdate.graphics.sprite:markDirty

```lua
playdate.graphics.sprite:markDirty(): nil
```

Marks the rect defined by the sprite’s current bounds as needing a redraw.

### playdate.graphics.sprite:moveBy

```lua
playdate.graphics.sprite:moveBy(x: integer, y: integer): nil
```

Moves the sprite by *x*, *y* pixels relative to its current position.

### playdate.graphics.sprite:moveTo

```lua
playdate.graphics.sprite:moveTo(x: integer, y: integer): nil
```

Moves the sprite and resets the bounds based on the image dimensions and center.

### playdate.graphics.sprite:moveWithCollisions

```lua
playdate.graphics.sprite:moveWithCollisions(goalPoint: _Point): (integer, integer, _SpriteCollisionData, integer)
playdate.graphics.sprite:moveWithCollisions(goalX: integer, goalY: integer): (integer, integer, _SpriteCollisionData, integer)
```

Moves the sprite towards *goalX*, *goalY* or *goalPoint* taking collisions into account, which means the sprite’s final position may not be the same as *goalX*, *goalY* or *goalPoint*.
Returns *actualX*, *actualY*, *collisions*, *length*.
Note that the collision info items are only valid until the next call of *moveWithCollisions* or *checkCollisions*. To save collision information for later, the data should be copied out of the collision info userdata object.
See also `checkCollisions()` to check for collisions without actually moving the sprite.
```
-- You can copy and paste this example directly as your main.lua file to see it in action
import "CoreLibs/graphics"
import "CoreLibs/sprites"
-- Creating a tags object, to keep track of tags more easily
TAGS = {
    player = 1,
    obstacle = 2,
    coin = 3,
    powerUp = 4
}
-- Creating a player sprite we can move around and collide things with
local playerImage = playdate.graphics.image.new(20, 20)
playdate.graphics.pushContext(playerImage)
    playdate.graphics.fillCircleInRect(0, 0, playerImage:getSize())
playdate.graphics.popContext()
local playerSprite = playdate.graphics.sprite.new(playerImage)
-- Setting a tag on the player, so we can check the tag to see if we're colliding against the player
playerSprite:setTag(TAGS.player)
playerSprite:moveTo(200, 120)
-- Remember to set a collision rect, or this all doesn't work!
playerSprite:setCollideRect(0, 0, playerSprite:getSize())
playerSprite:add()
-- Creating an obstacle sprite we can collide against
local obstacleImage = playdate.graphics.image.new(20, 20, playdate.graphics.kColorBlack)
local obstacleSprite = playdate.graphics.sprite.new(obstacleImage)
-- Setting a tag for the obstacle as well
obstacleSprite:setTag(TAGS.obstacle)
obstacleSprite:moveTo(300, 120)
-- Can't forget this!
obstacleSprite:setCollideRect(0, 0, obstacleSprite:getSize())
obstacleSprite:add()
function playdate.update()
    playdate.graphics.sprite.update()
    -- Some simple movement code for the sake of demonstration
    local moveSpeed = 3
    local goalX, goalY = playerSprite.x, playerSprite.y
    if playdate.buttonIsPressed(playdate.kButtonUp) then
        goalY -= moveSpeed
    elseif playdate.buttonIsPressed(playdate.kButtonDown) then
        goalY += moveSpeed
    elseif playdate.buttonIsPressed(playdate.kButtonLeft) then
        goalX -= moveSpeed
    elseif playdate.buttonIsPressed(playdate.kButtonRight) then
        goalX += moveSpeed
    end
    -- Remember to use :moveWithCollisions(), and not :moveTo() or :moveBy(), or collisions won't happen!
    -- To do a "moveBy" operation, sprite:moveBy(5, 5) == sprite:moveWithCollisions(sprite.x + 5, sprite.y + 5)
    local actualX, actualY, collisions, numberOfCollisions = playerSprite:moveWithCollisions(goalX, goalY)
    -- If we get into this loop, there was a collision
    for i=1, numberOfCollisions do
        -- This is getting data about one of things we're currently colliding with. Since we could
        -- be colliding with multiple things at once, we have to handle each collision individually
        local collision = collisions[i]
        -- Always prints 'true', as the sprite property is the sprite being moved (in this case, the player)
        print(collision.sprite == playerSprite)
        -- Also prints 'true', as we set the tag on the player sprite to the player tag
        print(collision.sprite:getTag() == TAGS.player)
        -- This gets the actual sprite object we're colliding with
        local collidedSprite = collision.other
        local collisionTag = collidedSprite:getTag()
        -- Since we set a tag on the obstacle, we can check if we're colliding with that
        if collisionTag == TAGS.obstacle then
            print("Collided with an obstacle!")
            -- We can use the collision normal to check which side we collided with
            local collisionNormal = collision.normal
            if collisionNormal.x == -1 then
                print("Touched left side!")
            elseif collisionNormal.x == 1 then
                print("Touched right side!")
            end
            if collisionNormal.y == -1 then
                print("Touched top!")
            elseif collisionNormal.y == 1 then
                print("Touched bottom!")
            end
        -- Handle some other collisions, like collecting a coin or a power up
        elseif collisionTag == TAGS.coin then
            print("Coin collected!")
        elseif collisionTag == TAGS.powerUp then
            print("Powered up!")
        end
    end
end
```

### playdate.graphics.sprite:overlappingSprites

```lua
playdate.graphics.sprite:overlappingSprites(): _Sprite[]
```

Returns an array of sprites that have collide rects that are currently overlapping the calling sprite’s collide rect, taking the sprites' groups and collides-with masks into consideration.

### playdate.graphics.sprite:remove

```lua
playdate.graphics.sprite:remove(): nil
```

Removes the given sprite from the display list.

### playdate.graphics.sprite:removeAnimator

```lua
playdate.graphics.sprite:removeAnimator(): nil
```

Removes a playdate.graphics.animator assigned to the sprite

### playdate.graphics.sprite:resetCollidesWithGroupsMask

```lua
playdate.graphics.sprite:resetCollidesWithGroupsMask(): nil
```

Resets the sprite’s collides-with-groups mask to `0x00000000`.

### playdate.graphics.sprite:resetGroupMask

```lua
playdate.graphics.sprite:resetGroupMask(): nil
```

Resets the sprite’s group mask to `0x00000000`.

### playdate.graphics.sprite:setAnimator

```lua
playdate.graphics.sprite:setAnimator(animator: _Animator, moveWithCollisions: boolean, removeOnCollision: boolean): nil
```

You must import *CoreLibs/sprites* to use the `setAnimator` method.
`setAnimator` assigns an playdate.graphics.animator to the sprite, which will cause the sprite to automatically update its position each frame while the animator is active.
*animator* should be a playdate.graphics.animator created using playdate.geometry.points for its start and end values.
*movesWithCollisions*, if provided and true will cause the sprite to move with collisions. A collision rect must be set on the sprite prior to passing true for this argument.
*removeOnCollision*, if provided and true will cause the animator to be removed from the sprite when a collision occurs.
`setAnimator` should be called only after any custom update method has been set on the sprite.

### playdate.graphics.sprite:setBounds

```lua
playdate.graphics.sprite:setBounds(rect: _Rect): nil
playdate.graphics.sprite:setBounds(x: integer, y: integer, width: integer, height: integer): nil
```

`setBounds(rect)` sets the bounds of the sprite with a `playdate.geometry.rect` object.

### playdate.graphics.sprite:setCenter

```lua
playdate.graphics.sprite:setCenter(x: number, y: number): nil
```

Sets the sprite’s drawing center as a fraction (ranging from 0.0 to 1.0) of the height and width. Default is 0.5, 0.5 (the center of the sprite). This means that when you call :moveTo(x, y), the center of your sprite will be positioned at *x*, *y*. If you want x and y to represent the upper left corner of your sprite, specify the center as 0, 0.

### playdate.graphics.sprite:setClipRect

```lua
playdate.graphics.sprite:setClipRect(rect: _Rect): nil
playdate.graphics.sprite:setClipRect(x: integer, y: integer, width: integer, height: integer): nil
```

Sets the clipping rectangle for the sprite, using separate parameters or a `playdate.geometry.rect` object. Only areas within the rect will be drawn.

### playdate.graphics.sprite:setCollideRect

```lua
playdate.graphics.sprite:setCollideRect(rect: _Rect): nil
playdate.graphics.sprite:setCollideRect(x: integer, y: integer, width: integer, height: integer): nil
```

`setCollideRect()` marks the area of the sprite, relative to its own internal coordinate system, to be checked for collisions with other sprites' collide rects. Note that the coordinate space is relative to the top-left corner of the bounds, regardless of where the sprite’s center/anchor is located.
If you want to set the sprite’s collide rect to be the same size as the sprite itself, you can write `sprite:setCollideRect( 0, 0, sprite:getSize() )`.
`setCollideRect()` must be invoked on a sprite in order to get it to participate in collisions.
Very large sprites with very large collide rects should be avoided, as they will have a negative impact on performance and memory usage.

### playdate.graphics.sprite:setCollidesWithGroups

```lua
playdate.graphics.sprite:setCollidesWithGroups(groups: (integer|integer[])): nil
```

Pass in a group number or an array of group numbers to specify which groups this sprite can collide with. Groups are numbered 1 through 32. Use `setGroups()` to specify which groups a sprite belongs to.
Alternatively, you can specify group collision behavior with a bitmask by using `setCollidesWithGroupsMask()`.

### playdate.graphics.sprite:setCollidesWithGroupsMask

```lua
playdate.graphics.sprite:setCollidesWithGroupsMask(mask: integer): nil
```

Sets the sprite’s collides-with-groups bitmask, which is 32 bits. The mask specifies which other sprite groups this sprite can collide with. Sprites only collide if the moving sprite’s *collidesWithGroupsMask* matches at least one group of a potential collision sprite (i.e. a bitwise AND (&amp;) between the moving sprite’s *collidesWithGroupsMask* and a potential collision sprite’s *groupMask* != zero) or if the moving sprite’s *collidesWithGroupsMask* and the other sprite’s *groupMask* are both set to 0x00000000 (the default values).
For large mask numbers, pass the number as a hex value, eg. `0xFFFFFFFF` to work around limitations in Lua’s integer sizes.

### playdate.graphics.sprite:setCollisionsEnabled

```lua
playdate.graphics.sprite:setCollisionsEnabled(flag: boolean): nil
```

The sprite’s *collisionsEnabled* flag (defaults to true) can be set to `false` in order to temporarily keep a sprite from colliding with any other sprite.

### playdate.graphics.sprite:setGroupMask

```lua
playdate.graphics.sprite:setGroupMask(mask: integer): nil
```

`setGroupMask()` sets the sprite’s group bitmask, which is 32 bits. In conjunction with the `setCollidesWithGroupsMask()` method, this controls which sprites can collide with each other.
For large group mask numbers, pass the number as a hex value, eg. `0xFFFFFFFF` to work around limitations in Lua’s integer sizes.

### playdate.graphics.sprite:setGroups

```lua
playdate.graphics.sprite:setGroups(groups: (integer|integer[])): nil
```

Adds the sprite to one or more collision groups. A group is a collection of sprites that exhibit similar collision behavior. (An example: in Atari’s *Asteroids*, asteroid sprites would all be added to the same group, while the player’s spaceship might be in a different group.) Use `setCollidesWithGroups()` to define which groups a sprite should collide with.
There are 32 groups, each defined by the integer 1 through 32. To add a sprite to only groups 1 and 3, for example, call `mySprite:setGroups({1, 3})`.
Alternatively, use `setGroupMask()` to set group membership via a bitmask.

### playdate.graphics.sprite:setIgnoresDrawOffset

```lua
playdate.graphics.sprite:setIgnoresDrawOffset(flag: boolean): nil
```

When set to *true*, the sprite will draw in screen coordinates, ignoring the currently-set *drawOffset*.
This only affects drawing, and should not be used on sprites being used for collisions, which will still happen in world-space.

### playdate.graphics.sprite:setImage

```lua
playdate.graphics.sprite:setImage(image: _Image, flip: (integer|string), scale: integer, yscale: integer): nil
```

Sets the sprite’s image to `image`, which should be an instance of playdate.graphics.image. The .flip_ argument is optional; see playdate.graphics.image:draw() for valid values. Optional scale arguments are also accepted. Unless disabled with playdate.graphics.sprite:setRedrawOnImageChange(), the sprite is automatically marked for redraw if the image isn’t the previous image.

### playdate.graphics.sprite:setImageDrawMode

```lua
playdate.graphics.sprite:setImageDrawMode(mode: integer): nil
```

Sets the mode for drawing the bitmap. See playdate.graphics.setImageDrawMode(mode) for valid modes.

### playdate.graphics.sprite:setImageFlip

```lua
playdate.graphics.sprite:setImageFlip(flip: (integer|string), flipCollideRect: boolean): nil
```

Flips the bitmap. See playdate.graphics.image:draw() for valid `flip` values.
If `true` is passed for the optional *flipCollideRect* argument, the sprite’s collideRect will be flipped as well.
Calling setImage() will reset the sprite to its default, non-flipped orientation.  So, if you call both setImage() and setImageFlip(), call setImage() first.

### playdate.graphics.sprite:setOpaque

```lua
playdate.graphics.sprite:setOpaque(flag: boolean): nil
```

Marking a sprite opaque tells the sprite system that it doesn’t need to draw anything underneath the sprite, since it will be overdrawn anyway. If you set an image without a mask/alpha channel on the sprite, it automatically sets the opaque flag.
Setting a sprite to opaque can have performance benefits.

### playdate.graphics.sprite:setRedrawsOnImageChange

```lua
playdate.graphics.sprite:setRedrawsOnImageChange(flag: boolean): nil
```

By default, sprites are automatically marked for redraw when their image is changed via playdate.graphics.sprite:setImage(). If disabled by calling this function with a *false* argument, playdate.graphics.sprite.addDirtyRect() can be used to mark the (potentially smaller) area of the screen that needs to be redrawn.

### playdate.graphics.sprite:setRotation

```lua
playdate.graphics.sprite:setRotation(angle: number, scale: integer, yScale: integer): nil
```

Sets the rotation for the sprite, in degrees clockwise, with an optional scaling factor. If setImage() is called after this, the rotation and scale is applied to the new image. Only affects sprites that have an image set. This function should be used with discretion, as it’s likely to be slow on the hardware. Consider pre-rendering rotated images for your sprites instead.

### playdate.graphics.sprite:setScale

```lua
playdate.graphics.sprite:setScale(scale: integer, yScale: integer): nil
```

Sets the scaling factor for the sprite, with an optional separate scaling for the y axis. If setImage() is called after this, the scale factor is applied to the new image. Only affects sprites that have an image set.

### playdate.graphics.sprite:setSize

```lua
playdate.graphics.sprite:setSize(width: integer, height: integer): nil
```

Sets the sprite’s size. The method has no effect if the sprite has an image set.

### playdate.graphics.sprite:setStencilImage

```lua
playdate.graphics.sprite:setStencilImage(stencil: _Image, tile: boolean): nil
```

Specifies a stencil image to be set before the sprite is drawn. As with playdate.graphics.setStencilImage(), the sprite pixels will be drawn where the stencil is white and nothing drawn where the stencil is black. Note that the stencil is attached to the frame buffer (i.e., the screen), not the sprite—it does not move along with the sprite. If *tile* is set, the stencil will be tiled; in this case, the image width must be a multiple of 32 pixels.

### playdate.graphics.sprite:setStencilPattern

```lua
playdate.graphics.sprite:setStencilPattern(level: any, ditherType: integer): nil
playdate.graphics.sprite:setStencilPattern(pattern: integer[]): nil
playdate.graphics.sprite:setStencilPattern(row1: integer, row2: integer, row3: integer, row4: integer, row5: integer, row6: integer, row7: integer, row8: integer): nil
```

Sets the sprite’s stencil to a dither pattern specified by *level* and optional *ditherType* (defaults to `playdate.graphics.image.kDitherTypeBayer8x8`).

### playdate.graphics.sprite:setTag

```lua
playdate.graphics.sprite:setTag(tag: integer): nil
```

Sets the sprite’s tag, an integer value in the range of 0 to 255, useful for identifying sprites later, particularly when working with collisions.

### playdate.graphics.sprite:setTilemap

```lua
playdate.graphics.sprite:setTilemap(tilemap: _TileMap): nil
```

Sets the sprite’s contents to the given tilemap. Useful if you want to automate drawing of your tilemap, especially if interleaved by depth with other sprites being drawn.

### playdate.graphics.sprite:setUpdatesEnabled

```lua
playdate.graphics.sprite:setUpdatesEnabled(flag: boolean): nil
```

The sprite’s *updatesEnabled* flag (defaults to true) determines whether a sprite’s update() method will be called. By default, a sprite’s `update` method does nothing; however, you may choose to have your sprite do something on every frame by implementing an update method on your sprite instance, or implementing it in your sprite subclass.

### playdate.graphics.sprite:setVisible

```lua
playdate.graphics.sprite:setVisible(flag: boolean): nil
```

Sprites that aren’t visible don’t get their draw() method called.

### playdate.graphics.sprite:setZIndex

```lua
playdate.graphics.sprite:setZIndex(z: integer): nil
```

Sets the Z-index of the given sprite. Sprites with higher Z-indexes are drawn on top of those with lower Z-indexes. Valid values for *z* are in the range (-32768, 32767).

### playdate.graphics.sprite:update

```lua
playdate.graphics.sprite:update(): nil
```

Called by playdate.graphics.sprite.update() (note the syntactic difference between the period and the colon) before sprites are drawn. Implementing `:update()` gives you the opportunity to perform some code upon every frame.
The update method will only be called on sprites that have had add() called on them, and have their updates enabled.
Be careful not confuse `sprite:update()` with `sprite.update()`: the latter updates all sprites; the former updates just the sprite being invoked.
```
local mySprite = playdate.graphics.sprite.new()
mySprite:moveTo(200, 120)
mySprite:add() -- Sprite needs to be added to get drawn and updated
-- mySprite:remove() will make it so the sprite stops getting drawn/updated
-- Option 1: override the update method using an anonymous function
mySprite.update = function(self)
    print("This gets called every frame when I'm added to the display list")
    -- Manipulate sprite using "self"
    print(self.x) -- Prints 200.0
    print(self.y) -- Prints 120.0
end
-- Option 2: override the update method using a function stored in a variable
local function mySpriteUpdate(self)
    print("This gets called every frame when I'm added to the display list")
    -- Manipulate sprite using "self"
    print(self.x) -- Prints 200.0
    print(self.y) -- Prints 120.0
end
mySprite.update = mySpriteUpdate
-- Option 3: override the update method by directly defining it
function mySprite:update()
    print("This gets called every frame when I'm added to the display list")
    -- Manipulate sprite using "self"
    print(self.x) -- Prints 200.0
    print(self.y) -- Prints 120.0
end
function playdate.update()
    -- Your custom update method gets called here every frame if the sprite has been added
    playdate.graphics.sprite.update()
end
-- VERY simplified psuedocode explanation of what's happening in sprite.update() (not real code)
local displayList = {} -- Added sprites are kept track of in a list
function playdate.graphics.sprite.update()
    -- The display list is iterated over
    for i=1, #displayList do
        local sprite = displayList[i]
        -- Checks if updates on the sprites are enabled
        if sprite:updatesEnabled() then
            -- The sprite update method is called
            sprite:update()
        end
        ...
        -- Redraw all of the dirty rects, handle collisions, etc.
    end
end
```

### playdate.graphics.sprite:updatesEnabled

```lua
playdate.graphics.sprite:updatesEnabled(): boolean
```

Returns a boolean value, true if updates are enabled on the sprite.

## Classes

### _Sprite

```lua
---@class _Sprite : playdate.graphics.sprite
---@field x integer
---@field y integer
---@field width integer
---@field height integer
---@field collisionResponse? (integer|fun(self: _Sprite, other: _Sprite): integer?)
---@field update? fun():nil
```

### _SpriteCollisionData

```lua
---@class _SpriteCollisionData
---@field sprite _Sprite
---@field other _Sprite
---@field type integer
---@field overlaps boolean
---@field ti number
---@field move _Vector2D
---@field normal _Vector2D
---@field touch _Point
---@field spriteRect _Rect
---@field otherRect _Rect
---@field bounce? _Point
---@field slide? _Point
```

### _SpriteCollisionInfo

```lua
---@class _SpriteCollisionInfo
---@field sprite _Sprite
---@field entryPoint _Point
---@field exitPoint _Point
---@field t1 number
---@field t2 number
```

### playdate.graphics.sprite

```lua
---@class playdate.graphics.sprite : _Object
---@field kCollisionTypeSlide integer 0
---@field kCollisionTypeFreeze integer 1
---@field kCollisionTypeOverlap integer 2
---@field kCollisionTypeBounce integer 3
```
