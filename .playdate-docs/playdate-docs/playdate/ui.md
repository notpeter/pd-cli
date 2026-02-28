# playdate.ui

## Functions

### playdate.ui.gridview.new

```lua
playdate.ui.gridview.new(cellWidth: integer, cellHeight: integer): _GridView
```

Returns a new playdate.ui.gridview with cells sized *cellWidth*, *cellHeight*. (Sizes are in pixels.) If cells should span the entire width of the grid (as in a list view), pass zero (0) for *cellWidth*.

### playdate.ui.gridview:addHorizontalDividerAbove

```lua
playdate.ui.gridview:addHorizontalDividerAbove(section: integer, row: integer): nil
```

Causes a horizontal divider to be drawn above the specified row. Drawing can be customized by overriding  playdate.ui.gridview:drawHorizontalDivider.

### playdate.ui.gridview:drawCell

```lua
playdate.ui.gridview:drawCell(section: integer, row: integer, column: integer, selected: boolean, x: integer, y: integer, width: integer, height: integer): nil
```

Override this method to draw the cells in the gridview. *selected* is a boolean, true if the cell being drawn is the currently-selected cell.

### playdate.ui.gridview:drawHorizontalDivider

```lua
playdate.ui.gridview:drawHorizontalDivider(x: integer, y: integer, width: integer, height: integer): nil
```

Override this method to customize the drawing of horizontal dividers. This function will only be called if the horizontal divider height is greater than zero (0) and at least one divider has been added.

### playdate.ui.gridview:drawInRect

```lua
playdate.ui.gridview:drawInRect(x: integer, y: integer, width: integer, height: integer): nil
```

Draws the gridview in the specified rect. Ideally this should be called on every playdate.update() to accommodate scrolling.

### playdate.ui.gridview:drawSectionHeader

```lua
playdate.ui.gridview:drawSectionHeader(section: integer, x: integer, y: integer, width: integer, height: integer): nil
```

Override this method to draw section headers. This function will only be called if the header height has been set to a value greater than zero (0).

### playdate.ui.gridview:getCellBounds

```lua
playdate.ui.gridview:getCellBounds(section: integer, row: integer, column: integer, gridWidth: integer): (integer, integer, integer, integer)
```

Returns multiple values (x, y, width, height) representing the bounds of the cell, not including padding, relative to the top-right corner of the grid view.
If the grid view is configured with zero width cells (see playdate.ui.gridview:new), *gridWidth* is required, and should be the same value you would pass to playdate.ui.gridview:drawInRect.

### playdate.ui.gridview:getHorizontalDividerHeight

```lua
playdate.ui.gridview:getHorizontalDividerHeight(): integer
```

Returns the height of the horizontal dividers.

### playdate.ui.gridview:getNumberOfColumns

```lua
playdate.ui.gridview:getNumberOfColumns(): integer
```

Returns the number of columns in the gridview. 1 by default.

### playdate.ui.gridview:getNumberOfRowsInSection

```lua
playdate.ui.gridview:getNumberOfRowsInSection(section: integer): integer
```

Returns the number of rows in *section*.

### playdate.ui.gridview:getNumberOfSections

```lua
playdate.ui.gridview:getNumberOfSections(): integer
```

Returns the number of sections in the grid view.

### playdate.ui.gridview:getScrollPosition

```lua
playdate.ui.gridview:getScrollPosition(): (integer, integer)
```

Returns the current scroll location as a pair *x*, *y*.

### playdate.ui.gridview:getSectionHeaderHeight

```lua
playdate.ui.gridview:getSectionHeaderHeight(): integer
```

Returns the current height of the section headers.

### playdate.ui.gridview:getSelectedRow

```lua
playdate.ui.gridview:getSelectedRow(): integer
```

Convenience method for list-style gridviews. Returns the selected cell at *row* in section 1.

### playdate.ui.gridview:getSelection

```lua
playdate.ui.gridview:getSelection(): (integer, integer, integer)
```

Returns the currently-selected cell as *section*, *row*, *column*

### playdate.ui.gridview:removeHorizontalDividers

```lua
playdate.ui.gridview:removeHorizontalDividers(): nil
```

Removes all horizontal dividers from the grid view.

### playdate.ui.gridview:scrollCellToCenter

```lua
playdate.ui.gridview:scrollCellToCenter(section: integer, row: integer, column: integer, animated: boolean): nil
```

Scrolls to the specified cell, so the cell is centered in the gridview, if possible.

### playdate.ui.gridview:scrollToCell

```lua
playdate.ui.gridview:scrollToCell(section: integer, row: integer, column: integer, animated: boolean): nil
```

Scrolls to the specified cell, just enough so the cell is visible.

### playdate.ui.gridview:scrollToRow

```lua
playdate.ui.gridview:scrollToRow(row: integer, animated: boolean): nil
```

Convenience function for list-style gridviews. Scrolls to the specified row in the list.

### playdate.ui.gridview:scrollToTop

```lua
playdate.ui.gridview:scrollToTop(animated: boolean): nil
```

Scrolls to the top of the gridview.

### playdate.ui.gridview:selectNextColumn

```lua
playdate.ui.gridview:selectNextColumn(wrapSelection: boolean, scrollToSelection: boolean, animate: boolean): nil
```

