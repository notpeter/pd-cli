# playdate.mirror

## Functions

### playdate.mirrorEnded

```lua
playdate.mirrorEnded(): nil
```

Called when the device is disconnected from Mirror.

### playdate.mirrorStarted

```lua
playdate.mirrorStarted(): nil
```

Called when the device is connected to Mirror.
In rare situations, Mirror may have trouble keeping up with games running at a high framerate (> 40 fps). If you find this consistently happens to your game, you can optionally use these callbacks to lower the amount of computation or drawing you do so as to give more time to Playdate OS on each frame, improving your user’s experience while playing your game via Mirror.
