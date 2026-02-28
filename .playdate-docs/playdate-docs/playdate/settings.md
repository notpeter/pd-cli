# playdate.settings

## Functions

### playdate.getFlipped

```lua
playdate.getFlipped(): boolean
```

Returns *true* if the user has checked the "Upside Down" option in Playdate Settings; *false* otherwise. (Upside Down mode can be convenient for players wanting to hold Playdate upside-down so they can use their left hand to operate the crank.)
Typically your game doesn’t need to anything in regards to this setting. But it is available in case your game wants to take some special actions, display special instructions, etc.
Reported d-pad directions are flipped when in Upside Down mode&nbsp;—&nbsp;RIGHT will be reported as LEFT, UP as DOWN, etc.&nbsp;—&nbsp;so that the d-pad will make sense to a user holding Playdate upside-down. However, the A and B buttons —&nbsp;since they are still labeled as "A" and "B"&nbsp;—&nbsp;retain their normal meanings and will be reported as usual.

### playdate.getReduceFlashing

```lua
playdate.getReduceFlashing(): boolean
```

Returns *true* if the user has checked the "Reduce Flashing" option in Playdate Settings; *false* otherwise. Games should read this value and, if *true*, avoid visuals that could be problematic for people with sensitivities to flashing lights or patterns.

### playdate.getSystemLanguage

```lua
playdate.getSystemLanguage(): integer
```

Returns the current language of the system, which will be one of the constants *playdate.graphics.font.kLanguageEnglish* or *playdate.graphics.font.kLanguageJapanese*.