Selects the cell directly to the right of the currently-selected cell.
If the last column is currently selected and *wrapSelection* is true, the selection will wrap around to the opposite side of the grid. If a wrap occurs and the gridview’s `changeRowOnColumnWrap` is `true` the row will also be advanced or moved back.
If *scrollToSelection* is true (or not provided), the newly-selected cell will be scrolled to. If *animate* is true (or not provided), the scroll will be animated.

### playdate.ui.gridview:selectNextRow

```lua
playdate.ui.gridview:selectNextRow(wrapSelection: boolean, scrollToSelection: boolean, animate: boolean): nil
```

Selects the cell directly below the currently-selected cell.
If *wrapSelection* is true, the selection will wrap around to the opposite end of the grid. If *scrollToSelection* is true (or not provided), the newly-selected cell will be scrolled to. If *animate* is true (or not provided), the scroll will be animated.

### playdate.ui.gridview:selectPreviousColumn

```lua
playdate.ui.gridview:selectPreviousColumn(wrapSelection: boolean, scrollToSelection: boolean, animate: boolean): nil
```

Identical to `selectNextColumn()` but goes the other direction.

### playdate.ui.gridview:selectPreviousRow

```lua
playdate.ui.gridview:selectPreviousRow(wrapSelection: boolean, scrollToSelection: boolean, animate: boolean): nil
```

Identical to `selectNextRow()` but goes the other direction.

### playdate.ui.gridview:setCellPadding

```lua
playdate.ui.gridview:setCellPadding(left: integer, right: integer, top: integer, bottom: integer): nil
```

Sets the amount of padding around cells.

### playdate.ui.gridview:setCellSize

```lua
playdate.ui.gridview:setCellSize(cellWidth: integer, cellHeight: integer): nil
```

Sets the size of the cells in the gridview. If cells should span the entire width of the grid (as in a list view), pass zero (0) for *cellWidth*.

### playdate.ui.gridview:setContentInset

```lua
playdate.ui.gridview:setContentInset(left: integer, right: integer, top: integer, bottom: integer): nil
```

Sets the amount of space the content is inset from the edges of the gridview. Useful if a background image is being used as a border.

### playdate.ui.gridview:setHorizontalDividerHeight

```lua
playdate.ui.gridview:setHorizontalDividerHeight(height: integer): nil
```

Sets the height of the horizontal dividers. The default height is half the cell height specified when creating the grid view.

### playdate.ui.gridview:setNumberOfColumns

```lua
playdate.ui.gridview:setNumberOfColumns(num: integer): nil
```

Sets the number of columns in the gridview. 1 by default.

### playdate.ui.gridview:setNumberOfRows

```lua
playdate.ui.gridview:setNumberOfRows(...: integer): nil
```

Convenience method for list-style gridviews, or for setting the number of rows for multiple sections at a time. Pass in a list of numbers of rows for sections starting from section 1.

### playdate.ui.gridview:setNumberOfRowsInSection

```lua
playdate.ui.gridview:setNumberOfRowsInSection(section: integer, num: integer): nil
```

Sets the number of rows in *section*.

### playdate.ui.gridview:setNumberOfSections

```lua
playdate.ui.gridview:setNumberOfSections(num: integer): nil
```

Sets the number of sections in the grid view. Each section contains at least one row, and row numbering starts at 1 in each section.

### playdate.ui.gridview:setScrollDuration

```lua
playdate.ui.gridview:setScrollDuration(ms: integer): nil
```

Controls the duration of scroll animations. 250ms by default.

### playdate.ui.gridview:setScrollPosition

```lua
playdate.ui.gridview:setScrollPosition(x: integer, y: integer, animated: boolean): nil
```

'set' scrolls to the coordinate *x*, *y*.
If *animated* is true (or not provided) the new scroll position is animated to using playdate.ui.gridview.scrollEasingFunction and the value set in playdate.ui.gridview:setScrollDuration().

### playdate.ui.gridview:setSectionHeaderHeight

```lua
playdate.ui.gridview:setSectionHeaderHeight(height: integer): nil
```

Sets the height of the section headers. 0 by default, which causes section headers not to be drawn.

### playdate.ui.gridview:setSectionHeaderPadding

```lua
playdate.ui.gridview:setSectionHeaderPadding(left: integer, right: integer, top: integer, bottom: integer): nil
```

Sets the amount of padding around section headers.

### playdate.ui.gridview:setSelectedRow

```lua
playdate.ui.gridview:setSelectedRow(row: integer): nil
```

Convenience method for list-style gridviews. Selects the cell at *row* in section 1.

### playdate.ui.gridview:setSelection

```lua
playdate.ui.gridview:setSelection(section: integer, row: integer, column: integer): nil
```

Selects the cell at the given position.

## Classes

### _GridView

```lua
---@class _GridView : playdate.ui.gridview
---@field needsDisplay boolean
---@field backgroundImage (_Image|_NineSlice)
---@field isScrolling boolean
---@field scrollEasingFunction fun(t:number, b:number, c:number, d:number, a?:number, p?:number): number
---@field easingAmplitude? number
---@field easingPeriod? number
---@field changeRowOnColumnWrap boolean
---@field scrollCellsToCenter boolean
```

### playdate.ui.crankIndicator

```lua
---@class playdate.ui.crankIndicator
---@field clockwise boolean
```
