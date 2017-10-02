# Example usage

Using the line-by-line default engine:
```
$ time ./target/release/fractal --resolution 1920x1080 --type SeaHorseValley --output-filename SeaHorseValley-default-serial.png
```

Using the line-by-line default engine, but rendering lines in parallel:
```
$ time ./target/release/fractal --resolution 1920x1080 --type SeaHorseValley --parallel --output-filename SeaHorseValley-default-parallel.png
```

Using the recursive engine:
```
$ time ./target/release/fractal --resolution 1920x1080 --type SeaHorseValley --engine Recursive --output-filename SeaHorseValley-recursive-serial.png
```

Using the recursive engine, but rendering bands in parallel:
```
$ time ./target/release/fractal --resolution 1920x1080 --type SeaHorseValley --engine Recursive --parallel --output-filename SeaHorseValley-recursive-parallel.png
```
