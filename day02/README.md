# Rock, Paper, Scissors

## Part 1

Each shape in "Rock, Paper, Scissors" is encoded with the following values:

```
rock     = 0
paper    = 1
scissors = 2
```

For any shape X:

```
X draws with:   (X + 0) % 3
X loses to:     (X + 1) % 3
X wins against: (X + 2) % 3
```

For any two shapes X and Y, the difference X - Y means

```
 0 => X draws with Y
-1 => X loses to Y
-2 => X wins against Y
```

Performing X - Y mod 3 gives us

```
0 => X draws with Y                         0 => X draws with Y
2 => X loses to Y         -- ordered -->    1 => X wins against Y
1 => X wins against Y                       2 => X loses to Y
```

Performing 1 + X - Y mod 3 rotates the results, giving us

```
1 => X draws with Y                         0 => X loses  Y
0 => X loses to Y         -- ordered -->    1 => X draws with Y
2 => X wins against Y                       2 => X wins against Y
```

This makes scoring a round easy since a loss gives 0 points, a draw gives 3
points, and a win gives 6 points. The only extra processing needed is to
compute a shape's score by adding 1 to its assigned value.

So, assuming the you are X and the opponent is Y:

```
round_score = (1 + X) + (3 * ((4 + X - Y) % 3))
```

Note that 4 is used instead of 1 because the % operator isn't true modulo.

## Part 2

We keep the same shape encodings as before and provide an encoding
for each result in a round.

```
rock     = 0               draw = 0
paper    = 1      and      win  = 1
scissors = 2               loss = 2
```

The result encoding is based on:

```
0 => X draws with Y                         0 => X draws with Y
2 => X loses to Y         -- ordered -->    1 => X wins against Y
1 => X wins against Y                       2 => X loses to Y
```

Because we computed the result of a round in the following manner:

```
Z = X - Y mod 3
```

It makes sense that

```
X = Y + Z mod 3
```

As a short example, assume the opponent chose rock and the result is to be a
loss. This means that `Y = 0` and `Z = 2`.

```
X = Y + Z mod 3
  = 0 + 2 mod 3
  = 2           # scissors
```

Unfortunately, if we're lazy (which we are) and reused the processing used for
part 1, the actual encoding for each result in a round ends up being:

```
actual_loss = 0     (aligns with rock)
actual_draw = 1     (aligns with paper)
actual_win  = 2     (aligns with scissors)
```

Fortunately, it is very easy to convert from the actual encoding to the encoding
used earlier by simply adding 2 and taking the result modulo 3. Thus, given that
our opponent is playing `Z` and we want result `Y`, the shape `X` we want is:

```
X = Y + Z + 2 mod 3
```
