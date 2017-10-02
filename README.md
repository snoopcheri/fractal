# Example usage

Using the line-by-line default engine:
```
$ ./fractal -r 1920x1080 -t SeaHorseValley -o SeaHorseValley-default-serial.png
```

Using the line-by-line default engine, but rendering lines in parallel:
```
$ ./fractal -r 1920x1080 -t SeaHorseValley -p -o SeaHorseValley-default-parallel.png
```

Using the recursive engine:
```
$ ./fractal -r 1920x1080 -t SeaHorseValley -e Recursive -o SeaHorseValley-recursive-serial.png
```

Using the recursive engine, but rendering bands in parallel:
```
$ ./fractal -r 1920x1080 -t SeaHorseValley -e Recursive -p -o SeaHorseValley-recursive-parallel.png
```
