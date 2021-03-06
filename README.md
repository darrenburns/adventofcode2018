# Advent of Code 2018

These are my solutions to the Advent of Code challenge 2018.

I'm trying to solve as many of them as possible using Rust, but might fall back to 
Python for some questions if required.

### Benchmarks

#### Day 12: Subterranean Sustainability

##### Part 1 + Part 2 (Benchmarked together)
```text
time                 22.27 ms   (22.16 ms .. 22.42 ms)
                     1.000 R²   (1.000 R² .. 1.000 R²)
mean                 22.59 ms   (22.47 ms .. 22.97 ms)
std dev              422.2 μs   (157.7 μs .. 801.9 μs)
```

#### Day 11: Chronal Charge

Part 1 was fast, part 2 was not. It took over a minute to execute part 2, 
but the approach was quite brute-force. The question seems quite suited towards a dynamic 
programming approach, which would probably improve performance considerably.


#### Day 10: The Stars Align

##### Part 1 + Part 2 (Solved and benchmarked together)
```text
time                 28.88 ms   (28.50 ms .. 29.20 ms)
                     0.999 R²   (0.999 R² .. 1.000 R²)
mean                 29.35 ms   (29.07 ms .. 29.75 ms)
std dev              699.6 μs   (464.1 μs .. 1.108 ms)
```

#### Day 9: Marble Mania

##### Part 1
```text
time                 8.197 ms   (8.136 ms .. 8.280 ms)
                     1.000 R²   (0.999 R² .. 1.000 R²)
mean                 8.286 ms   (8.250 ms .. 8.352 ms)
std dev              136.8 μs   (80.57 μs .. 237.8 μs)
```

##### Part 2
```text
time                 115.6 ms   (107.2 ms .. 129.6 ms)
                     0.986 R²   (0.968 R² .. 0.999 R²)
mean                 108.9 ms   (104.9 ms .. 113.8 ms)
std dev              6.914 ms   (4.228 ms .. 11.06 ms)
```

#### Day 8: Memory Maneuver

##### Part 1
```text
time                 8.083 ms   (8.019 ms .. 8.179 ms)
                     0.999 R²   (0.997 R² .. 1.000 R²)
mean                 8.376 ms   (8.281 ms .. 8.665 ms)
std dev              429.3 μs   (173.0 μs .. 819.5 μs)
```

##### Part 2
```text
time                 8.027 ms   (7.954 ms .. 8.085 ms)
                     0.999 R²   (0.999 R² .. 1.000 R²)
mean                 8.256 ms   (8.145 ms .. 8.544 ms)
std dev              489.9 μs   (150.2 μs .. 970.2 μs)
```

#### Day 7: The Sum Of Its Parts

##### Part 1
```text
time                 5.854 ms   (5.729 ms .. 6.046 ms)
                     0.994 R²   (0.990 R² .. 0.997 R²)
mean                 6.345 ms   (6.253 ms .. 6.455 ms)
std dev              307.1 μs   (242.6 μs .. 443.7 μs)
```

#### Day 6: Chronal Coordinates

##### Part 1
```text
time                 230.6 ms   (220.8 ms .. 239.5 ms)
                     0.999 R²   (0.995 R² .. 1.000 R²)
mean                 233.0 ms   (229.4 ms .. 237.4 ms)
std dev              5.087 ms   (3.238 ms .. 7.149 ms)
```

##### Part 2
```text
time                 230.9 ms   (226.0 ms .. 237.0 ms)
                     0.999 R²   (0.998 R² .. 1.000 R²)
mean                 230.9 ms   (228.9 ms .. 233.1 ms)
std dev              2.960 ms   (2.495 ms .. 3.530 ms)
```

#### Day 5: Alchemical Reduction

##### Part 1 + Part 2 benchmarked together

```text
time                 48.84 ms   (48.05 ms .. 49.74 ms)
                     0.999 R²   (0.998 R² .. 1.000 R²)
mean                 50.07 ms   (49.59 ms .. 50.64 ms)
std dev              1.076 ms   (825.6 μs .. 1.339 ms)
```

#### Day 4: Repose Record

##### Part 1 + Part 2 benchmarked together
```text
time                 6.726 ms   (6.661 ms .. 6.790 ms)
                     0.999 R²   (0.999 R² .. 1.000 R²)
mean                 6.921 ms   (6.883 ms .. 6.965 ms)
std dev              117.9 μs   (99.90 μs .. 139.6 μs)
```

#### Day 3: No Matter How You Slice It

##### Part 1 + Part 2 benchmarked together
```text
time                 16.48 ms   (16.30 ms .. 16.64 ms)
                     1.000 R²   (0.999 R² .. 1.000 R²)
mean                 17.11 ms   (16.84 ms .. 17.92 ms)
std dev              1.023 ms   (304.8 μs .. 1.977 ms)
``` 

#### Day 2: Inventory Management System

##### Part 1
```text
time                 7.756 ms   (7.362 ms .. 8.015 ms)
                     0.988 R²   (0.982 R² .. 0.994 R²)
mean                 7.062 ms   (6.919 ms .. 7.240 ms)
std dev              460.8 μs   (364.9 μs .. 515.7 μs)
```

##### Part 2
```text
time                 7.954 ms   (7.889 ms .. 8.017 ms)
                     1.000 R²   (0.999 R² .. 1.000 R²)
mean                 8.104 ms   (8.053 ms .. 8.209 ms)
std dev              207.7 μs   (91.73 μs .. 379.0 μs)
```

#### Day 1: Chronal Calibration

##### Part 1
```text
time                 7.264 ms   (7.190 ms .. 7.328 ms)
                     0.999 R²   (0.999 R² .. 1.000 R²)
mean                 7.474 ms   (7.361 ms .. 7.913 ms)
std dev              539.0 μs   (135.6 μs .. 1.172 ms)
```
##### Part 2
```text
time                 45.34 ms   (43.83 ms .. 46.35 ms)
                     0.997 R²   (0.994 R² .. 0.999 R²)
mean                 44.76 ms   (43.40 ms .. 45.69 ms)
std dev              2.138 ms   (1.298 ms .. 3.026 ms)
```
