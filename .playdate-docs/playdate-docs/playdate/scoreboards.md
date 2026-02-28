# playdate.scoreboards

## Functions

### playdate.scoreboards.addScore

```lua
playdate.scoreboards.addScore(boardName: string, value: integer, callback: fun(status: _ServerStatus, result: _ScoreBoardAddResult): nil): nil
```

### playdate.scoreboards.getPersonalBest

```lua
playdate.scoreboards.getPersonalBest(boardName: string, callback: fun(status?: _ServerStatus, result?: _ScoreBoardAddResult): nil): nil
```

### playdate.scoreboards.getScoreboards

```lua
playdate.scoreboards.getScoreboards(callback: fun(status: _ServerStatus, result: _ScoreBoardsScoreboardsResult): nil): nil
```

### playdate.scoreboards.getScores

```lua
playdate.scoreboards.getScores(boardID: string, callback: fun(status: _ServerStatus, result: _ScoreBoardScoresResult): nil): nil
```

## Classes

### _ScoreBoardAddResult

```lua
---@class _ScoreBoardAddResult
---@field rank? integer
---@field player string
---@field value integer
```

### _ScoreBoardBoards

```lua
---@class _ScoreBoardBoards
---@field boardId string
---@field name string
```

### _ScoreBoardScores

```lua
---@class _ScoreBoardScores
---@field rank integer
---@field player string
---@field value integer
```

### _ScoreBoardScoresResult

```lua
---@class _ScoreBoardScoresResult
---@field lastUpdated integer
---@field scores _ScoreBoardScores[]
```

### _ScoreBoardsScoreboardsResult

```lua
---@class _ScoreBoardsScoreboardsResult
---@field lastUpdated integer
---@field boards _ScoreBoardBoards[]
```

### _ServerStatus

```lua
---@class _ServerStatus
---@field code string
---@field message string
```
