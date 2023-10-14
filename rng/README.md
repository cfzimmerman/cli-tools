**RNG**
This is a tool for quickly generating sequences of random numbers. Especially useful for example data when sketching ideas or writing tests.

Here's some example usage (`rng` is my zsh alias for the binary):

Usage:

```zsh
❯ rng

Please provide one or more arguments:
rng (count):
rng (count) (max):
rng (count) (min) (max):
default min = 0, max = 10

ex: `rng 3 0 2` might yield `[0, 2, 1]`
```

Some count of elements from 0 through 10:

```zsh
❯ rng 4

[2, 0, 2, 9]
```

Some count of elements from 0 through max:

```zsh
❯ rng 4 100

[0, 40, 38, 8]
```

Some count of elements from min through max:

```zsh
❯ rng 4 -1000 1000

[519, -251, 674, -360]
```
