# playdate.graphics.perlin

## Functions

### playdate.graphics.perlin

```lua
playdate.graphics.perlin(x: integer, y: integer, z: integer, _repeat: number, octaves: integer, persistence: number): number
```

Returns the Perlin value (from 0.0 to 1.0) at position *(x, y, z)*.
If *repeat* is greater than 0, the pattern of noise will repeat at that point on all 3 axes.
*octaves* is the number of octaves of noise to apply.  Compute time increases linearly with each additional octave, but the results are a bit more organic, consisting of a combination of larger and smaller variations.
When using more than one octave, *persistence* is a value from 0.0 - 1.0 describing the amount the amplitude is scaled each octave.  The lower the value of *persistence*, the less influence each successive octave has on the final value.

### playdate.graphics.perlinArray

```lua
playdate.graphics.perlinArray(count: integer, x: integer, dx: integer, y: integer, dy: integer, z: integer, dz: integer, _repeat: number, octaves: integer, persistence: number): number[]
```

Returns an array of Perlin values at once, avoiding the performance penalty of calling *perlin()* multiple times in a loop.
The parameters are the same as *perlin()* except:
*count* is the number of values to be returned.
*dx*, *dy*, and *dz* are how far to step along the x, y, and z axes in each iteration.
